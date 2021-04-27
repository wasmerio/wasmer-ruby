use crate::RubyClass;
use lazy_static::lazy_static;
use rutie::{methods, AnyObject};

#[derive(RubyClass)]
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
