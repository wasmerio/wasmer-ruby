use crate::{
    error::{to_ruby_err, TypeError},
    prelude::*,
};
use rutie::{AnyException, AnyObject, Array, Boolean, Integer, NilClass, Object};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Type {
    I32 = 1,
    I64 = 2,
    F32 = 3,
    F64 = 4,
    V128 = 5,
    ExternRef = 6,
    FuncRef = 7,
}

impl Type {
    fn to_integer(&self) -> Integer {
        match self {
            Self::I32 => Integer::new(1),
            Self::I64 => Integer::new(2),
            Self::F32 => Integer::new(3),
            Self::F64 => Integer::new(4),
            Self::V128 => Integer::new(5),
            Self::ExternRef => Integer::new(6),
            Self::FuncRef => Integer::new(7),
        }
    }
}

impl From<&wasmer::Type> for Type {
    fn from(value: &wasmer::Type) -> Self {
        match value {
            wasmer::Type::I32 => Self::I32,
            wasmer::Type::I64 => Self::I64,
            wasmer::Type::F32 => Self::F32,
            wasmer::Type::F64 => Self::F64,
            wasmer::Type::V128 => Self::V128,
            wasmer::Type::ExternRef => Self::ExternRef,
            wasmer::Type::FuncRef => Self::FuncRef,
        }
    }
}

impl Into<wasmer::Type> for Type {
    fn into(self) -> wasmer::Type {
        match self {
            Self::I32 => wasmer::Type::I32,
            Self::I64 => wasmer::Type::I64,
            Self::F32 => wasmer::Type::F32,
            Self::F64 => wasmer::Type::F64,
            Self::V128 => wasmer::Type::V128,
            Self::ExternRef => wasmer::Type::ExternRef,
            Self::FuncRef => wasmer::Type::FuncRef,
        }
    }
}

impl TryFrom<&Integer> for Type {
    type Error = &'static str;

    fn try_from(value: &Integer) -> Result<Self, Self::Error> {
        Ok(match value.to_i32() {
            1 => Type::I32,
            2 => Type::I64,
            3 => Type::F32,
            4 => Type::F64,
            5 => Type::V128,
            6 => Type::ExternRef,
            7 => Type::FuncRef,
            _ => return Err("Unrecognized type"),
        })
    }
}

#[rubyclass(module = "Wasmer")]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub results: Vec<Type>,
}

#[rubymethods]
impl FunctionType {
    pub fn new(params: &Array, results: &Array) -> RubyResult<AnyObject> {
        let params = unsafe { params.to_any_object().to::<Array>() }
            .into_iter()
            .map(|param| {
                param
                    .try_convert_to::<Integer>()
                    .and_then(|param| Type::try_from(&param).map_err(to_ruby_err::<TypeError, _>))
            })
            .collect::<Result<Vec<Type>, AnyException>>()?;
        let results = unsafe { results.to_any_object().to::<Array>() }
            .into_iter()
            .map(|result| {
                result
                    .try_convert_to::<Integer>()
                    .and_then(|result| Type::try_from(&result).map_err(to_ruby_err::<TypeError, _>))
            })
            .collect::<Result<Vec<Type>, AnyException>>()?;

        Ok(FunctionType::ruby_new(FunctionType { params, results }))
    }

    pub fn params(&self) -> RubyResult<Array> {
        Ok(self
            .params
            .iter()
            .map(|ty| Type::to_integer(ty).to_any_object())
            .collect())
    }

    pub fn results(&self) -> RubyResult<Array> {
        Ok(self
            .results
            .iter()
            .map(|ty| Type::to_integer(ty).to_any_object())
            .collect())
    }
}

#[rubyclass(module = "Wasmer")]
pub struct MemoryType {
    pub minimum: u32,
    pub maximum: Option<u32>,
    pub shared: bool,
}

#[rubymethods]
impl MemoryType {
    pub fn new(minimum: &Integer, maximum: &AnyObject, shared: &Boolean) -> RubyResult<AnyObject> {
        Ok(MemoryType::ruby_new(MemoryType {
            minimum: minimum.to_u64() as _,
            maximum: if maximum.is_nil() {
                None
            } else {
                Some(maximum.try_convert_to::<Integer>()?.to_u64() as _)
            },
            shared: shared.to_bool(),
        }))
    }

    pub fn minimum(&self) -> RubyResult<Integer> {
        Ok(Integer::new(self.minimum.into()))
    }

    pub fn maximum(&self) -> RubyResult<AnyObject> {
        Ok(match self.maximum {
            Some(maximum) => Integer::new(maximum.into()).to_any_object(),
            None => NilClass::new().to_any_object(),
        })
    }

    pub fn shared(&self) -> RubyResult<Boolean> {
        Ok(Boolean::new(self.shared))
    }
}

#[rubyclass(module = "Wasmer")]
pub struct GlobalType {
    pub ty: Type,
    pub mutable: bool,
}

#[rubymethods]
impl GlobalType {
    pub fn new(ty: &Integer, mutable: &Boolean) -> RubyResult<AnyObject> {
        Ok(GlobalType::ruby_new(GlobalType {
            ty: Type::try_from(ty).map_err(to_ruby_err::<TypeError, _>)?,
            mutable: mutable.to_bool(),
        }))
    }

    pub fn r#type(&self) -> RubyResult<Integer> {
        Ok(self.ty.to_integer())
    }

    pub fn mutable(&self) -> RubyResult<Boolean> {
        Ok(Boolean::new(self.mutable))
    }
}

#[rubyclass(module = "Wasmer")]
pub struct TableType {
    pub ty: Type,
    pub minimum: u32,
    pub maximum: Option<u32>,
}

#[rubymethods]
impl TableType {
    pub fn new(ty: &Integer, minimum: &Integer, maximum: &AnyObject) -> RubyResult<AnyObject> {
        Ok(TableType::ruby_new(TableType {
            ty: Type::try_from(ty).map_err(to_ruby_err::<TypeError, _>)?,
            minimum: minimum.to_u64() as _,
            maximum: if maximum.is_nil() {
                None
            } else {
                Some(maximum.try_convert_to::<Integer>()?.to_u64() as _)
            },
        }))
    }

    pub fn r#type(&self) -> RubyResult<Integer> {
        Ok(self.ty.to_integer())
    }

    pub fn minimum(&self) -> RubyResult<Integer> {
        Ok(Integer::new(self.minimum.into()))
    }

    pub fn maximum(&self) -> RubyResult<AnyObject> {
        Ok(match self.maximum {
            Some(maximum) => Integer::new(maximum.into()).to_any_object(),
            None => NilClass::new().to_any_object(),
        })
    }
}
