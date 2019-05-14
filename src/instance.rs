//! The `Instance` WebAssembly class.

use crate::error::unwrap_or_raise;
use crate::memory::{Memory, RubyMemory, MEMORY_WRAPPER};
use lazy_static::lazy_static;
use rutie::{
    class, methods,
    rubysys::{class, value::ValueType},
    types::{Argc, Value},
    util::str_to_cstring,
    wrappable_struct, AnyException, AnyObject, Array, Boolean, Exception, Fixnum, Float, Module,
    NilClass, Object, RString, Symbol,
};
use std::{mem, rc::Rc};
use wasmer_runtime::{self as runtime, imports, Export};
use wasmer_runtime_core::types::Type;

/// The `ExportedFunctions` Ruby class.
pub struct ExportedFunctions {
    /// The WebAssembly runtime.
    instance: Rc<runtime::Instance>,
}

impl ExportedFunctions {
    /// Create a new instance of the `ExportedFunctions` Ruby class.
    pub fn new(instance: Rc<runtime::Instance>) -> Self {
        Self { instance }
    }

    pub fn respond_to_missing(&self, method_name: &str) -> bool {
        self.instance.dyn_func(method_name).is_ok()
    }

    /// Call an exported function on the given WebAssembly instance.
    pub fn method_missing(
        &self,
        method_name: &str,
        arguments: Array,
    ) -> Result<AnyObject, AnyException> {
        let function = self.instance.dyn_func(method_name).map_err(|_| {
            AnyException::new(
                "RuntimeError",
                Some(&format!("Function `{}` does not exist.", method_name)),
            )
        })?;
        let signature = function.signature();
        let parameters = signature.params();
        let number_of_parameters = parameters.len() as isize;
        let number_of_arguments = arguments.length() as isize;
        let diff: isize = number_of_parameters - number_of_arguments;

        if diff > 0 {
            return Err(AnyException::new(
                "ArgumentError",
                Some(&format!(
                    "Missing {} argument(s) when calling `{}`: Expect {} argument(s), given {}.",
                    diff, method_name, number_of_parameters, number_of_arguments
                )),
            ));
        } else if diff < 0 {
            return Err(AnyException::new(
                "ArgumentError",
                Some(&format!(
                    "Given {} extra argument(s) when calling `{}`: Expect {} argument(s), given {}.",
                    diff.abs(), method_name, number_of_parameters, number_of_arguments
                )),
            ));
        }

        let mut function_arguments =
            Vec::<runtime::Value>::with_capacity(number_of_parameters as usize);

        for (nth, (parameter, argument)) in parameters.iter().zip(arguments.into_iter()).enumerate()
        {
            let value = match (parameter, argument.ty()) {
                (Type::I32, ValueType::Fixnum) => runtime::Value::I32(
                    argument
                        .try_convert_to::<Fixnum>()
                        .map_err(|_| {
                            AnyException::new(
                                "TypeError",
                                Some(&format!(
                                    "Cannot convert argument #{} to a WebAssembly i32 value.",
                                    nth + 1
                                )),
                            )
                        })?
                        .to_i32(),
                ),
                (Type::I64, ValueType::Fixnum) => runtime::Value::I64(
                    argument
                        .try_convert_to::<Fixnum>()
                        .map_err(|_| {
                            AnyException::new(
                                "TypeError",
                                Some(&format!(
                                    "Cannot convert argument #{} to a WebAssembly i64 value.",
                                    nth + 1
                                )),
                            )
                        })?
                        .to_i64(),
                ),
                (Type::F32, ValueType::Float) => runtime::Value::F32(
                    argument
                        .try_convert_to::<Float>()
                        .map_err(|_| {
                            AnyException::new(
                                "TypeError",
                                Some(&format!(
                                    "Cannot convert argument #{} to a WebAssembly f32 value.",
                                    nth + 1
                                )),
                            )
                        })?
                        .to_f64() as f32,
                ),
                (Type::F64, ValueType::Float) => runtime::Value::F64(
                    argument
                        .try_convert_to::<Float>()
                        .map_err(|_| {
                            AnyException::new(
                                "TypeError",
                                Some(&format!(
                                    "Cannot convert argument #{} to a WebAssembly f64 value.",
                                    nth + 1
                                )),
                            )
                        })?
                        .to_f64(),
                ),
                (_, ty) => {
                    return Err(AnyException::new(
                        "ArgumentError",
                        Some(&format!(
                            "Cannot convert argument #{} to a WebAssembly value. Only integers and floats are supported. Given `{:?}`.",
                            nth + 1,
                            ty
                        ))));
                }
            };

            function_arguments.push(value);
        }

        let results = function
            .call(function_arguments.as_slice())
            .map_err(|e| AnyException::new("RuntimeError", Some(&format!("{}", e))))?;

        if results.len() > 0 {
            Ok(match results[0] {
                runtime::Value::I32(result) => Fixnum::new(result as i64).into(),
                runtime::Value::I64(result) => Fixnum::new(result).into(),
                runtime::Value::F32(result) => Float::new(result as f64).into(),
                runtime::Value::F64(result) => Float::new(result).into(),
            })
        } else {
            Ok(NilClass::new().into())
        }
    }
}

