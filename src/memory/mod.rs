pub mod view;

use crate::memory::view::*;
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

    pub fn uint8_view(&self, offset: usize) -> Uint8Array {
        view::Uint8Array::new(self.memory.clone(), offset)
    }
}

wrappable_struct!(Memory, MemoryWrapper, MEMORY_WRAPPER);

class!(RubyMemory);

#[rustfmt::skip]
methods!(
    RubyMemory,
    itself,

    // Glue code to call the `Memory.view` method.
    fn ruby_memory_uint8array(offset: Integer) -> RubyUint8Array {
        let offset = offset
            .map(|offset| offset.to_i64() as usize)
            .unwrap_or(0);
        let memory_view = itself.get_data(&*MEMORY_WRAPPER).uint8_view(offset);

        Class::from_existing("Uint8Array").wrap_data(memory_view, &*UINT8ARRAY_WRAPPER)
    }
);
