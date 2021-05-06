use crate::{
    error::{to_ruby_err, RuntimeError},
    prelude::*,
    store::Store,
    types::MemoryType,
};
use rutie::AnyObject;

#[rubyclass(module = "Wasmer")]
pub struct Memory {
    inner: wasmer::Memory,
}

impl Memory {
    pub fn raw_new(inner: wasmer::Memory) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wasmer::Memory {
        &self.inner
    }
}

#[rubymethods]
impl Memory {
    pub fn new(store: &Store, memory_type: &MemoryType) -> RubyResult<AnyObject> {
        Ok(Memory::ruby_new(Memory::raw_new(
            wasmer::Memory::new(store.inner(), memory_type.into())
                .map_err(to_ruby_err::<RuntimeError, _>)?,
        )))
    }
}
