use crate::archive::tar_xz::{self, FarmError as ExtractError};
use anyhow::Result;
use std::env;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Can't extract the file: {source:?}")]
    ExtractError { source: ExtractError },
    #[error("Can't find version upstream")]
    VersionNotFound,
}

pub fn install(version: String) -> Result<(), FarmError> {
    let package_url = format!(
        "{}/ruby-{}.tar.xz",
        crate::RUBY_BUILD_DEFAULT_MIRROR,
        version
    );
    let response = reqwest::blocking::get(&package_url)?;
    if response.status() == 404 {
        return Err(FarmError::VersionNotFound);
    }
    extract_archive_into(env::current_dir()?, response)?;
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
