use crate::{
    error::{to_ruby_err, RuntimeError},
    prelude::*,
};
use rutie::{Boolean, Fixnum, Symbol};
use std::convert::TryInto;

#[rubyclass(module = "Wasmer")]
pub struct Exports {
    inner: wasmer::Exports,
}

impl Exports {
    pub fn new(inner: wasmer::Exports) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wasmer::Exports {
        &self.inner
    }
}

#[rubymethods]
impl Exports {
    pub fn respond_to_missing(
        &self,
        symbol: &Symbol,
        _include_private: &Boolean,
    ) -> RubyResult<Boolean> {
        Ok(Boolean::new(self.inner().contains(symbol.to_str())))
    }

    pub fn length(&self) -> RubyResult<Fixnum> {
        Ok(Fixnum::new(
            self.inner()
                .len()
                .try_into()
                .map_err(to_ruby_err::<RuntimeError, _>)?,
        ))
    }
}

pub(crate) mod ruby_exports_extra {
    use crate::{
        error::{to_ruby_err, unwrap_or_raise, NameError},
        externals::{Function, Global, Memory, Table},
    };
    use rutie::{
        rubysys::class,
        types::{Argc, Value},
        util::str_to_cstring,
        AnyObject, Array, Object, Symbol,
    };
    use rutie_derive::UpcastRubyClass;

    #[allow(improper_ctypes_definitions)] // No choice, that's how `rutie` is designed.
    pub extern "C" fn method_missing(
        argc: Argc,
        argv: *const AnyObject,
        itself: super::RubyExports,
    ) -> AnyObject {
        unwrap_or_raise(|| {
            let arguments = Value::from(0);

            unsafe {
                let argv_pointer = argv as *const Value;

                class::rb_scan_args(argc, argv_pointer, str_to_cstring("*").as_ptr(), &arguments)
            };

            let exports = itself.upcast();
            let mut arguments = Array::from(arguments);
            let extern_name = arguments.shift().try_convert_to::<Symbol>()?;
            let extern_name = extern_name.to_str();

            Ok(match exports.inner().get_extern(extern_name) {
                Some(wasmer::Extern::Function(function)) => {
                    Function::ruby_new(Function::raw_new(function.clone())).to_any_object()
                }
                Some(wasmer::Extern::Memory(memory)) => {
                    Memory::ruby_new(Memory::raw_new(memory.clone())).to_any_object()
                }
                Some(wasmer::Extern::Global(global)) => {
                    Global::ruby_new(Global::raw_new(global.clone())).to_any_object()
                }
                Some(wasmer::Extern::Table(table)) => {
                    Table::ruby_new(Table::raw_new(table.clone())).to_any_object()
                }
                None => {
                    return Err(to_ruby_err::<NameError, _>(format!(
                        "Export `{}` does not exist",
                        extern_name
                    )))
                }
            })
        })
    }
}
