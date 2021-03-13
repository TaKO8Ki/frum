use crate::archive::tar_xz::{self, FarmError as ExtractError};
use anyhow::Result;
use log::debug;
use std::path::Path;
use std::process::Command;
use thiserror::Error;

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

pub fn install(version: String, config: crate::config::FarmConfg) -> Result<(), FarmError> {
    let package_url = format!(
        "{}/ruby-{}.tar.xz",
        config.ruby_build_default_mirror, version
    );
    let response = reqwest::blocking::get(&package_url)?;
    if response.status() == 404 {
        return Err(FarmError::VersionNotFound { version });
    }
    let installations_dir = config.installation_dir();
    std::fs::create_dir_all(&installations_dir).map_err(FarmError::IoError)?;
    let installation_dir = std::path::PathBuf::from(&installations_dir).join(version.clone());
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
    debug!("./configure ruby-{}", version);
    Command::new("sh")
        .arg("configure")
        .arg("--disable-install-doc")
        .current_dir(&installed_directory)
        .output()
        .expect("./configure failed to start");
    debug!("make -j 2");
    Command::new("make")
        .arg("-j")
        .arg("2")
        .current_dir(&installed_directory)
        .output()
        .expect("make failed to start");
    let renamed_installation_dir = temp_dir.path().join("installation");
    std::fs::rename(&installed_directory, &renamed_installation_dir).map_err(FarmError::IoError)?;
    std::fs::rename(&temp_dir, &installation_dir)?;
    Ok(())
}

pub fn extract_archive_into<P: AsRef<Path>>(
    path: P,
    response: reqwest::blocking::Response,
) -> Result<(), FarmError> {
    let extractor = tar_xz::TarXz::new(response);
    extractor
        .extract_into(path)
        .map_err(|source| FarmError::ExtractError { source })?;
    Ok(())
}
