use lazy_static::lazy_static;
use rutie::{class, wrappable_struct};
use std::rc::Rc;
use wasmer_runtime as runtime;

pub struct Memory {
    #[allow(unused)]
    memory: Rc<runtime::Memory>,
}

impl Memory {
    pub fn new(memory: Rc<runtime::Memory>) -> Self {
        Self { memory }
    }
}

wrappable_struct!(Memory, MemoryWrapper, MEMORY_WRAPPER);

class!(RubyMemory);
