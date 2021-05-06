use crate::{
    error::{to_ruby_err, RuntimeError},
    prelude::*,
    store::Store,
    types::MemoryType,
};
use rutie::{AnyObject, Fixnum};
use std::convert::{TryFrom, TryInto};

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

    pub fn r#type(&self) -> RubyResult<AnyObject> {
        Ok(MemoryType::ruby_new(self.inner().ty().into()))
    }

    pub fn size(&self) -> RubyResult<Fixnum> {
        Ok(Fixnum::new(self.inner().size().0.into()))
    }

    pub fn data_size(&self) -> RubyResult<Fixnum> {
        Ok(Fixnum::new(
            self.inner()
                .data_size()
                .try_into()
                .map_err(to_ruby_err::<RuntimeError, _>)?,
        ))
    }

    pub fn grow(&self, number_of_pages: &Fixnum) -> RubyResult<Fixnum> {
        Ok(Fixnum::new(
            self.inner()
                .grow(
                    u32::try_from(number_of_pages.to_u64())
                        .map_err(to_ruby_err::<RuntimeError, _>)?,
                )
                .map_err(to_ruby_err::<RuntimeError, _>)
                .and_then(|pages| pages.0.try_into().map_err(to_ruby_err::<RuntimeError, _>))?,
        ))
    }
}
