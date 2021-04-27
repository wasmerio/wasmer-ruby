use crate::{
    error::{to_ruby_err, unwrap_or_raise, RuntimeError, TypeError},
    rubyclass,
    store::{RubyStore, STORE_WRAPPER},
};
use lazy_static::lazy_static;
use rutie::{methods, AnyObject, Object, RString};

#[rubyclass(module = "Wasmer")]
pub struct Module {
    inner: wasmer::Module,
}

methods!(
    RubyModule,
    _ruby_module,
    fn ruby_new(store: RubyStore, bytes: AnyObject) -> AnyObject {
        unwrap_or_raise(|| {
            let store = store?;
            let bytes = bytes?;

            let store = store.get_data(&*STORE_WRAPPER).inner();

            let module = match bytes.try_convert_to::<RString>() {
                Ok(bytes) => wasmer::Module::new(store, bytes.to_str_unchecked()),
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
