//! The `Memory` WebAssembly class.

pub mod view;

use crate::memory::view::{
    int16array::{RubyMemoryView as RubyInt16Array, MEMORY_VIEW_WRAPPER as INT16ARRAY_WRAPPER},
    int32array::{RubyMemoryView as RubyInt32Array, MEMORY_VIEW_WRAPPER as INT32ARRAY_WRAPPER},
    int8array::{RubyMemoryView as RubyInt8Array, MEMORY_VIEW_WRAPPER as INT8ARRAY_WRAPPER},
    uint16array::{RubyMemoryView as RubyUint16Array, MEMORY_VIEW_WRAPPER as UINT16ARRAY_WRAPPER},
    uint32array::{RubyMemoryView as RubyUint32Array, MEMORY_VIEW_WRAPPER as UINT32ARRAY_WRAPPER},
    uint8array::{RubyMemoryView as RubyUint8Array, MEMORY_VIEW_WRAPPER as UINT8ARRAY_WRAPPER},
};
use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, Class, Integer, Object};
use std::rc::Rc;
use wasmer_runtime as runtime;

pub struct Memory {
    memory: Rc<runtime::Memory>,
}

impl Memory {
    pub fn new(memory: Rc<runtime::Memory>) -> Self {
        Self { memory }
    }

    pub fn uint8_view(&self, offset: usize) -> view::uint8array::MemoryView {
        view::uint8array::MemoryView::new(self.memory.clone(), offset)
    }

    pub fn int8_view(&self, offset: usize) -> view::int8array::MemoryView {
        view::int8array::MemoryView::new(self.memory.clone(), offset)
    }

    pub fn uint16_view(&self, offset: usize) -> view::uint16array::MemoryView {
        view::uint16array::MemoryView::new(self.memory.clone(), offset)
    }

    pub fn int16_view(&self, offset: usize) -> view::int16array::MemoryView {
        view::int16array::MemoryView::new(self.memory.clone(), offset)
    }

    pub fn uint32_view(&self, offset: usize) -> view::uint32array::MemoryView {
        view::uint32array::MemoryView::new(self.memory.clone(), offset)
    }

    pub fn int32_view(&self, offset: usize) -> view::int32array::MemoryView {
        view::int32array::MemoryView::new(self.memory.clone(), offset)
    }
}

wrappable_struct!(Memory, MemoryWrapper, MEMORY_WRAPPER);

class!(RubyMemory);

#[rustfmt::skip]
methods!(
    RubyMemory,
    itself,

    // Glue code to call the `Memory.uint8_view` method.
    fn ruby_memory_uint8array(offset: Integer) -> RubyUint8Array {
        let offset = offset
            .map(|offset| offset.to_i64() as usize)
            .unwrap_or(0);
        let memory_view = itself.get_data(&*MEMORY_WRAPPER).uint8_view(offset);

        Class::from_existing("Uint8Array").wrap_data(memory_view, &*UINT8ARRAY_WRAPPER)
    }

    // Glue code to call the `Memory.int8_view` method.
    fn ruby_memory_int8array(offset: Integer) -> RubyInt8Array {
        let offset = offset
            .map(|offset| offset.to_i64() as usize)
            .unwrap_or(0);
        let memory_view = itself.get_data(&*MEMORY_WRAPPER).int8_view(offset);

        Class::from_existing("Int8Array").wrap_data(memory_view, &*INT8ARRAY_WRAPPER)
    }

    // Glue code to call the `Memory.uint16_view` method.
    fn ruby_memory_uint16array(offset: Integer) -> RubyUint16Array {
        let offset = offset
            .map(|offset| offset.to_i64() as usize)
            .unwrap_or(0);
        let memory_view = itself.get_data(&*MEMORY_WRAPPER).uint16_view(offset);

        Class::from_existing("Uint16Array").wrap_data(memory_view, &*UINT16ARRAY_WRAPPER)
    }

    // Glue code to call the `Memory.int16_view` method.
    fn ruby_memory_int16array(offset: Integer) -> RubyInt16Array {
        let offset = offset
            .map(|offset| offset.to_i64() as usize)
            .unwrap_or(0);
        let memory_view = itself.get_data(&*MEMORY_WRAPPER).int16_view(offset);

        Class::from_existing("Int16Array").wrap_data(memory_view, &*INT16ARRAY_WRAPPER)
    }

    // Glue code to call the `Memory.uint32_view` method.
    fn ruby_memory_uint32array(offset: Integer) -> RubyUint32Array {
        let offset = offset
            .map(|offset| offset.to_i64() as usize)
            .unwrap_or(0);
        let memory_view = itself.get_data(&*MEMORY_WRAPPER).uint32_view(offset);

        Class::from_existing("Uint32Array").wrap_data(memory_view, &*UINT32ARRAY_WRAPPER)
    }

    // Glue code to call the `Memory.int32_view` method.
    fn ruby_memory_int32array(offset: Integer) -> RubyInt32Array {
        let offset = offset
            .map(|offset| offset.to_i64() as usize)
            .unwrap_or(0);
        let memory_view = itself.get_data(&*MEMORY_WRAPPER).int32_view(offset);

        Class::from_existing("Int32Array").wrap_data(memory_view, &*INT32ARRAY_WRAPPER)
    }
);
