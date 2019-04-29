#![deny(warnings)]

use rutie::{Class, Object};

pub mod instance;
pub mod memory;
pub mod module;

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_wasmer() {
    let instance_data_class = Class::from_existing("Object");

    // Declare the `Instance` Ruby class.
    Class::new("Instance", Some(&instance_data_class)).define(|itself| {
        // Declare the `self.new` method.
        itself.def_self("new", instance::ruby_instance_new);

        // Declare the `exports` getter method.
        itself.def("exports", instance::ruby_instance_exported_functions);

        // Declare the `memory` getter method.
        itself.def("memory", instance::ruby_instance_memory);
    });

    let exported_functions_data_class = Class::from_existing("Object");

    // Declare the `ExportedFunctions` Ruby class.
    Class::new("ExportedFunctions", Some(&exported_functions_data_class)).define(|itself| {
        // Declare the `method_missing` method.
        itself.def(
            "method_missing",
            instance::ruby_exported_functions_method_missing,
        );
    });

    let module_data_class = Class::from_existing("Object");

    // Declare the `Module` Ruby class.
    Class::new("Module", Some(&module_data_class)).define(|itself| {
        // Declare the `self.validate` method.
        itself.def_self("validate", module::ruby_module_validate);
    });

    let memory_data_class = Class::from_existing("Object");

    // Declare the `Memory` Ruby class.
    Class::new("Memory", Some(&memory_data_class)).define(|itself| {
        // Declare the `view` method.
        itself.def("view", memory::ruby_memory_view);
    });

    let memory_view_data_class = Class::from_existing("Object");

    // Declare the `MemoryView` Ruby class.
    Class::new("MemoryView", Some(&memory_view_data_class)).define(|itself| {
        // Declare the `length` method.
        itself.def("length", memory::ruby_memory_view_length);
    });
}
