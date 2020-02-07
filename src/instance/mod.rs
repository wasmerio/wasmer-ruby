//! The `Instance` WebAssembly class.

pub mod exports;
pub mod globals;

use crate::{
    error::unwrap_or_raise,
    instance::{
        exports::{ExportedFunctions, RubyExportedFunctions, EXPORTED_FUNCTIONS_WRAPPER},
        globals::{ExportedGlobals, RubyExportedGlobals, EXPORTED_GLOBALS_WRAPPER},
    },
    memory::{Memory, RubyMemory, MEMORY_WRAPPER},
};
use lazy_static::lazy_static;
use rutie::{
    class, methods, wrappable_struct, AnyException, AnyObject, Exception, Module, Object, RString,
};
use std::rc::Rc;
use wasmer_runtime::{self as runtime, imports, Export};

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
            let exported_globals = ExportedGlobals::new(instance.instance.clone());
            let exported_memory =
                instance
                    .instance
                    .exports()
                    .find_map(|(_, export)| match export {
                        Export::Memory(memory) => Some(Memory::new(Rc::new(memory))),
                        _ => None,
                    });

            let wasmer_module = Module::from_existing("Wasmer");

            let mut ruby_instance: AnyObject = wasmer_module
                .get_nested_class("Instance")
                .wrap_data(instance, &*INSTANCE_WRAPPER);

            let ruby_exported_functions: RubyExportedFunctions = wasmer_module
                .get_nested_class("ExportedFunctions")
                .wrap_data(exported_functions, &*EXPORTED_FUNCTIONS_WRAPPER);

            let ruby_exported_globals: RubyExportedGlobals = wasmer_module
                .get_nested_class("ExportedGlobals")
                .wrap_data(exported_globals, &*EXPORTED_GLOBALS_WRAPPER);

            ruby_instance.instance_variable_set("@exports", ruby_exported_functions);
            ruby_instance.instance_variable_set("@globals", ruby_exported_globals);

            if let Some(exported_memory) = exported_memory {
                let ruby_exported_memory: RubyMemory = wasmer_module
                    .get_nested_class("Memory")
                    .wrap_data(exported_memory, &*MEMORY_WRAPPER);
                ruby_instance.instance_variable_set("@memory", ruby_exported_memory);
            }

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

    // Glue code to call the `Instance.globals` getter method.
    fn ruby_instance_exported_globals() -> RubyExportedGlobals {
        unsafe {
            _itself
                .instance_variable_get("@globals")
                .to::<RubyExportedGlobals>()
        }
    }

    // Glue code to call the `Instance.memory` getter method.
    fn ruby_instance_memory() -> RubyMemory {
        unwrap_or_raise(|| {
            let memory = _itself.instance_variable_get("@memory");

            if !memory.is_nil() {
                Ok(unsafe { memory.to::<RubyMemory>() })
            } else {
                Err(AnyException::new(
                    "RuntimeError",
                    Some("The WebAssembly module has no exported memory."),
                ))
            }
        })
    }
);
