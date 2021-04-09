use crate::input_version::InputVersion;
use encoding_rs_io::DecodeReaderBytes;
use log::{debug, info};
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

const VERSION_FILE: &str = ".ruby-version";

pub fn get_user_version_for_directory(path: PathBuf) -> Option<InputVersion> {
    let version_file_path = find_up(path, VERSION_FILE)?;
    info!(
        "Looking for version file in {}. exists? {}",
        version_file_path.display(),
        version_file_path.exists()
    );
    if let Some(version) = get_user_version_for_file(version_file_path) {
        return Some(version);
    }

    None
}

pub fn get_user_version_for_file(path: PathBuf) -> Option<InputVersion> {
    let file = std::fs::File::open(path).ok()?;
    let version = {
        let mut reader = DecodeReaderBytes::new(file);
        let mut version = String::new();
        reader.read_to_string(&mut version).map(|_| version)
    };

    match version {
        Err(err) => {
            info!("Can't read file: {}", err);
            None
        }
        Ok(version) => {
            info!("Found string {:?}  in version file", version);
            InputVersion::from_str(version.trim()).ok()
        }
    }
}

pub fn find_up(search_dir: PathBuf, file_name: &str) -> Option<PathBuf> {
    for dir in each_dir(search_dir) {
        let path = dir.join(&file_name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

fn each_dir(path: PathBuf) -> Vec<PathBuf> {
    let mut path = std::fs::canonicalize(path).unwrap();
    let mut paths = vec![path.clone()];

    while let Some(parent) = path.clone().parent() {
        path = parent.to_path_buf();
        debug!("get parent of {:?}...", parent);
        paths.push(parent.to_path_buf())
    }
    paths.push(PathBuf::from("/"));

    paths
}
