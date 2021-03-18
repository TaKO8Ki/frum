use thiserror::Error;

pub struct IndexedRubyVersion {
    pub version: String,
    pub url: String,
    pub sha1: String,
    pub sha256: String,
    pub sha521: String,
}

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub struct InstallList {}

impl crate::command::Command for InstallList {
    type Error = FarmError;

    fn apply(&self, config: &crate::config::FarmConfig) -> Result<(), FarmError> {
        let value = reqwest::blocking::get(
            config
                .ruby_build_default_mirror
                .join("index.txt")
                .expect("invalid index url"),
        )?
        .text()?;
        let re = regex::Regex::new(r"(\S+)\s+(\S+)\s+(\S+)\s+(\S+)\s+(\S+)").unwrap();
        let versions = value
            .split('\n')
            .map(|n| {
                if n.is_empty() {
                    return None;
                }
                let cap = re.captures(n).unwrap();
                Some(IndexedRubyVersion {
                    version: cap
                        .get(1)
                        .map_or("".to_string(), |m| m.as_str().to_string()),
                    url: cap
                        .get(2)
                        .map_or("".to_string(), |m| m.as_str().to_string()),
                    sha1: cap
                        .get(3)
                        .map_or("".to_string(), |m| m.as_str().to_string()),
                    sha256: cap
                        .get(4)
                        .map_or("".to_string(), |m| m.as_str().to_string()),
                    sha521: cap
                        .get(5)
                        .map_or("".to_string(), |m| m.as_str().to_string()),
                })
            })
            .collect::<Vec<Option<IndexedRubyVersion>>>();
        for version in versions {
            if let Some(version) = version {
                println!("{}", version.version)
            }
        }
        Ok(())
    }
}
