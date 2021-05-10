use crate::{
    error::{to_ruby_err, ArgumentError, IndexError, TypeError},
    prelude::*,
};
use rutie::{Integer, NilClass, VM};
use std::{
    convert::{TryFrom, TryInto},
    mem::size_of,
};

macro_rules! memory_view {
    ($class_name:ident over $wasm_type:ty | $bytes_per_element:expr) => {
        #[rubyclass(module = "Wasmer")]
        pub struct $class_name {
            memory: wasmer::Memory,
            offset: usize,
        }

        impl $class_name {
            pub const BYTES_PER_ELEMENT: u32 = $bytes_per_element;

            pub fn new(memory: wasmer::Memory, offset: usize) -> Self {
                Self { memory, offset }
            }
        }

        #[rubymethods]
        impl $class_name {
            pub fn length(&self) -> RubyResult<Integer> {
                Ok(Integer::new(
                    (self.memory.view::<$wasm_type>()[self.offset..].len()
                        / size_of::<$wasm_type>())
                    .try_into()
                    .map_err(to_ruby_err::<TypeError, _>)?,
                ))
            }

            pub fn set(&self, index: &Integer, value: &Integer) -> RubyResult<NilClass> {
                let index =
                    isize::try_from(index.to_i32()).map_err(to_ruby_err::<ArgumentError, _>)?;
                let value = <$wasm_type>::try_from(value.to_u64())
                    .map_err(to_ruby_err::<ArgumentError, _>)?;

                let offset = self.offset;
                let view = self.memory.view::<$wasm_type>();

                if index < 0 {
                    return Err(to_ruby_err::<IndexError, _>(
                        "Out of bound: Index cannot be negative",
                    ));
                }

                let index = index as usize;

                if view.len() <= offset + index {
                    return Err(to_ruby_err::<IndexError, _>(format!(
                        "Out of bound: Maximum index {} is larger than the memory size {}",
                        offset + index,
                        view.len()
                    )));
                }

                view[offset + index].set(value);

                Ok(NilClass::new())
            }

            pub fn get(&self, index: &Integer) -> RubyResult<Integer> {
                let index =
                    isize::try_from(index.to_i32()).map_err(to_ruby_err::<ArgumentError, _>)?;

                let offset = self.offset;
                let view = self.memory.view::<$wasm_type>();

                if index < 0 {
                    return Err(to_ruby_err::<IndexError, _>(
                        "Out of bound: Index cannot be negative.",
                    ));
                }

                let index = index as usize;

                if view.len() <= offset + index {
                    return Err(to_ruby_err::<IndexError, _>(format!(
                        "Out of bound: Maximum index {} is larger than the memory size {}.",
                        offset + index,
                        view.len()
                    )));
                }

                Ok(Integer::new(view[offset + index].get().into()))
            }

            pub fn each(&self) -> RubyResult<NilClass> {
                let view = self.memory.view::<$wasm_type>();

                for nth in self.offset..view.len() {
                    VM::yield_object(Integer::new(view[nth].get().into()));
                }

                Ok(NilClass::new())
            }
        }
    };
}

memory_view!(Uint8Array over u8|1);
memory_view!(Int8Array over i8|1);
memory_view!(Uint16Array over u16|2);
memory_view!(Int16Array over i16|2);
memory_view!(Uint32Array over u32|4);
memory_view!(Int32Array over i32|4);
