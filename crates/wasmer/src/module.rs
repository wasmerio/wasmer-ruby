use crate::{
    error::{to_ruby_err, unwrap_or_raise, RuntimeError, TypeError},
    rubyclass,
    store::RubyStore,
};
use lazy_static::lazy_static;
use rutie::{methods, AnyObject, Boolean, Object, RString};

#[rubyclass(module = "Wasmer")]
pub struct Module {
    inner: wasmer::Module,
}

methods!(
    RubyModule,
    _ruby_module,
    fn ruby_validate(store: RubyStore, bytes: AnyObject) -> Boolean {
        unwrap_or_raise(|| {
            let store = store?;
            let bytes = bytes?;

            Ok(Boolean::new(match bytes.try_convert_to::<RString>() {
                Ok(bytes) => wasmer::Module::validate(
                    store.unwrap().inner(),
                    bytes.to_str_unchecked().as_bytes(),
                )
                .is_ok(),
                _ => false,
            }))
        })
    },
    fn ruby_new(store: RubyStore, bytes: AnyObject) -> AnyObject {
        unwrap_or_raise(|| {
            let store = store?;
            let bytes = bytes?;

            let module = match bytes.try_convert_to::<RString>() {
                Ok(bytes) => wasmer::Module::new(store.unwrap().inner(), bytes.to_str_unchecked()),
                _ => {
                    return Err(to_ruby_err::<TypeError, _>(
                        "`Module` accepts Wasm bytes or a WAT string",
                    ))
                }
            };

            Ok(Module::wrap(Module {
                inner: module.map_err(to_ruby_err::<RuntimeError, _>)?,
            }))
        })
    }
);
