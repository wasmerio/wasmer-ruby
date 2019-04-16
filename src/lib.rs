#[macro_use]
extern crate rutie;

use rutie::{Class, Object, RString, VM};

class!(Instance);

methods!(
    Instance,
    _itself,
    fn pub_echo(input: RString) -> RString {
        let ruby_string = input.map_err(|e| VM::raise_ex(e)).unwrap();

        RString::new_utf8(&ruby_string.to_string())
    }
);

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_wasmer() {
    Class::new("Instance", None).define(|itself| {
        itself.def_self("echo", pub_echo);
    });
}
