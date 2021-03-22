use crate::alias::create_alias;
use crate::input_version::InputVersion;
use crate::version::Version;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub struct Global {
    pub version: InputVersion,
}

impl crate::command::Command for Global {
    type Error = FarmError;

    fn apply(&self, config: &crate::config::FarmConfig) -> Result<(), FarmError> {
        debug!("Use {} as the default version", &self.version);
        let version = match self.version.clone() {
            InputVersion::Full(Version::Semver(v)) => Version::Semver(v),
            _ => return Ok(()),
        };
        create_alias(&config, "default", &version).map_err(FarmError::IoError)?;
        Ok(())
    }
}
