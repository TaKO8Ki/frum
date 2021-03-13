pub struct FarmConfg {
    pub ruby_build_default_mirror: reqwest::Url,
}

impl Default for FarmConfg {
    fn default() -> Self {
        Self {
            ruby_build_default_mirror: reqwest::Url::parse("https://cache.ruby-lang.org/pub/ruby")
                .unwrap(),
        }
    }
}

impl FarmConfg {
    pub fn base_dir(&self) -> std::path::PathBuf {
        // TODO: support base directory
        ensure_dir_exists(
            std::env::current_dir()
                .expect("Can't get current directory")
                .join(".farm"),
        )
    }

    pub fn installation_dir(&self) -> std::path::PathBuf {
        ensure_dir_exists(self.base_dir().join("versions"))
    }
}

fn ensure_dir_exists<T: AsRef<std::path::Path>>(path: T) -> T {
    std::fs::create_dir_all(path.as_ref()).ok();
    path
}
