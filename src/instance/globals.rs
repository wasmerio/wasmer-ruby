//! The `ExportedGlobals` WebAssembly class.

use crate::error::unwrap_or_raise;
use lazy_static::lazy_static;
use rutie::{
    class, methods,
    rubysys::class,
    types::{Argc, Value},
    util::str_to_cstring,
    wrappable_struct, AnyException, AnyObject, Array, Boolean, Exception, Fixnum, Float, Module,
    NilClass, Object, Symbol,
};
use std::rc::Rc;
use wasmer_runtime::{
    self as runtime, types::Type as WasmType, Export, Global, Value as WasmValue,
};

/// The `ExportedGlobals` Ruby class.
pub struct ExportedGlobals {
    /// The WebAssembly runtime.
    instance: Rc<runtime::Instance>,
}

impl ExportedGlobals {
    /// Create a new instance of the `ExportedGlobals` Ruby class.
    pub fn new(instance: Rc<runtime::Instance>) -> Self {
        Self { instance }
    }

    /// Check that an exported function exists.
    pub fn respond_to_missing(&self, method_name: &str) -> bool {
        self.instance
            .exports()
            .any(|(export_name, export)| match export {
                Export::Global(_) if export_name == method_name => true,
                _ => false,
            })
    }

    /// Call an exported function on the given WebAssembly instance.
    pub fn method_missing(
        &self,
        method_name: &str,
        _arguments: Array,
    ) -> Result<ExportedGlobal, AnyException> {
        let global = self
            .instance
            .exports()
            .find_map(|(export_name, export)| match export {
                Export::Global(global) if export_name == method_name => Some(global),
                _ => None,
            })
            .ok_or_else(|| {
                AnyException::new(
                    "RuntimeError",
                    Some(&format!("Global `{}` does not exist.", method_name)),
                )
            })?;

        Ok(ExportedGlobal {
            global_name: method_name.to_string(),
            global: Rc::new(global),
        })
    }
}

wrappable_struct!(
    ExportedGlobals,
    ExportedGlobalsWrapper,
    EXPORTED_GLOBALS_WRAPPER
);

class!(RubyExportedGlobals);

#[rustfmt::skip]
methods!(
    RubyExportedGlobals,
    itself,

    // Glue code to call the `Exportedglobals.respond_to` method.
    fn ruby_exported_globals_method_exists(symbol: Symbol, _include_private: Boolean) -> Boolean {
        unwrap_or_raise(|| {
            let symbol = symbol?;
            let exported_globals = itself.get_data(&*EXPORTED_GLOBALS_WRAPPER);

            Ok(Boolean::new(exported_globals.respond_to_missing(symbol.to_str())))
        })
    }
);

/// Glue code to call the `ExportedGlobals.method_missing` method.
pub extern "C" fn ruby_exported_globals_method_missing(
    argc: Argc,
    argv: *const AnyObject,
    itself: RubyExportedGlobals,
) -> RubyExportedGlobal {
    unwrap_or_raise(|| {
        let arguments = Value::from(0);

        unsafe {
            let argv_pointer = argv as *const Value;

            class::rb_scan_args(argc, argv_pointer, str_to_cstring("*").as_ptr(), &arguments)
        };

        let mut arguments = Array::from(arguments);
        let method_name = unsafe { arguments.shift().to::<Symbol>() };
        let method_name = method_name.to_str();

        Ok(Module::from_existing("Wasmer")
            .get_nested_class("ExportedGlobal")
            .wrap_data(
                itself
                    .get_data(&*EXPORTED_GLOBALS_WRAPPER)
                    .method_missing(method_name, arguments)?,
                &*EXPORTED_GLOBAL_WRAPPER,
            ))
    })
}

#[allow(unused)]
/// The `ExportedGlobal` Ruby class.
pub struct ExportedGlobal {
    /// The exported global name from the WebAssembly instance.
    global_name: String,

    /// The exported global from the WebAssembly instance.
    global: Rc<Global>,
}

impl ExportedGlobal {
    pub fn get_value(&self) -> WasmValue {
        self.global.get()
    }

    pub fn set_value(&self, value: WasmValue) {
        self.global.set(value);
    }

    pub fn mutable(&self) -> bool {
        self.global.descriptor().mutable
    }

    fn ty(&self) -> WasmType {
        self.global.descriptor().ty
    }
}

wrappable_struct!(
    ExportedGlobal,
    ExportedGlobalWrapper,
    EXPORTED_GLOBAL_WRAPPER
);

class!(RubyExportedGlobal);

#[rustfmt::skip]
methods!(
    RubyExportedGlobal,
    itself,

    // Glue code to call the `ExportedGlobal.get_value` method.
    fn ruby_exported_global_get_value() -> AnyObject {
        unwrap_or_raise(|| {
            let exported_global = itself.get_data(&*EXPORTED_GLOBAL_WRAPPER);

            Ok(match exported_global.get_value() {
                WasmValue::I32(result) => Fixnum::new(result as i64).to_any_object(),
                WasmValue::I64(result) => Fixnum::new(result).to_any_object(),
                WasmValue::F32(result) => Float::new(result as f64).to_any_object(),
                WasmValue::F64(result) => Float::new(result).to_any_object(),
                WasmValue::V128(_result) => {
                    return Err(AnyException::new(
                        "RuntimeError",
                        Some("Type `V128` isn't supported yet."),
                    ))
                }
            })
        })
    }

    // Glue code to call the `ExportedGlobal.set_value` method.
    fn ruby_exported_global_set_value(value: AnyObject) -> NilClass {
        unwrap_or_raise(|| {
            let value = value?;
            let exported_global = itself.get_data(&*EXPORTED_GLOBAL_WRAPPER);

            let wasm_type = exported_global.ty();
            let wasm_value = match wasm_type {
                WasmType::I32 if value.is_fixnum() => {
                    WasmValue::I32(unsafe { value.to::<Fixnum>().to_i32() })
                }
                WasmType::I64 if value.is_fixnum() => {
                    WasmValue::I64(unsafe { value.to::<Fixnum>().to_i64() })
                }
                WasmType::F32 if value.is_flonum() => {
                    WasmValue::F32(unsafe { value.to::<Float>().to_f64() as f32 })
                }
                WasmType::F64 if value.is_flonum() => {
                    WasmValue::F64(unsafe { value.to::<Float>().to_f64() })
                }
                _ => {
                    return Err(AnyException::new(
                        "TypeError",
                        Some(&format!(
                            "Failed to set `{:?}` to the global `{}` (with type `{}`).",
                            value, exported_global.global_name, wasm_type
                        )),
                    ))
                }
            };

            exported_global.set_value(wasm_value);

            Ok(NilClass::new())
        })
    }

    // Glue code to call the `ExportedGlobal.mutable` getter.
    fn ruby_exported_global_mutable() -> Boolean {
        Boolean::new(itself.get_data(&*EXPORTED_GLOBAL_WRAPPER).mutable())
    }
);