wrappable_struct!(
    ExportedFunctions,
    ExportedFunctionsWrapper,
    EXPORTED_FUNCTIONS_WRAPPER
);

class!(RubyExportedFunctions);

#[rustfmt::skip]
methods!(
    RubyExportedFunctions,
    itself,

    fn ruby_exported_functions_method_exists(symbol: Symbol, _include_private: Boolean) -> Boolean {
        unwrap_or_raise(|| {
            let symbol = symbol?;
            let instance = itself.get_data(&*EXPORTED_FUNCTIONS_WRAPPER);
            Ok(Boolean::new(instance.respond_to_missing(symbol.to_str())))
        })
    }
);

/// Glue code to call the `ExportedFunctions.method_missing` method.
pub extern "C" fn ruby_exported_functions_method_missing(
    argc: Argc,
    argv: *const AnyObject,
    itself: RubyExportedFunctions,
) -> AnyObject {
    unwrap_or_raise(|| {
        let arguments = Value::from(0);

        unsafe {
            let argv_pointer: *const Value = mem::transmute(argv);

            class::rb_scan_args(argc, argv_pointer, str_to_cstring("*").as_ptr(), &arguments)
        };

        let mut arguments = Array::from(arguments);
        let method_name = unsafe { arguments.shift().to::<Symbol>() };
        let method_name = method_name.to_str();

        itself
            .get_data(&*EXPORTED_FUNCTIONS_WRAPPER)
            .method_missing(method_name, arguments)
    })
}

/// The `Instance` Ruby class.
pub struct Instance {
    /// The WebAssembly instance.
    instance: Rc<runtime::Instance>,
}

impl Instance {
    /// Create a new instance of the `Instance` Ruby class.
    /// The constructor receives bytes from a string.
    pub fn new(bytes: &[u8]) -> Result<Self, AnyException> {
        let import_object = imports! {};

        Ok(Self {
            instance: Rc::new(runtime::instantiate(bytes, &import_object).map_err(|e| {
                AnyException::new(
                    "RuntimeError",
                    Some(&format!("Failed to instantiate the module:\n    {}", e)),
                )
            })?),
        })
    }
}

wrappable_struct!(Instance, InstanceWrapper, INSTANCE_WRAPPER);

class!(RubyInstance);

#[rustfmt::skip]
methods!(
    RubyInstance,
    _itself,

    // Glue code to call the `Instance.new` method.
    fn ruby_instance_new(bytes: RString) -> AnyObject {
        unwrap_or_raise(|| {
            let instance = Instance::new(
                bytes
                    .map_err(|_| {
                        AnyException::new(
                            "ArgumentError",
                            Some("WebAssembly module must be represented by Ruby bytes only."),
                        )
                    })?
                    .to_bytes_unchecked(),
            )?;
            let exported_functions = ExportedFunctions::new(instance.instance.clone());

            let memory = instance
                .instance
                .exports()
                .find_map(|(_, export)| match export {
                    Export::Memory(memory) => Some(Memory::new(Rc::new(memory))),
                    _ => None,
                })
                .ok_or_else(|| {
                    AnyException::new(
                        "RuntimeError",
                        Some("The WebAssembly module has no exported memory."),
                    )
                })?;

            let wasmer_module = Module::from_existing("Wasmer");

            let mut ruby_instance: AnyObject = wasmer_module
                .get_nested_class("Instance")
                .wrap_data(instance, &*INSTANCE_WRAPPER);

            let ruby_exported_functions: RubyExportedFunctions = wasmer_module
                .get_nested_class("ExportedFunctions")
                .wrap_data(exported_functions, &*EXPORTED_FUNCTIONS_WRAPPER);

            ruby_instance.instance_variable_set("@exports", ruby_exported_functions);

            let ruby_memory: RubyMemory = wasmer_module
                .get_nested_class("Memory")
                .wrap_data(memory, &*MEMORY_WRAPPER);

            ruby_instance.instance_variable_set("@memory", ruby_memory);

            Ok(ruby_instance)
        })
    }

    // Glue code to call the `Instance.exports` getter method.
    fn ruby_instance_exported_functions() -> RubyExportedFunctions {
        unsafe {
            _itself
                .instance_variable_get("@exports")
                .to::<RubyExportedFunctions>()
        }
    }

    // Glue code to call the `Instance.memory` getter method.
    fn ruby_instance_memory() -> RubyMemory {
        unsafe { _itself.instance_variable_get("@memory").to::<RubyMemory>() }
    }
);
