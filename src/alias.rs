use crate::config::FarmConfig;
use crate::symlink::{create_symlink_dir, remove_symlink_dir};
use crate::version::Version;
use std::path::PathBuf;

pub fn create_alias(
    config: &FarmConfig,
    common_name: &str,
    version: &Version,
) -> std::io::Result<()> {
    let aliases_dir = config.aliases_dir();
    std::fs::create_dir_all(&aliases_dir)?;

    let version_dir = version
        .installation_path(config)
        .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))?;
    let alias_dir = aliases_dir.join(common_name);

    if alias_dir.exists() {
        remove_symlink_dir(&alias_dir)?;
    }

    create_symlink_dir(&version_dir, &alias_dir)?;

    Ok(())
}

#[derive(Debug)]
pub struct StoredAlias {
    alias_path: PathBuf,
    destination_path: PathBuf,
}

impl std::convert::TryInto<StoredAlias> for &std::path::Path {
    type Error = std::io::Error;

    fn try_into(self) -> Result<StoredAlias, Self::Error> {
        let destination_path = std::fs::canonicalize(&self)?;
        Ok(StoredAlias {
            alias_path: PathBuf::from(self),
            destination_path,
        })
    }
}

impl StoredAlias {
    pub fn s_ver(&self) -> &str {
        self.destination_path
            .parent()
            .unwrap()
            .file_name()
            .expect("must have basename")
            .to_str()
            .unwrap()
    }

    pub fn name(&self) -> &str {
        self.alias_path
            .file_name()
            .expect("must have basename")
            .to_str()
            .unwrap()
    }

    pub fn path(&self) -> &std::path::Path {
        &self.alias_path
    }
}
