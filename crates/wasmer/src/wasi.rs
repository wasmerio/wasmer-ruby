use crate::{
    error::{to_ruby_err, unwrap_or_raise, RuntimeError},
    prelude::*,
};
use rutie::{AnyObject, Array, Hash, Integer, Object, RString};
use std::{convert::TryFrom, path::PathBuf};

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

#[rubyclass(module = "Wasmer::Wasi")]
pub struct StateBuilder {
    inner: wasmer_wasi::WasiStateBuilder,
}

#[rubymethods]
impl StateBuilder {
    pub fn new(program_name: &RString) -> RubyResult<AnyObject> {
        Ok(StateBuilder::ruby_new(StateBuilder {
            inner: wasmer_wasi::WasiState::new(program_name.to_str()),
        }))
    }

    pub fn arguments(&mut self, arguments: &Array) -> RubyResult<RubyStateBuilder> {
        self.inner.args(
            unsafe { arguments.to_any_object().to::<Array>() }
                .into_iter()
                .map(|argument| {
                    argument
                        .try_convert_to::<RString>()
                        .map(|argument| argument.to_string())
                })
                .collect::<RubyResult<Vec<_>>>()?,
        );

        Ok(_ruby_self)
    }

    pub fn argument(&mut self, argument: &RString) -> RubyResult<RubyStateBuilder> {
        self.inner.arg(argument.to_str());

        Ok(_ruby_self)
    }

    pub fn environments(&mut self, environments: &Hash) -> RubyResult<RubyStateBuilder> {
        let mut environment_pairs = Vec::with_capacity(environments.length());

        environments.each(|key, value| {
            unwrap_or_raise(|| {
                environment_pairs.push((
                    key.try_convert_to::<RString>()?.to_string(),
                    value.try_convert_to::<RString>()?.to_string(),
                ));

                Ok(())
            });
        });

        Ok(_ruby_self)
    }

    pub fn environment(&mut self, key: &RString, value: &RString) -> RubyResult<RubyStateBuilder> {
        self.inner.env(key.to_str(), value.to_str());

        Ok(_ruby_self)
    }

    pub fn preopen_directories(
        &mut self,
        preopen_directories: &Array,
    ) -> RubyResult<RubyStateBuilder> {
        self.inner
            .preopen_dirs(
                unsafe { preopen_directories.to_any_object().to::<Array>() }
                    .into_iter()
                    .map(|directory| {
                        Ok(PathBuf::from(
                            directory.try_convert_to::<RString>()?.to_string(),
                        ))
                    })
                    .collect::<RubyResult<Vec<_>>>()?,
            )
            .map_err(to_ruby_err::<RuntimeError, _>)?;

        Ok(_ruby_self)
    }

    pub fn preopen_directory(
        &mut self,
        preopen_directory: &RString,
    ) -> RubyResult<RubyStateBuilder> {
        self.inner
            .preopen_dir(preopen_directory.to_str())
            .map_err(to_ruby_err::<RuntimeError, _>)?;

        Ok(_ruby_self)
    }
}
