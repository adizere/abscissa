//! Support for managing global configuration, as well as loading it from TOML

mod configurable;
mod overrides;
mod reader;

pub use self::{configurable::Configurable, overrides::Override, reader::Reader};

use crate::{
    path::AbsPath,
    FrameworkError,
    FrameworkErrorKind::{ConfigError, IoError, PathError},
};
use serde::de::DeserializeOwned;
use std::{fmt::Debug, fs::File, io::Read};

/// Trait for Abscissa configuration data structures
pub trait Config: Debug + Default + DeserializeOwned {
    /// Load the configuration from the given TOML string
    fn load_toml(toml_string: impl AsRef<str>) -> Result<Self, FrameworkError>;

    /// Load the global configuration from the TOML file at the given path.
    /// If an error occurs reading or parsing the file, print it out and exit.
    fn load_toml_file(path: impl AsRef<AbsPath>) -> Result<Self, FrameworkError>;
}

impl<C> Config for C
where
    C: Debug + Default + DeserializeOwned,
{
    fn load_toml(toml_string: impl AsRef<str>) -> Result<Self, FrameworkError> {
        Ok(toml::from_str(toml_string.as_ref())?)
    }

    fn load_toml_file(path: impl AsRef<AbsPath>) -> Result<Self, FrameworkError> {
        let mut file = File::open(path.as_ref()).map_err(|e| {
            let io_error = IoError.context(e);
            let path_error = PathError {
                name: Some(path.as_ref().as_path().into()),
            }
            .context(io_error);
            ConfigError.context(path_error)
        })?;

        let mut toml_string = String::new();
        file.read_to_string(&mut toml_string)?;
        Self::load_toml(toml_string)
    }
}
