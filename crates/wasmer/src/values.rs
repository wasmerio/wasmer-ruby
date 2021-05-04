use crate::{
    error::{to_ruby_err, RubyResult, TypeError},
    prelude::*,
};
use rutie::{AnyObject, Fixnum, Float, Object};
use std::convert::TryInto;

pub(crate) fn to_wasm_value((any, ty): (AnyObject, wasmer::Type)) -> RubyResult<wasmer::Value> {
    Ok(match ty {
        wasmer::Type::I32 => wasmer::Value::I32(
            any.try_convert_to::<Fixnum>()?
                .to_u64()
                .try_into()
                .map_err(to_ruby_err::<TypeError, _>)?,
        ),
        wasmer::Type::I64 => wasmer::Value::I64(
            any.try_convert_to::<Fixnum>()?
                .to_u64()
                .try_into()
                .map_err(to_ruby_err::<TypeError, _>)?,
        ),
        wasmer::Type::F32 => wasmer::Value::F32(any.try_convert_to::<Float>()?.to_f64() as _),
        wasmer::Type::F64 => wasmer::Value::F64(any.try_convert_to::<Float>()?.to_f64()),
        _ => unimplemented!(),
    })
}

#[rubyclass(module = "Wasmer")]
pub struct Value {
    inner: wasmer::Value,
}

impl Value {
    pub(crate) fn inner(&self) -> &wasmer::Value {
        &self.inner
    }
}

#[rubymethods]
impl Value {
    pub fn i32(value: &Fixnum) -> RubyResult<AnyObject> {
        Ok(Value::ruby_new(Value {
            inner: wasmer::Value::I32(
                value
                    .to_u64()
                    .try_into()
                    .map_err(to_ruby_err::<TypeError, _>)?,
            ),
        }))
    }

    pub fn i64(value: &Fixnum) -> RubyResult<AnyObject> {
        Ok(Value::ruby_new(Value {
            inner: wasmer::Value::I64(
                value
                    .to_u64()
                    .try_into()
                    .map_err(to_ruby_err::<TypeError, _>)?,
            ),
        }))
    }

    pub fn f32(value: &Float) -> RubyResult<AnyObject> {
        Ok(Value::ruby_new(Value {
            inner: wasmer::Value::F32(value.to_f64() as _),
        }))
    }

    pub fn f64(value: &Float) -> RubyResult<AnyObject> {
        Ok(Value::ruby_new(Value {
            inner: wasmer::Value::F64(value.to_f64()),
        }))
    }
}
