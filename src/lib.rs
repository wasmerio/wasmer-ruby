#![deny(warnings)]

use rutie::{Class, Module, Object};

pub mod error;
pub mod instance;
pub mod memory;
pub mod module;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_wasmer() {
    let mut wasmer_module = Module::from_existing("Wasmer");

    let instance_data_class = Class::from_existing("Object");

    // Declare the `Instance` Ruby class.
    wasmer_module
        .define_nested_class("Instance", Some(&instance_data_class))
        .define(|itself| {
            // Declare the `self.new` method.
            itself.def_self("new", instance::ruby_instance_new);

            // Declare the `exports` getter method.
            itself.def("exports", instance::ruby_instance_exported_functions);

            // Declare the `memory` getter method.
            itself.def("memory", instance::ruby_instance_memory);
        });

    let exported_functions_data_class = Class::from_existing("Object");

    // Declare the `ExportedFunctions` Ruby class.
    wasmer_module
        .define_nested_class("ExportedFunctions", Some(&exported_functions_data_class))
        .define(|itself| {
            // Declare the `respond_to_missing?` method.
            itself.def(
                "respond_to_missing?",
                instance::ruby_exported_functions_method_exists,
            );

            // Declare the `method_missing` method.
            itself.def(
                "method_missing",
                instance::ruby_exported_functions_method_missing,
            );
        });

    let module_data_class = Class::from_existing("Object");

    // Declare the `Module` Ruby class.
    wasmer_module
        .define_nested_class("Module", Some(&module_data_class))
        .define(|itself| {
            // Declare the `self.validate` method.
            itself.def_self("validate", module::ruby_module_validate);
        });

    let memory_data_class = Class::from_existing("Object");

    // Declare the `Memory` Ruby class.
    wasmer_module
        .define_nested_class("Memory", Some(&memory_data_class))
        .define(|itself| {
            // Declare the `view` method.
            itself.def("uint8_view", memory::ruby_memory_uint8array);

            // Declare the `view` method.
            itself.def("int8_view", memory::ruby_memory_int8array);

            // Declare the `view` method.
            itself.def("uint16_view", memory::ruby_memory_uint16array);

            // Declare the `view` method.
            itself.def("int16_view", memory::ruby_memory_int16array);

            // Declare the `view` method.
            itself.def("uint32_view", memory::ruby_memory_uint32array);

            // Declare the `view` method.
            itself.def("int32_view", memory::ruby_memory_int32array);
        });

    macro_rules! memory_view {
        ($class_name:ident in $mod_name:ident) => {
            let uint8array_data_class = Class::from_existing("Object");

            // Declare the `MemoryView` Ruby class.
            wasmer_module
                .define_nested_class(stringify!($class_name), Some(&uint8array_data_class))
                .define(|itself| {
                    // Declare the `bytes_per_element` getter method.
                    itself.def(
                        "bytes_per_element",
                        memory::view::$mod_name::ruby_memory_view_bytes_per_element,
                    );

                    // Declare the `length` method.
                    itself.def("length", memory::view::$mod_name::ruby_memory_view_length);

                    // Declare the `[]=` (set) method.
                    itself.def("[]=", memory::view::$mod_name::ruby_memory_view_set);

                    // Declare the `[]` (get) method.
                    itself.def("[]", memory::view::$mod_name::ruby_memory_view_get);

                    // Declare the `each` method.
                    itself.def("each", memory::view::$mod_name::ruby_memory_view_each);
                })
                .include("Enumerable");
        };
    }

    memory_view!(Uint8Array in uint8array);
    memory_view!(Int8Array in int8array);
    memory_view!(Uint16Array in uint16array);
    memory_view!(Int16Array in int16array);
    memory_view!(Uint32Array in uint32array);
    memory_view!(Int32Array in int32array);
}
