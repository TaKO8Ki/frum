use crate::archive::tar_xz::{self, FarmError as ExtractError};
use crate::config::FarmConfig;
use anyhow::Result;
use log::debug;
use reqwest::Url;
use std::path::Path;
use std::process::Command;
use thiserror::Error;
pub struct Install {
    pub version: String,
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
    VersionNotFound { version: String },
}

impl crate::command::Command for Install {
    type Error = FarmError;

    fn apply(&self, config: &FarmConfig) -> Result<(), FarmError> {
        let response = reqwest::blocking::get(package_url(
            config.ruby_build_default_mirror.clone(),
            self.version.clone(),
        ))?;
        if response.status() == 404 {
            return Err(FarmError::VersionNotFound {
                version: self.version.clone(),
            });
        }
        let installations_dir = config.installation_dir();
        std::fs::create_dir_all(&installations_dir).map_err(FarmError::IoError)?;
        let installation_dir =
            std::path::PathBuf::from(&installations_dir).join(self.version.clone());
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
        Command::new("sh")
            .arg("configure")
            .arg("--disable-install-doc")
            .current_dir(&installed_directory)
            .output()
            .expect("./configure failed to start");
        debug!("make -j 2");
        Command::new("make")
            .arg("-j")
            .arg("5")
            .current_dir(&installed_directory)
            .output()
            .expect("make failed to start");
        let renamed_installation_dir = temp_dir.path().join("installation");
        std::fs::rename(&installed_directory, &renamed_installation_dir)
            .map_err(FarmError::IoError)?;
        std::fs::rename(&temp_dir, &installation_dir)?;
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

fn package_url(mirror_url: Url, version: String) -> Url {
    mirror_url
        .join(format!("ruby-{}.tar.xz", version.as_str()).as_str())
        .expect("invalid mirror url")
}
