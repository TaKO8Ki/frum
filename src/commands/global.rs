use crate::alias::create_alias;
use crate::input_version::InputVersion;
use crate::version::Version;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub struct Global {
    pub version: InputVersion,
}

impl crate::command::Command for Global {
    type Error = FarmError;

    fn apply(&self, config: &crate::config::FarmConfig) -> Result<(), FarmError> {
        debug!("Use {} as the default version", &self.version);
        let version = match self.version.clone() {
            InputVersion::Full(Version::Semver(v)) => Version::Semver(v),
            _ => return Ok(()),
        };
        create_alias(&config, "default", &version).map_err(FarmError::IoError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Global;
    use crate::command::Command;
    use crate::config::FarmConfig;
    use crate::input_version::InputVersion;
    use crate::version::Version;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_global_specified_version() {
        let mut config = FarmConfig::default();
        config.base_dir = Some(tempdir().unwrap().path().to_path_buf());
        config.farm_path = Some(std::env::temp_dir().join(format!(
            "farm_{}_{}",
            std::process::id(),
            chrono::Utc::now().timestamp_millis(),
        )));
        let dir_path = config.versions_dir().join("2.6.4").join("bin");
        std::fs::create_dir_all(&dir_path).unwrap();
        File::create(dir_path.join("ruby")).unwrap();

        Global {
            version: InputVersion::Full(Version::Semver(semver::Version::parse("2.6.4").unwrap())),
        }
        .apply(&config)
        .expect("failed to install");

        assert!(config
            .default_version_dir()
            .join("bin")
            .join("ruby")
            .exists());
    }
}
