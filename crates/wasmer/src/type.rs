use crate::prelude::*;
use rutie::Integer;
use std::convert::TryFrom;

#[rubyclass(module = "Wasmer")]
pub struct Type {
    inner: wasmer::Type,
}

impl Type {
    pub(crate) fn inner(&self) -> wasmer::Type {
        self.inner
    }
}

impl TryFrom<&Integer> for Type {
    type Error = &'static str;

    fn try_from(value: &Integer) -> Result<Self, Self::Error> {
        Ok(Type {
            inner: match value.to_i32() {
                1 => wasmer::Type::I32,
                2 => wasmer::Type::I64,
                3 => wasmer::Type::F32,
                4 => wasmer::Type::F64,
                5 => wasmer::Type::V128,
                6 => wasmer::Type::ExternRef,
                7 => wasmer::Type::FuncRef,
                _ => return Err("Unrecognized type"),
            },
        })
    }
}
