//#![deny(warnings)]

mod error;
mod module;
mod prelude;
mod store;

use rutie::{Class, Module, Object};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_wasmer() {
    let mut wasmer_module = Module::from_existing("Wasmer");

    // Declare the `Store` Ruby class.
    {
        let data_class = Class::from_existing("Object");

        wasmer_module
            .define_nested_class("Store", Some(&data_class))
            .define(|this| {
                this.def_self("new", store::ruby::new);
            });
    }

    // Declare the `Module` Ruby class.
    {
        let data_class = Class::from_existing("Object");

        wasmer_module
            .define_nested_class("Module", Some(&data_class))
            .define(|this| {
                this.def_self("validate", module::ruby::validate);
                this.def_self("new", module::ruby::new);
                this.def("name", module::ruby::get_name);
                this.def("name=", module::ruby::set_name);
            });
    }
}
