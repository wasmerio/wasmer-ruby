use rutie::{class, methods, rubysys::value::ValueType, AnyObject, Boolean, Object, RString};
use wasmer_runtime::validate;

class!(Module);

methods!(
    Module,
    _itself,
    fn ruby_module_validate(bytes: AnyObject) -> Boolean {
        if let Ok(bytes) = bytes {
            match bytes.ty() {
                ValueType::RString => bytes.try_convert_to::<RString>().ok().map_or_else(
                    || Boolean::new(false),
                    |string| {
                        if validate(string.to_bytes_unchecked()) {
                            Boolean::new(true)
                        } else {
                            Boolean::new(false)
                        }
                    },
                ),
                _ => Boolean::new(false),
            }
        } else {
            Boolean::new(false)
        }
    }
);
