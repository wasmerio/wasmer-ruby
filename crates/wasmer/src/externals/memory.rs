use crate::{
    error::{to_ruby_err, ArgumentError, RuntimeError},
    memory::views::{Int16Array, Int32Array, Int8Array, Uint16Array, Uint32Array, Uint8Array},
    prelude::*,
    store::Store,
    types::MemoryType,
};
use rutie::{AnyObject, Fixnum, Integer, Object};
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

    pub fn uint8_view(&self, offset: &AnyObject) -> RubyResult<AnyObject> {
        Ok(Uint8Array::ruby_new(Uint8Array::new(
            self.inner().clone(),
            if offset.is_nil() {
                0
            } else {
                offset
                    .try_convert_to::<Integer>()?
                    .to_u64()
                    .try_into()
                    .map_err(to_ruby_err::<ArgumentError, _>)?
            },
        )))
    }

    pub fn int8_view(&self, offset: &AnyObject) -> RubyResult<AnyObject> {
        Ok(Int8Array::ruby_new(Int8Array::new(
            self.inner().clone(),
            if offset.is_nil() {
                0
            } else {
                offset
                    .try_convert_to::<Integer>()?
                    .to_u64()
                    .try_into()
                    .map_err(to_ruby_err::<ArgumentError, _>)?
            },
        )))
    }

    pub fn uint16_view(&self, offset: &AnyObject) -> RubyResult<AnyObject> {
        Ok(Uint16Array::ruby_new(Uint16Array::new(
            self.inner().clone(),
            if offset.is_nil() {
                0
            } else {
                offset
                    .try_convert_to::<Integer>()?
                    .to_u64()
                    .try_into()
                    .map_err(to_ruby_err::<ArgumentError, _>)?
            },
        )))
    }

    pub fn int16_view(&self, offset: &AnyObject) -> RubyResult<AnyObject> {
        Ok(Int16Array::ruby_new(Int16Array::new(
            self.inner().clone(),
            if offset.is_nil() {
                0
            } else {
                offset
                    .try_convert_to::<Integer>()?
                    .to_u64()
                    .try_into()
                    .map_err(to_ruby_err::<ArgumentError, _>)?
            },
        )))
    }

    pub fn uint32_view(&self, offset: &AnyObject) -> RubyResult<AnyObject> {
        Ok(Uint32Array::ruby_new(Uint32Array::new(
            self.inner().clone(),
            if offset.is_nil() {
                0
            } else {
                offset
                    .try_convert_to::<Integer>()?
                    .to_u64()
                    .try_into()
                    .map_err(to_ruby_err::<ArgumentError, _>)?
            },
        )))
    }

    pub fn int32_view(&self, offset: &AnyObject) -> RubyResult<AnyObject> {
        Ok(Int32Array::ruby_new(Int32Array::new(
            self.inner().clone(),
            if offset.is_nil() {
                0
            } else {
                offset
                    .try_convert_to::<Integer>()?
                    .to_u64()
                    .try_into()
                    .map_err(to_ruby_err::<ArgumentError, _>)?
            },
        )))
    }
}
