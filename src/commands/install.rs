use crate::alias::create_alias;
use crate::archive::tar_xz::{self, FarmError as ExtractError};
use crate::config::FarmConfig;
use crate::input_version::InputVersion;
use crate::outln;
use crate::version::Version;
use anyhow::Result;
use log::debug;
use reqwest::Url;
use std::path::{Path, PathBuf};
use std::process::Command;
use thiserror::Error;
pub struct Install {
    pub version: InputVersion,
}

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Can't extract the file: {source:?}")]
    ExtractError { source: ExtractError },
    #[error("The downloaded archive is empty")]
    TarIsEmpty,
    #[error("Can't find version: {version}")]
    VersionNotFound { version: Version },
}

impl crate::command::Command for Install {
    type Error = FarmError;

    fn apply(&self, config: &FarmConfig) -> Result<(), FarmError> {
        let current_version = self.version.clone();
        let version = match current_version.clone() {
            InputVersion::Full(Version::Semver(v)) => Version::Semver(v),
            _ => Version::parse("2.6.4").unwrap(),
        };

        outln!(config#Info, "Installing Ruby {}...", self.version);
        let response =
            reqwest::blocking::get(package_url(config.ruby_build_mirror.clone(), &version))?;
        if response.status() == 404 {
            return Err(FarmError::VersionNotFound {
                version: version.clone(),
            });
        }
        let installations_dir = config.versions_dir();
        std::fs::create_dir_all(&installations_dir).map_err(FarmError::IoError)?;
        let installation_dir =
            std::path::PathBuf::from(&installations_dir).join(version.to_string());
        let temp_installations_dir = installations_dir.join(".downloads");
        std::fs::create_dir_all(&temp_installations_dir).map_err(FarmError::IoError)?;
        let temp_dir = tempfile::TempDir::new_in(&temp_installations_dir)
            .expect("Can't generate a temp directory");
        extract_archive_into(&temp_dir, response)?;
        let installed_directory = std::fs::read_dir(&temp_dir)
            .map_err(FarmError::IoError)?
            .next()
            .ok_or(FarmError::TarIsEmpty)?
            .map_err(FarmError::IoError)?;
        let installed_directory = installed_directory.path();
        debug!("./configure ruby-{}", self.version);
        build_package(&installed_directory, &installation_dir);

        if !config.default_version_dir().exists() {
            debug!("Use {} as the default version", self.version);
            create_alias(&config, "default", &version).map_err(FarmError::IoError)?;
        }
        Ok(())
    }
}

fn extract_archive_into<P: AsRef<Path>>(
    path: P,
    response: reqwest::blocking::Response,
) -> Result<(), FarmError> {
    let extractor = tar_xz::TarXz::new(response);
    extractor
        .extract_into(path)
        .map_err(|source| FarmError::ExtractError { source })?;
    Ok(())
}

fn package_url(mirror_url: Url, version: &Version) -> Url {
    mirror_url
        .join(format!("ruby-{}.tar.xz", version).as_str())
        .expect("invalid mirror url")
}

fn build_package(current_dir: &PathBuf, installed_dir: &PathBuf) {
    Command::new("sh")
        .arg("configure")
        .arg(format!("--prefix={}", installed_dir.to_str().unwrap()))
        .current_dir(&current_dir)
        .output()
        .expect("./configure failed to start");
    debug!("make");
    Command::new("make")
        .arg("-j")
        .arg("5")
        .current_dir(&current_dir)
        .output()
        .expect("make failed to start");
    debug!("make install");
    Command::new("make")
        .arg("install")
        .current_dir(&current_dir)
        .output()
        .expect("make install failed to start");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::config::FarmConfig;
    use crate::version::Version;

    #[test]
    #[ignore]
    fn test_set_default_on_new_installation() {
        let base_dir = tempfile::tempdir().unwrap();
        let config = FarmConfig::default();

        Install {
            version: InputVersion::Full(Version::Semver(semver::Version::parse("2.6.4").unwrap())),
        }
        .apply(&config)
        .expect("Can't install");
    }
}
