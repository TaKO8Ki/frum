use crate::config::FarmConfig;
use crate::outln;
use crate::version::{current_version, Version};
use colored::Colorize;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SemverError(#[from] semver::SemVerError),
}

pub struct Versions {}

impl crate::command::Command for Versions {
    type Error = FarmError;

    fn apply(&self, config: &FarmConfig) -> Result<(), Self::Error> {
        for entry in config
            .versions_dir()
            .read_dir()
            .map_err(FarmError::IoError)?
        {
            let entry = entry.map_err(FarmError::IoError)?;
            if is_dotfile(&entry) {
                continue;
            }

            let path = entry.path();
            let filename = path
                .file_name()
                .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
                .map_err(FarmError::IoError)?
                .to_str()
                .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
                .map_err(FarmError::IoError)?;
            let version = Version::parse(filename).map_err(FarmError::SemverError)?;
            let current_version = current_version(&config).ok().flatten();
            debug!("current version: {}", current_version.clone().unwrap());
            if let Some(current_version) = current_version {
                if current_version == version {
                    outln!(config#Info, "{} {}", "*".green(), version.to_string().green());
                } else {
                    outln!(config#Info, "{} {}", " ", version);
                }
            } else {
                outln!(config#Info, "{} {}", " ", version);
            };
        }
        Ok(())
    }
}

fn is_dotfile(dir: &std::fs::DirEntry) -> bool {
    dir.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
