use crate::prelude::*;
use rutie::Integer;
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Version {
    Latest = 1,
    Snapshot0 = 2,
    Snapshot1 = 3,
}

impl Version {
    fn to_integer(&self) -> Integer {
        match self {
            Self::Latest => Integer::new(1),
            Self::Snapshot0 => Integer::new(2),
            Self::Snapshot1 => Integer::new(3),
        }
    }
}

impl From<&wasmer_wasi::WasiVersion> for Version {
    fn from(value: &wasmer_wasi::WasiVersion) -> Self {
        match value {
            wasmer_wasi::WasiVersion::Latest => Self::Latest,
            wasmer_wasi::WasiVersion::Snapshot0 => Self::Snapshot0,
            wasmer_wasi::WasiVersion::Snapshot1 => Self::Snapshot1,
        }
    }
}

impl Into<wasmer_wasi::WasiVersion> for Version {
    fn into(self) -> wasmer_wasi::WasiVersion {
        match self {
            Self::Latest => wasmer_wasi::WasiVersion::Latest,
            Self::Snapshot0 => wasmer_wasi::WasiVersion::Snapshot0,
            Self::Snapshot1 => wasmer_wasi::WasiVersion::Snapshot1,
        }
    }
}

impl TryFrom<&Integer> for Version {
    type Error = &'static str;

    fn try_from(value: &Integer) -> Result<Self, Self::Error> {
        Ok(match value.to_i32() {
            1 => Version::Latest,
            2 => Version::Snapshot0,
            3 => Version::Snapshot1,
            _ => return Err("Unrecognized WASI version"),
        })
    }
}
