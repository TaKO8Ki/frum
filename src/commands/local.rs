use crate::input_version::InputVersion;
use crate::symlink::{create_symlink_dir, remove_symlink_dir};
use crate::version::Version;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("We can't find the necessary environment variables to replace the Ruby version.")]
    FarmPathNotFound,
    #[error("Requested version {version:?} is not currently installed")]
    VersionNotFound { version: InputVersion },
}

pub struct Local {
    pub version: InputVersion,
}

impl crate::command::Command for Local {
    type Error = FarmError;

    fn apply(&self, config: &crate::config::FarmConfig) -> Result<(), FarmError> {
        debug!("Use {} as the current version", &self.version);
        let farm_path = config
            .farm_path
            .clone()
            .ok_or(FarmError::FarmPathNotFound)?;
        let version = match self.version.clone() {
            InputVersion::Full(Version::Semver(v)) => Version::Semver(v),
            _ => {
                return Err(FarmError::VersionNotFound {
                    version: self.version.clone(),
                })
            }
        };
        replace_symlink(&config.versions_dir().join(version.to_string()), &farm_path)
            .map_err(FarmError::IoError)?;
        Ok(())
    }
}

fn replace_symlink(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    let symlink_deletion_result = remove_symlink_dir(&to);
    match create_symlink_dir(&from, &to) {
        ok @ Ok(_) => ok,
        err @ Err(_) => symlink_deletion_result.and(err),
    }
}
