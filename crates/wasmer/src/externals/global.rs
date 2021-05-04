use crate::{prelude::*, store::Store, values::Value};
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
}
