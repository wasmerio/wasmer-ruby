use crate::prelude::*;
use lazy_static::lazy_static;
use rutie::AnyObject;

#[rubyclass(module = "Wasmer")]
pub struct Store {
    inner: wasmer::Store,
}

impl Store {
    pub(crate) fn inner(&self) -> &wasmer::Store {
        &self.inner
    }
}

#[rubymethods]
impl Store {
    pub fn new() -> RubyResult<AnyObject> {
        Ok(Store::ruby_new(Store {
            inner: Default::default(),
        }))
    }
}
