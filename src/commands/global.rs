use crate::alias::create_alias;
use crate::input_version::InputVersion;
use crate::version::Version;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrumError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Requested version {version} is not currently installed")]
    VersionNotFound { version: InputVersion },
}

pub struct Global {
    pub version: InputVersion,
}

impl crate::command::Command for Global {
    type Error = FrumError;

    fn apply(&self, config: &crate::config::FrumConfig) -> Result<(), Self::Error> {
        debug!("Use {} as the default version", &self.version);
        let version = match self.version.clone() {
            InputVersion::Full(Version::Semver(v)) => Version::Semver(v),
            version => return Err(FrumError::VersionNotFound { version }),
        };
        if !&config
            .versions_dir()
            .join(self.version.to_string())
            .exists()
        {
            return Err(FrumError::VersionNotFound {
                version: self.version.clone(),
            });
        }
        create_alias(config, "default", &version).map_err(FrumError::IoError)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Global;
    use crate::command::Command;
    use crate::config::FrumConfig;
    use crate::input_version::InputVersion;
    use crate::version::Version;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_global_specified_version_success() {
        let config = FrumConfig {
            base_dir: Some(tempdir().unwrap().path().to_path_buf()),
            frum_path: Some(std::env::temp_dir().join(format!(
                "frum_{}_{}",
                std::process::id(),
                chrono::Utc::now().timestamp_millis(),
            ))),
            ..Default::default()
        };
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

    #[test]
    fn test_global_specified_version_failure() {
        let config = FrumConfig {
            base_dir: Some(tempdir().unwrap().path().to_path_buf()),
            frum_path: Some(std::env::temp_dir().join(format!(
                "frum_{}_{}",
                std::process::id(),
                chrono::Utc::now().timestamp_millis(),
            ))),
            ..Default::default()
        };
        let dir_path = config.versions_dir().join("2.6.4").join("bin");
        std::fs::create_dir_all(&dir_path).unwrap();
        File::create(dir_path.join("ruby")).unwrap();

        let result = Global {
            version: InputVersion::Full(Version::Semver(semver::Version::parse("2.7.0").unwrap())),
        }
        .apply(&config);
        match result {
            Ok(()) => assert!(false, "global must return an error"),
            Err(e) => assert_eq!(
                e.to_string(),
                "Requested version 2.7.0 is not currently installed"
            ),
        }

        assert!(!config
            .default_version_dir()
            .join("bin")
            .join("ruby")
            .exists());
    }
}
