use crate::config::FarmConfig;
use crate::input_version::InputVersion;
use crate::outln;
use crate::symlink::remove_symlink_dir;
use crate::version::Version;
use anyhow::Result;
use colored::Colorize;
use log::debug;
use std::ffi::OsStr;
use std::io::prelude::*;
use std::path::Component;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Can't find the number of cores")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Can't find version: {version}")]
    VersionNotFound { version: InputVersion },
    #[error("The requested version is not installable: {version}")]
    NotInstallableVersion { version: Version },
    #[error("We can't find the necessary environment variables to replace the Ruby version.")]
    FarmPathNotFound,
}

pub struct Uninstall {
    pub version: InputVersion,
}

impl crate::command::Command for Uninstall {
    type Error = FarmError;

    fn apply(&self, config: &FarmConfig) -> Result<(), FarmError> {
        let current_version = self.version.clone();
        let version = match current_version.clone() {
            InputVersion::Full(Version::Semver(v)) => Version::Semver(v),
            InputVersion::Full(Version::System) => {
                return Err(FarmError::NotInstallableVersion {
                    version: Version::System,
                })
            }
            _ => unreachable!(),
        };
        let installation_dir = PathBuf::from(&config.versions_dir()).join(version.to_string());
        if !installation_dir.exists() {
            return Err(FarmError::VersionNotFound {
                version: current_version,
            });
        }
        outln!(config#Info, "{} Uninstalling {}", "==>".green(), format!("Ruby {}", current_version).green());
        if symlink_exists(
            config
                .farm_path
                .clone()
                .ok_or(FarmError::FarmPathNotFound)?,
            &version,
        )? {
            debug!("remove farm path symlink");
            remove_symlink_dir(
                &config
                    .farm_path
                    .clone()
                    .ok_or(FarmError::FarmPathNotFound)?,
            )?;
        }
        if symlink_exists(config.default_version_dir(), &version)? {
            debug!("remove default alias symlink");
            remove_symlink_dir(&config.default_version_dir())?;
        }
        debug!("remove dir");
        std::fs::remove_dir_all(&installation_dir)?;
        Ok(())
    }
}

fn symlink_exists(to: PathBuf, version: &Version) -> Result<bool, FarmError> {
    Ok(std::fs::read_link(to)?.components().last()
        == Some(Component::Normal(OsStr::new(&version.to_string()))))
}
