use crate::version::Version;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum InputVersion {
    Major(u64),
    MajorMinor(u64, u64),
    Full(Version),
}

impl std::fmt::Display for InputVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full(x) => x.fmt(f),
            Self::Major(major) => write!(f, "v{}.x.x", major),
            Self::MajorMinor(major, minor) => write!(f, "v{}.{}.x", major, minor),
        }
    }
}

impl FromStr for InputVersion {
    type Err = semver::SemVerError;
    fn from_str(s: &str) -> Result<InputVersion, Self::Err> {
        match Version::parse(s) {
            Ok(v) => Ok(Self::Full(v)),
            Err(e) => Err(e),
        }
    }
}
