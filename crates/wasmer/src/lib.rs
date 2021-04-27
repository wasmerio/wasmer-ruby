//#![deny(warnings)]

mod error;
mod module;
mod store;

pub use ruby_derive::rubyclass;
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
                this.def_self("new", store::ruby_new);
            });
    }

    // Declare the `Module` Ruby class.
    {
        let data_class = Class::from_existing("Object");

        wasmer_module
            .define_nested_class("Module", Some(&data_class))
            .define(|this| {
                this.def_self("validate", module::ruby_validate);
                this.def_self("new", module::ruby_new);
                this.def("name", module::ruby_get_name);
                this.def("name=", module::ruby_set_name);
            });
    }
}
