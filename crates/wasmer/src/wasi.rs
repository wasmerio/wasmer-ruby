use crate::{
    error::{to_ruby_err, unwrap_or_raise, RuntimeError, TypeError},
    import_object::ImportObject,
    module::Module,
    prelude::*,
    store::Store,
};
use rutie::{AnyObject, Array, Boolean, Hash, Integer, NilClass, Object, RString};
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

        self.inner.envs(environment_pairs);

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

    pub fn map_directories(&mut self, map_directories: &Hash) -> RubyResult<RubyStateBuilder> {
        let mut map_directory_pairs = Vec::with_capacity(map_directories.length());

        map_directories.each(|key, value| {
            unwrap_or_raise(|| {
                map_directory_pairs.push((
                    key.try_convert_to::<RString>()?.to_string(),
                    PathBuf::from(value.try_convert_to::<RString>()?.to_str()),
                ));

                Ok(())
            });
        });

        self.inner
            .map_dirs(map_directory_pairs)
            .map_err(to_ruby_err::<RuntimeError, _>)?;

        Ok(_ruby_self)
    }

    pub fn map_directory(
        &mut self,
        alias: &RString,
        directory: &RString,
    ) -> RubyResult<RubyStateBuilder> {
        self.inner
            .map_dir(alias.to_str(), PathBuf::from(directory.to_str()))
            .map_err(to_ruby_err::<RuntimeError, _>)?;

        Ok(_ruby_self)
    }

    pub fn finalize(&mut self) -> RubyResult<AnyObject> {
        Ok(Environment::ruby_new(Environment {
            inner: self
                .inner
                .finalize()
                .map_err(to_ruby_err::<RuntimeError, _>)?,
        }))
    }
}

#[rubyclass(module = "Wasmer::Wasi")]
pub struct Environment {
    inner: wasmer_wasi::WasiEnv,
}

#[rubymethods]
impl Environment {
    pub fn generate_import_object(
        &self,
        store: &Store,
        wasi_version: &Integer,
    ) -> RubyResult<AnyObject> {
        let import_object = wasmer_wasi::generate_import_object_from_env(
            store.inner(),
            self.inner.clone(),
            Version::try_from(wasi_version)
                .map_err(to_ruby_err::<TypeError, _>)?
                .into(),
        );

        Ok(ImportObject::ruby_new(ImportObject::raw_new(import_object)))
    }
}

#[rubyfunction]
pub fn get_version(module: &Module, strict: &Boolean) -> RubyResult<AnyObject> {
    Ok(
        wasmer_wasi::get_wasi_version(&module.inner(), strict.to_bool())
            .map(|version| Version::from(&version).to_integer().to_any_object())
            .unwrap_or_else(|| NilClass::new().to_any_object()),
    )
}
