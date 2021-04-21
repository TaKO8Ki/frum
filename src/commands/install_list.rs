use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrumError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub struct InstallList {}

impl crate::command::Command for InstallList {
    type Error = FrumError;

    fn apply(&self, config: &crate::config::FrumConfig) -> Result<(), FrumError> {
        let versions = crate::remote_ruby_index::list(&config.ruby_build_mirror)?;
        for version in versions {
            crate::outln!(config#Info, "{}", version.version);
        }
        Ok(())
    }
}
