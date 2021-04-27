use crate::{
    error::{to_ruby_err, RuntimeError, TypeError},
    prelude::*,
    store::RubyStore,
};
use lazy_static::lazy_static;
use rutie::{AnyObject, Boolean, NilClass, Object, RString};

#[rubyclass(module = "Wasmer")]
pub struct Module {
    inner: wasmer::Module,
}

impl Module {
    pub(crate) fn inner(&self) -> &wasmer::Module {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut wasmer::Module {
        &mut self.inner
    }
}

#[rubymethods]
impl Module {
    pub fn new(store: RubyStore, bytes: AnyObject) -> RubyResult<AnyObject> {
        let module = match bytes.try_convert_to::<RString>() {
            Ok(bytes) => wasmer::Module::new(store.unwrap().inner(), bytes.to_str_unchecked()),
            _ => {
                return Err(to_ruby_err::<TypeError, _>(
                    "`Module` accepts Wasm bytes or a WAT string",
                ))
            }
        };

        Ok(Module::wrap(Module {
            inner: module.map_err(to_ruby_err::<RuntimeError, _>)?,
        }))
    }

    pub fn validate(store: RubyStore, bytes: AnyObject) -> RubyResult<Boolean> {
        Ok(Boolean::new(match bytes.try_convert_to::<RString>() {
            Ok(bytes) => wasmer::Module::validate(
                store.unwrap().inner(),
                bytes.to_str_unchecked().as_bytes(),
            )
            .is_ok(),
            _ => false,
        }))
    }

    pub fn get_name(&self) -> RubyResult<AnyObject> {
        Ok(self.unwrap().inner().name().map_or_else(
            || NilClass::new().to_any_object(),
            |name| RString::new_utf8(name).to_any_object(),
        ))
    }

    pub fn set_name(&mut self, name: RString) -> RubyResult<NilClass> {
        self.unwrap_mut().inner_mut().set_name(name.to_str());

        Ok(NilClass::new())
    }
}
