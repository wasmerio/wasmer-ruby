use crate::prelude::*;
use rutie::{AnyObject, Boolean, Hash, NilClass, RString};

#[rubyclass(module = "Wasmer")]
pub struct ImportObject {
    inner: wasmer::ImportObject,
}

impl ImportObject {
    pub(crate) fn inner(&self) -> &wasmer::ImportObject {
        &self.inner
    }
}

#[rubymethods]
impl ImportObject {
    pub fn new() -> RubyResult<AnyObject> {
        Ok(ImportObject::ruby_new(ImportObject {
            inner: Default::default(),
        }))
    }

    pub fn contains_namespace(&self, namespace_name: &RString) -> RubyResult<Boolean> {
        Ok(Boolean::new(
            self.inner().contains_namespace(namespace_name.to_str()),
        ))
    }
}
