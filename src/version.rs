use crate::config::FrumConfig;
use log::debug;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Version {
    Semver(semver::Version),
    System,
}

fn start_with_number(s: &str) -> bool {
    s.chars().next().map(|x| x.is_digit(10)).unwrap_or(false)
}

pub fn is_dotfile(dir: &std::fs::DirEntry) -> bool {
    dir.file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

impl Version {
    pub fn parse<S: AsRef<str>>(version_str: S) -> Result<Self, semver::SemVerError> {
        let lowercased = version_str.as_ref().to_lowercase();
        let trimed_lowercased = lowercased.trim_start_matches("ruby-");
        debug!("{}", trimed_lowercased);
        if lowercased == "system" {
            Ok(Self::System)
        } else if start_with_number(trimed_lowercased) {
            Ok(Self::Semver(semver::Version::parse(&trimed_lowercased)?))
        } else {
            unreachable!()
        }
    }

    pub fn installation_path(
        &self,
        config: &crate::config::FrumConfig,
    ) -> Option<std::path::PathBuf> {
        match self {
            v @ Self::Semver(_) => Some(config.versions_dir().join(v.to_string())),
            Self::System => None,
        }
    }
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("frum path doesn't exist")]
    EnvNotFound,
    #[error(transparent)]
    SemverError(#[from] semver::SemVerError),
}

pub fn current_version(config: &FrumConfig) -> Result<Option<Version>, Error> {
    debug!(
        "frum_path: {}",
        config.frum_path.clone().unwrap().to_str().unwrap()
    );
    let multishell_path = config.frum_path.as_ref().ok_or(Error::EnvNotFound)?;

    if let Ok(resolved_path) = std::fs::canonicalize(multishell_path) {
        debug!("frum_path: {}", resolved_path.to_str().unwrap());
        let file_name = resolved_path
            .file_name()
            .expect("Can't get filename")
            .to_str()
            .expect("Invalid OS string");
        let version = Version::parse(file_name).map_err(Error::SemverError)?;
        Ok(Some(version))
    } else {
        Ok(None)
    }
}

impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version_str = String::deserialize(deserializer)?;
        Version::parse(version_str).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Semver(semver) => write!(f, "{}", semver),
            Self::System => write!(f, "system"),
        }
    }
}

impl FromStr for Version {
    type Err = semver::SemVerError;
    fn from_str(s: &str) -> Result<Version, Self::Err> {
        Self::parse(s)
    }
}

impl PartialEq<semver::Version> for Version {
    fn eq(&self, other: &semver::Version) -> bool {
        match self {
            Self::Semver(v) => v == other,
            Self::System => false,
        }
    }
}
