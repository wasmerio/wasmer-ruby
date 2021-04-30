use crate::{
    error::{to_ruby_err, RuntimeError},
    prelude::*,
    store::Store,
    types::ExportType,
};
use rutie::{AnyObject, Array, Boolean, Encoding, NilClass, Object, RString};
use std::convert::TryFrom;

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
    pub fn new(store: &Store, bytes: &RString) -> RubyResult<AnyObject> {
        let module = wasmer::Module::new(store.inner(), bytes.to_str_unchecked());

        Ok(Module::ruby_new(Module {
            inner: module.map_err(to_ruby_err::<RuntimeError, _>)?,
        }))
    }

    pub fn validate(store: &Store, bytes: &AnyObject) -> RubyResult<Boolean> {
        Ok(Boolean::new(match bytes.try_convert_to::<RString>() {
            Ok(bytes) => {
                wasmer::Module::validate(store.inner(), bytes.to_str_unchecked().as_bytes()).is_ok()
            }
            _ => false,
        }))
    }

    pub fn get_name(&self) -> RubyResult<AnyObject> {
        Ok(self.inner().name().map_or_else(
            || NilClass::new().to_any_object(),
            |name| RString::new_utf8(name).to_any_object(),
        ))
    }

    pub fn set_name(&mut self, name: &RString) -> RubyResult<NilClass> {
        self.inner_mut().set_name(name.to_str());

        Ok(NilClass::new())
    }

    pub fn exports(&self) -> RubyResult<Array> {
        let exports = self.inner.exports();
        let mut array = Array::with_capacity(exports.len());

        for export_type in exports.map(|export_type| ExportType::try_from(export_type)) {
            array.push(ExportType::ruby_new(export_type?));
        }

        Ok(array)
    }

    pub fn custom_sections(&self, name: &RString) -> RubyResult<Array> {
        Ok(self
            .inner()
            .custom_sections(name.to_str())
            .map(|custom_section| {
                RString::from_bytes(&custom_section, &Encoding::us_ascii()).to_any_object()
            })
            .collect())
    }

    pub fn serialize(&self) -> RubyResult<RString> {
        Ok(RString::from_bytes(
            self.inner()
                .serialize()
                .map_err(to_ruby_err::<RuntimeError, _>)?
                .as_slice(),
            &Encoding::us_ascii(),
        ))
    }
}
