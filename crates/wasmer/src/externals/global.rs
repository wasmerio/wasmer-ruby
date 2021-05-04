use crate::{
    prelude::*,
    store::Store,
    types::GlobalType,
    values::{to_ruby_object, Value},
};
use rutie::{AnyObject, Boolean};

#[rubyclass(module = "Wasmer")]
pub struct Global {
    inner: wasmer::Global,
}

impl Global {
    pub fn raw_new(inner: wasmer::Global) -> Self {
        Self { inner }
    }

    fn inner(&self) -> &wasmer::Global {
        &self.inner
    }
}

#[rubymethods]
impl Global {
    pub fn new(store: &Store, value: &Value, mutable: &Boolean) -> RubyResult<AnyObject> {
        Ok(Global::ruby_new(Global::raw_new(if mutable.to_bool() {
            wasmer::Global::new_mut(store.inner(), value.inner().clone())
        } else {
            wasmer::Global::new(store.inner(), value.inner().clone())
        })))
    }

    pub fn mutable(&self) -> RubyResult<Boolean> {
        Ok(Boolean::new(self.inner().ty().mutability.is_mutable()))
    }

    pub fn value(&self) -> RubyResult<AnyObject> {
        Ok(to_ruby_object(&self.inner.get()))
    }

    pub fn r#type(&self) -> RubyResult<AnyObject> {
        Ok(GlobalType::ruby_new(self.inner().ty().into()))
    }
}
