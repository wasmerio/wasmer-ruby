use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, Class, Fixnum, Integer, Object};
use std::{mem::size_of, rc::Rc};
use wasmer_runtime as runtime;

pub struct Memory {
    memory: Rc<runtime::Memory>,
}

impl Memory {
    pub fn new(memory: Rc<runtime::Memory>) -> Self {
        Self { memory }
    }

    pub fn view(&self, offset: usize) -> MemoryView {
        MemoryView {
            memory: self.memory.clone(),
            offset,
        }
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

pub struct MemoryView {
    memory: Rc<runtime::memory::Memory>,
    offset: usize,
}

impl MemoryView {
    pub fn len(&self) -> usize {
        self.memory.view::<u8>()[self.offset..].len() / size_of::<u8>()
    }
}

wrappable_struct!(MemoryView, MemoryViewWrapper, MEMORY_VIEW_WRAPPER);

class!(RubyMemoryView);

#[rustfmt::skip]
methods!(
    RubyMemoryView,
    itself,

    fn ruby_memory_view_length() -> Fixnum {
        Fixnum::new(itself.get_data(&*MEMORY_VIEW_WRAPPER).len() as i64)
    }
);
