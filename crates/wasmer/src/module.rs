use crate::{
    error::{to_ruby_err, unwrap_or_raise, RuntimeError},
    store::{RubyStore, STORE_WRAPPER},
    RubyClass,
};
use lazy_static::lazy_static;
use rutie::{methods, AnyObject, Object};

#[derive(RubyClass)]
pub struct Module {
    inner: wasmer::Module,
}

methods!(
    RubyModule,
    _itself,
    fn ruby_new(store: RubyStore) -> AnyObject {
        unwrap_or_raise(|| {
            let store = store?;
            let store = store.get_data(&*STORE_WRAPPER).inner();

            dbg!(&store);

            Ok(Module::wrap(Module {
                inner: wasmer::Module::new(store, "(module)")
                    .map_err(|e| to_ruby_err::<RuntimeError, _>(e))?,
            }))
        })
    }
);
