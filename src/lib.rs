#![deny(warnings)]

use rutie::{Class, Object};

pub mod instance;

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
}
