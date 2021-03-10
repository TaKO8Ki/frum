use reqwest::blocking::Response;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
}

pub struct TarXz {
    response: Response,
}

impl TarXz {
    #[allow(dead_code)]
    pub fn new(response: Response) -> Self {
        Self { response }
    }
}

impl TarXz {
    pub fn extract_into<P: AsRef<Path>>(self, path: P) -> Result<(), FarmError> {
        let xz_stream = xz2::read::XzDecoder::new(self.response);
        let mut tar_archive = tar::Archive::new(xz_stream);
        tar_archive.unpack(&path)?;
        Ok(())
    }
}
