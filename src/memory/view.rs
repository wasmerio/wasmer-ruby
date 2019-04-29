use lazy_static::lazy_static;
use rutie::{class, methods, wrappable_struct, Fixnum, Integer, NilClass, Object};
use std::{mem::size_of, rc::Rc};
use wasmer_runtime as runtime;

pub struct Uint8Array {
    memory: Rc<runtime::memory::Memory>,
    offset: usize,
}

impl Uint8Array {
    pub fn new(memory: Rc<runtime::Memory>, offset: usize) -> Self {
        Self { memory, offset }
    }

    pub fn len(&self) -> usize {
        self.memory.view::<u8>()[self.offset..].len() / size_of::<u8>()
    }

    pub fn set(&self, index: isize, value: u8) -> Result<(), &str> {
        let offset = self.offset;
        let view = self.memory.view::<u8>();

        if index < 0 {
            return Err("foo");
        }

        let index = index as usize;

        if view.len() <= offset + index {
            Err("bar")
        } else {
            view[offset + index].set(value);

            Ok(())
        }
    }

    pub fn get(&self, index: isize) -> Result<u8, &str> {
        let offset = self.offset;
        let view = self.memory.view::<u8>();

        if index < 0 {
            return Err("foo");
        }

        let index = index as usize;

        if view.len() <= offset + index {
            Err("bar")
        } else {
            Ok(view[offset + index].get())
        }
    }
}

wrappable_struct!(Uint8Array, Uint8ArrayWrapper, UINT8ARRAY_WRAPPER);

class!(RubyUint8Array);

#[rustfmt::skip]
methods!(
    RubyUint8Array,
    itself,

    fn ruby_uint8array_length() -> Fixnum {
        Fixnum::new(itself.get_data(&*UINT8ARRAY_WRAPPER).len() as i64)
    }

    fn ruby_uint8array_set(index: Integer, value: Integer) -> NilClass {
        let uint8array = itself.get_data(&*UINT8ARRAY_WRAPPER);
        uint8array.set(index.unwrap().to_i32() as isize, value.unwrap().to_i32() as u8).unwrap();

        NilClass::new()
    }

    fn ruby_uint8array_get(index: Integer) -> Fixnum {
        let uint8array = itself.get_data(&*UINT8ARRAY_WRAPPER);

        Fixnum::new(uint8array.get(index.unwrap().to_i32() as isize).unwrap() as i64)
    }
);
