use crate::version::Version;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum InputVersion {
    Major(u64),
    MajorMinor(u64, u64),
    Full(Version),
}

impl InputVersion {
    pub fn to_version<'a, T>(&self, available_versions: T) -> Option<&'a Version>
    where
        T: IntoIterator<Item = &'a Version>,
    {
        available_versions
            .into_iter()
            .filter(|x| self.matches(x))
            .max()
    }

    pub fn matches(&self, version: &Version) -> bool {
        match (self, version) {
            (Self::Full(a), b) => a == b,
            (_, Version::System) => false,
            (Self::Major(major), Version::Semver(other)) => *major == other.major,
            (Self::MajorMinor(major, minor), Version::Semver(other)) => {
                *major == other.major && *minor == other.minor
            }
        }
    }
}

impl std::fmt::Display for InputVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full(x) => x.fmt(f),
            Self::Major(major) => write!(f, "{}.x.x", major),
            Self::MajorMinor(major, minor) => write!(f, "{}.{}.x", major, minor),
        }
    }
}

impl FromStr for InputVersion {
    type Err = semver::SemVerError;
    fn from_str(s: &str) -> Result<InputVersion, Self::Err> {
        match Version::parse(s) {
            Ok(v) => Ok(Self::Full(v)),
            Err(e) => {
                let mut parts = s.trim().split('.');
                match (next_of::<u64, _>(&mut parts), next_of::<u64, _>(&mut parts)) {
                    (Some(major), None) => Ok(Self::Major(major)),
                    (Some(major), Some(minor)) => Ok(Self::MajorMinor(major, minor)),
                    _ => Err(e),
                }
            }
        }
    }
}

fn next_of<'a, T: FromStr, It: Iterator<Item = &'a str>>(i: &mut It) -> Option<T> {
    let x = i.next()?;
    T::from_str(x).ok()
}
