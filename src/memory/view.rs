//! The `TypedArray`/`MemoryView` WebAssembly classes.

#[rustfmt::skip]

macro_rules! memory_view {
    ($mod_name:ident over $wasm_type:ty | $bytes_per_element:expr) => {
        pub mod $mod_name {
            use crate::error::unwrap_or_raise;
            use lazy_static::lazy_static;
            use rutie::{
                class, methods, wrappable_struct, AnyException, Exception, Fixnum, Integer,
                NilClass, Object, VM,
            };
            use std::{mem::size_of, rc::Rc};
            use wasmer_runtime as runtime;

            pub struct MemoryView {
                memory: Rc<runtime::memory::Memory>,
                offset: usize,
            }

            impl MemoryView {
                pub fn new(memory: Rc<runtime::Memory>, offset: usize) -> Self {
                    Self { memory, offset }
                }

                pub fn len(&self) -> usize {
                    self.memory.view::<$wasm_type>()[self.offset..].len() / size_of::<$wasm_type>()
                }

                pub fn set(&self, index: isize, value: $wasm_type) -> Result<(), String> {
                    let offset = self.offset;
                    let view = self.memory.view::<$wasm_type>();

                    if index < 0 {
                        return Err("Out of bound: Index cannot be negative.".into());
                    }

                    let index = index as usize;

                    if view.len() <= offset + index {
                        Err(format!(
                            "Out of bound: Maximum index {} is larger than the memory size {}.",
                            offset + index,
                            view.len()
                        ))
                    } else {
                        view[offset + index].set(value);

                        Ok(())
                    }
                }

                pub fn get(&self, index: isize) -> Result<$wasm_type, String> {
                    let offset = self.offset;
                    let view = self.memory.view::<$wasm_type>();

                    if index < 0 {
                        return Err("Out of bound: Index cannot be negative.".into());
                    }

                    let index = index as usize;

                    if view.len() <= offset + index {
                        Err(format!(
                            "Out of bound: Maximum index {} is larger than the memory size {}.",
                            offset + index,
                            view.len()
                        ))
                    } else {
                        Ok(view[offset + index].get())
                    }
                }

                pub fn each(&self) {
                    let view = self.memory.view::<$wasm_type>();

                    for nth in self.offset..view.len() {
                        let value = view[nth].get() as i64;
                        VM::yield_object(Integer::from(value));
                    }
                }
            }

            wrappable_struct!(MemoryView, MemoryViewWrapper, MEMORY_VIEW_WRAPPER);

            class!(RubyMemoryView);

            methods!(
                RubyMemoryView,
                _itself,

                // The `TypedArray.bytes_per_element` method.
                fn ruby_memory_view_bytes_per_element() -> Fixnum {
                    Fixnum::new($bytes_per_element)
                }

                // Glue code to call the `TypedArray.length` method.
                fn ruby_memory_view_length() -> Fixnum {
                    Fixnum::new(_itself.get_data(&*MEMORY_VIEW_WRAPPER).len() as i64)
                }

                // Glue code to call the `TypedArray.set` method.
                fn ruby_memory_view_set(index: Integer, value: Integer) -> NilClass {
                    unwrap_or_raise(|| {
                        let memory_view = _itself.get_data(&*MEMORY_VIEW_WRAPPER);

                        memory_view
                            .set(index?.to_i32() as isize, value?.to_i32() as $wasm_type)
                            .map_err(|e| AnyException::new("ArgumentError", Some(&e)))?;

                        Ok(NilClass::new())
                    })
                }

                // Glue code to call the `TypedArray.get` method.
                fn ruby_memory_view_get(index: Integer) -> Fixnum {
                    unwrap_or_raise(|| {
                        let memory_view = _itself.get_data(&*MEMORY_VIEW_WRAPPER);

                        Ok(Fixnum::new(
                            memory_view
                                .get(index?.to_i32() as isize)
                                .map_err(|e| AnyException::new("ArgumentError", Some(&e)))?
                                as i64,
                        ))
                    })
                }

                fn ruby_memory_view_each() -> RubyMemoryView {
                    let memory_view = _itself.get_data(&*MEMORY_VIEW_WRAPPER);
                    memory_view.each();
                    _itself
                }
            );
        }
    };
}

memory_view!(uint8array over u8|1);
memory_view!(int8array over i8|1);
memory_view!(uint16array over u16|2);
memory_view!(int16array over i16|2);
memory_view!(uint32array over u32|4);
memory_view!(int32array over i32|4);
