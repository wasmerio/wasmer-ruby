use crate::rubyclass;
use lazy_static::lazy_static;
use rutie::{methods, AnyObject};

#[rubyclass(module = "Wasmer")]
pub struct Store {
    inner: wasmer::Store,
}

impl Store {
    pub(crate) fn inner(&self) -> &wasmer::Store {
        &self.inner
    }
}

methods!(
    RubyStore,
    _ruby_store,
    fn ruby_new() -> AnyObject {
        Store::wrap(Store {
            inner: Default::default(),
        })
    }
);
