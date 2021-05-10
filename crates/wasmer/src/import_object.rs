use crate::{
    error::{to_ruby_err, unwrap_or_raise, TypeError},
    externals::{function::RubyFunction, global::RubyGlobal, memory::RubyMemory, table::RubyTable},
    prelude::*,
};
use rutie::{AnyObject, Boolean, Hash, NilClass, Object, RString, Symbol};

#[rubyclass(module = "Wasmer")]
pub struct ImportObject {
    inner: wasmer::ImportObject,
}

impl ImportObject {
    pub(crate) fn raw_new(inner: wasmer::ImportObject) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wasmer::ImportObject {
        &self.inner
    }
}

#[rubymethods]
impl ImportObject {
    pub fn new() -> RubyResult<AnyObject> {
        Ok(ImportObject::ruby_new(ImportObject {
            inner: Default::default(),
        }))
    }

    pub fn contains_namespace(&self, namespace_name: &RString) -> RubyResult<Boolean> {
        Ok(Boolean::new(
            self.inner().contains_namespace(namespace_name.to_str()),
        ))
    }

    pub fn register(&mut self, namespace_name: &RString, namespace: &Hash) -> RubyResult<NilClass> {
        let mut wasmer_namespace = wasmer::Exports::new();

        namespace.each(|key, value| {
            unwrap_or_raise(|| {
                let name = if let Ok(name) = key.try_convert_to::<RString>() {
                    name.to_string()
                } else if let Ok(name) = key.try_convert_to::<Symbol>() {
                    name.to_string()
                } else {
                    return Err(to_ruby_err::<TypeError, _>(format!(
                        "`ImportObject` cannot register the name because it has an invalid type `{:?}` (expects `String` or `Symbol`)",
                        key.ty()
                    )));
                };

                if let Ok(function) = value.try_convert_to::<RubyFunction>() {
                    wasmer_namespace.insert(name, function.upcast().inner().clone());
                } else if let Ok(memory) = value.try_convert_to::<RubyMemory>() {
                    wasmer_namespace.insert(name, memory.upcast().inner().clone());
                } else if let Ok(global) = value.try_convert_to::<RubyGlobal>() {
                    wasmer_namespace.insert(name, global.upcast().inner().clone());
                } else if let Ok(table) = value.try_convert_to::<RubyTable>() {
                    wasmer_namespace.insert(name, table.upcast().inner().clone());
                } else {
                    return Err(to_ruby_err::<TypeError, _>(format!(
                        "`ImportObject` cannot register the given type `{:?}` associated to `{:?}`",
                        value.ty(),
                        name,
                    )));
                }

                Ok(())
            });
        });

        self.inner
            .register(namespace_name.to_str(), wasmer_namespace);

        Ok(NilClass::new())
    }
}
