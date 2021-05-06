use crate::{
    error::{to_ruby_err, RuntimeError},
    prelude::*,
    store::Store,
    types::TableType,
    values::Value,
};
use rutie::AnyObject;

#[rubyclass(module = "Wasmer")]
pub struct Table {
    inner: wasmer::Table,
}

impl Table {
    pub fn raw_new(inner: wasmer::Table) -> Self {
        Self { inner }
    }

    pub(crate) fn inner(&self) -> &wasmer::Table {
        &self.inner
    }
}

#[rubymethods]
impl Table {
    pub fn new(
        store: &Store,
        table_type: &TableType,
        initial_value: &Value,
    ) -> RubyResult<AnyObject> {
        Ok(Table::ruby_new(Table::raw_new(
            wasmer::Table::new(
                store.inner(),
                table_type.into(),
                initial_value.inner().clone(),
            )
            .map_err(to_ruby_err::<RuntimeError, _>)?,
        )))
    }
}
