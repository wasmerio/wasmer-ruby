//#![deny(warnings)]

mod error;
mod module;
mod prelude;
mod store;
mod r#type;

use rutie::{Class, Integer, Module, Object};

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
                this.def("custom_sections", module::ruby::custom_sections);
                this.def("serialize", module::ruby::serialize);
            });
    }

    // Declare the `Type` class.
    {
        let data_class = Class::from_existing("Object");

        wasmer_module
            .define_nested_class("Type", Some(&data_class))
            .define(|this| {
                this.const_set("I32", &Integer::new(1));
                this.const_set("I64", &Integer::new(2));
                this.const_set("F32", &Integer::new(3));
                this.const_set("F64", &Integer::new(4));
                this.const_set("V128", &Integer::new(5));
                this.const_set("EXTERN_REF", &Integer::new(6));
                this.const_set("FUNC_REF", &Integer::new(7));
            });
    }
}
