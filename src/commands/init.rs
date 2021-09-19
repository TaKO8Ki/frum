use crate::shell::infer_shell;
use crate::shell::Shell;
use crate::symlink::create_symlink_dir;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrumError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Can't infer shell!")]
    CantInferShell,
}

pub struct Init {}

impl crate::command::Command for Init {
    type Error = FrumError;

    fn apply(&self, config: &crate::config::FrumConfig) -> Result<(), Self::Error> {
        let shell: Box<dyn Shell> = infer_shell().ok_or(FrumError::CantInferShell)?;
        let frum_path = create_symlink(config);
        let binary_path = if cfg!(windows) {
            frum_path.clone()
        } else {
            frum_path.join("bin")
        };
        println!("{}", shell.path(&binary_path));
        println!(
            "{}",
            shell.set_env_var("FRUM_MULTISHELL_PATH", frum_path.to_str().unwrap())
        );
        println!(
            "{}",
            shell.set_env_var("FRUM_DIR", config.base_dir().to_str().unwrap())
        );
        println!(
            "{}",
            shell.set_env_var("FRUM_LOGLEVEL", config.log_level.clone().into())
        );
        println!(
            "{}",
            shell.set_env_var("FRUM_RUBY_BUILD_MIRROR", config.ruby_build_mirror.as_str())
        );
        println!("{}", shell.use_on_cd(config));
        Ok(())
    }
}

fn create_symlink(config: &crate::config::FrumConfig) -> std::path::PathBuf {
    let system_temp_dir = std::env::temp_dir();
    let mut temp_dir = generate_symlink_path(&system_temp_dir);

    while temp_dir.exists() {
        temp_dir = generate_symlink_path(&system_temp_dir);
    }

    create_symlink_dir(config.default_version_dir(), &temp_dir).expect("Can't create symlink!");
    temp_dir
}

fn generate_symlink_path(root: &std::path::Path) -> std::path::PathBuf {
    let temp_dir_name = format!(
        "frum_{}_{}",
        std::process::id(),
        chrono::Utc::now().timestamp_millis(),
    );
    root.join(temp_dir_name)
}
