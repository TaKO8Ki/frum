use crate::input_version::InputVersion;
use crate::symlink::{create_symlink_dir, remove_symlink_dir};
use crate::version_file::get_user_version_for_directory;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrumError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("We can't find the necessary environment variables to replace the Ruby version.")]
    FrumPathNotFound,
    #[error("Requested version {version} is not currently installed")]
    VersionNotFound { version: InputVersion },
    #[error("Can't find version in dotfiles. Please provide a version manually to the command.")]
    CantInferVersion,
}

pub struct Local {
    pub version: Option<InputVersion>,
    pub quiet: bool
}

impl crate::command::Command for Local {
    type Error = FrumError;

    fn apply(&self, config: &crate::config::FrumConfig) -> Result<(), Self::Error> {
        let current_version = match self.version.clone().ok_or_else(|| {
            match get_user_version_for_directory(std::env::current_dir().unwrap()) {
                Some(version) => Ok(version),
                None => {
                    replace_symlink(
                        &config.default_version_dir(),
                        &config
                        .frum_path
                        .clone()
                        .ok_or(FrumError::FrumPathNotFound)?,
                        )?;

                    Err(FrumError::CantInferVersion)
                }
            }
        }) {
            Ok(version) => version,
            Err(result) => {
                if self.quiet {
                    return Ok(())
                }
                result?
            },
        };
        debug!("Use {} as the current version", current_version);
        if !&config
            .versions_dir()
                .join(current_version.to_string())
                .exists()
                {
                    return Err(FrumError::VersionNotFound {
                        version: current_version,
                    });
                }
        replace_symlink(
            &config.versions_dir().join(current_version.to_string()),
            &config
            .frum_path
            .clone()
            .ok_or(FrumError::FrumPathNotFound)?,
            )
            .map_err(FrumError::IoError)?;
        Ok(())
    }
}

fn replace_symlink(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    let symlink_deletion_result = remove_symlink_dir(&to);
    match create_symlink_dir(&from, &to) {
        ok @ Ok(_) => ok,
        err @ Err(_) => symlink_deletion_result.and(err),
    }
}

#[cfg(test)]
mod tests {
    use super::{FrumError, Local};
    use crate::command::Command;
    use crate::config::FrumConfig;
    use crate::input_version::InputVersion;
    use crate::version::Version;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_local_specified_version() {
        let mut config = FrumConfig::default();
        config.base_dir = Some(tempdir().unwrap().path().to_path_buf());
        config.frum_path = Some(std::env::temp_dir().join(format!(
                    "frum_{}_{}",
                    std::process::id(),
                    chrono::Utc::now().timestamp_millis(),
                    )));
        let dir_path = config.versions_dir().join("2.6.4").join("bin");
        std::fs::create_dir_all(&dir_path).unwrap();
        File::create(dir_path.join("ruby")).unwrap();

        crate::commands::global::Global {
            version: InputVersion::Full(Version::Semver(semver::Version::parse("2.6.4").unwrap())),
        }
        .apply(&config)
            .unwrap();

        Local {
            version: Some(InputVersion::Full(Version::Semver(
                                 semver::Version::parse("2.6.4").unwrap(),
                                 ))),
                                 quiet: false,
        }
        .apply(&config)
            .expect("failed to install");

        assert!(config.frum_path.unwrap().join("bin").join("ruby").exists());
    }

    #[test]
    fn test_not_found_version() {
        let mut config = FrumConfig::default();
        config.base_dir = Some(tempdir().unwrap().path().to_path_buf());
        config.frum_path = Some(std::env::temp_dir().join(format!(
                    "frum_{}_{}",
                    std::process::id(),
                    chrono::Utc::now().timestamp_millis(),
                    )));
        let result = Local {
            version: Some(InputVersion::Full(Version::Semver(
                                 semver::Version::parse("2.6.4").unwrap(),
                                 ))),
                                 quiet: false,
        }
        .apply(&config);
        match result {
            Ok(_) => assert!(false),
            Err(FrumError::VersionNotFound { .. }) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_not_found_version_file() {
        let mut config = FrumConfig::default();
        config.base_dir = Some(tempdir().unwrap().path().to_path_buf());
        config.frum_path = Some(std::env::temp_dir().join(format!(
                    "frum_{}_{}",
                    std::process::id(),
                    chrono::Utc::now().timestamp_millis(),
                    )));
        let result = Local {
            version: None,
            quiet: false,
        }
        .apply(&config);
        match result {
            Ok(_) => assert!(false),
            Err(FrumError::CantInferVersion { .. }) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_not_found_version_file_with_quiet() {
        let mut config = FrumConfig::default();
        config.base_dir = Some(tempdir().unwrap().path().to_path_buf());
        config.frum_path = Some(std::env::temp_dir().join(format!(
                    "frum_{}_{}",
                    std::process::id(),
                    chrono::Utc::now().timestamp_millis(),
                    )));
        let result = Local {
            version: None,
            quiet: true,
        }
        .apply(&config);
        match result {
            Ok(()) => assert!(true),
            _ => assert!(false),
        }
    }
}
