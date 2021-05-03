use crate::{
    error::{to_ruby_err, RuntimeError},
    exports::Exports,
    module::Module,
    prelude::*,
};
use rutie::{AnyObject, Object};

#[rubyclass(module = "Wasmer")]
pub struct Instance {
    inner: wasmer::Instance,
    exports: AnyObject,
}

#[rubymethods]
impl Instance {
    pub fn new(module: &Module, import_object: &AnyObject) -> RubyResult<AnyObject> {
        let module = module.inner();

        let instance = if import_object.is_nil() {
            wasmer::Instance::new(&module, &wasmer::imports! {})
        } else {
            todo!()
        };

        let instance = instance.map_err(to_ruby_err::<RuntimeError, _>)?;
        let exports = Exports::ruby_new(Exports::new(instance.exports.clone()));

        Ok(Instance::ruby_new(Instance {
            inner: instance,
            exports,
        }))
    }

    pub fn exports(&self) -> RubyResult<AnyObject> {
        Ok(self.exports.clone())
    }
}
