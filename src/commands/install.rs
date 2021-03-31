use crate::alias::create_alias;
use crate::archive::tar_xz::{self, FarmError as ExtractError};
use crate::config::FarmConfig;
use crate::input_version::InputVersion;
use crate::outln;
use crate::version::Version;
use anyhow::Result;
use log::debug;
use reqwest::Url;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Can't find the number of cores")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error("Can't extract the file: {source:?}")]
    ExtractError { source: ExtractError },
    #[error("The downloaded archive is empty")]
    TarIsEmpty,
    #[error("Can't find version: {version}")]
    VersionNotFound { version: InputVersion },
    #[error("Can't list the remote versions: {source:?}")]
    CantListRemoteVersions { source: reqwest::Error },
    #[error("Version already installed at {path:?}")]
    VersionAlreadyInstalled { path: PathBuf },
}

pub struct Install {
    pub version: InputVersion,
}

impl crate::command::Command for Install {
    type Error = FarmError;

    fn apply(&self, config: &FarmConfig) -> Result<(), FarmError> {
        let current_version = self.version.clone();
        let version = match current_version {
            InputVersion::Full(Version::Semver(v)) => Version::Semver(v),
            current_version => {
                let available_versions = crate::remote_ruby_index::list(&config.ruby_build_mirror)
                    .map_err(|source| FarmError::CantListRemoteVersions { source })?
                    .drain(..)
                    .map(|x| x.version)
                    .collect::<Vec<_>>();

                current_version
                    .to_version(&available_versions)
                    .ok_or(FarmError::VersionNotFound {
                        version: current_version,
                    })?
                    .clone()
            }
        };

        outln!(config#Info, "Installing Ruby {}...", self.version);
        let response =
            reqwest::blocking::get(package_url(config.ruby_build_mirror.clone(), &version))?;
        if response.status() == 404 {
            return Err(FarmError::VersionNotFound {
                version: self.version.clone(),
            });
        }
        let installations_dir = config.versions_dir();
        let installation_dir = PathBuf::from(&installations_dir).join(version.to_string());

        if installation_dir.exists() {
            return Err(FarmError::VersionAlreadyInstalled {
                path: installation_dir,
            });
        }

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
        build_package(&installed_directory, &installation_dir)?;

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
    debug!("pakage url");
    Url::parse(&format!(
        "{}/ruby-{}.tar.xz",
        mirror_url.as_str().trim_end_matches('/'),
        version,
    ))
    .unwrap()
}

fn build_package(current_dir: &Path, installed_dir: &Path) -> Result<(), FarmError> {
    Command::new("sh")
        .arg("configure")
        .arg(format!("--prefix={}", installed_dir.to_str().unwrap()))
        .current_dir(&current_dir)
        .output()
        .map_err(FarmError::IoError)?;
    debug!("make -j {}", number_of_cores().unwrap_or(2).to_string());
    Command::new("make")
        .arg("-j")
        .arg(number_of_cores().unwrap_or(2).to_string())
        .current_dir(&current_dir)
        .output()
        .map_err(FarmError::IoError)?;
    debug!("make install");
    Command::new("make")
        .arg("install")
        .current_dir(&current_dir)
        .output()
        .map_err(FarmError::IoError)?;
    Ok(())
}

fn number_of_cores() -> Result<u8, FarmError> {
    let mut reader = BufReader::new(
        Command::new("uname")
            .arg("-s")
            .stdout(std::process::Stdio::piped())
            .spawn()
            .map_err(FarmError::IoError)?
            .stdout
            .unwrap(),
    );
    let mut uname = String::new();
    reader.read_line(&mut uname).map_err(FarmError::IoError)?;

    let output = match uname.as_str().trim() {
        "Darwin" => {
            Command::new("sysctl")
                .arg("-n")
                .arg("hw.ncpu")
                .output()
                .map_err(FarmError::IoError)?
                .stdout
        }
        "SunOS" => {
            Command::new("getconf")
                .arg("NPROCESSORS_ONLN")
                .output()
                .map_err(FarmError::IoError)?
                .stdout
        }
        _ => {
            let output = Command::new("getconf")
                .arg("_NPROCESSORS_ONLN")
                .output()
                .map_err(FarmError::IoError)?
                .stdout;
            if String::from_utf8(output.clone())?
                .trim()
                .parse::<u8>()
                .is_ok()
            {
                output
            } else {
                Command::new("grep")
                    .arg("-c")
                    .arg("^processor")
                    .arg("/proc/cpuinfo")
                    .output()
                    .map_err(FarmError::IoError)?
                    .stdout
            }
        }
    };

    Ok(String::from_utf8(output)?
        .trim()
        .parse()
        .expect("can't convert cores to integer"))
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
        let config = FarmConfig::default();

        Install {
            version: InputVersion::Full(Version::Semver(semver::Version::parse("2.6.4").unwrap())),
        }
        .apply(&config)
        .expect("Can't install");
    }

    #[test]
    fn test_number_of_cores() {
        number_of_cores().unwrap();
    }
}
