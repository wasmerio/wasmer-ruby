pub mod view;

use crate::memory::view::{MemoryView, RubyMemoryView, MEMORY_VIEW_WRAPPER};
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

    pub fn view(&self, offset: usize) -> MemoryView {
        MemoryView::new(self.memory.clone(), offset)
    }
}

wrappable_struct!(Memory, MemoryWrapper, MEMORY_WRAPPER);

class!(RubyMemory);

#[rustfmt::skip]
methods!(
    RubyMemory,
    itself,

    // Glue code to call the `Memory.view` method.
    fn ruby_memory_view(offset: Integer) -> RubyMemoryView {
        let offset = offset
            .map(|offset| offset.to_i64() as usize)
            .unwrap_or(0);
        let memory_view = itself.get_data(&*MEMORY_WRAPPER).view(offset);

        Class::from_existing("MemoryView").wrap_data(memory_view, &*MEMORY_VIEW_WRAPPER)
    }
);
