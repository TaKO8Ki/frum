use crate::shell::infer_shell;
use crate::shell::Shell;
use crate::symlink::create_symlink_dir;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(transparent)]
    HttpError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Can't infer shell!")]
    CantInferShell,
}

pub struct Init {}

impl crate::command::Command for Init {
    type Error = FarmError;

    fn apply(&self, config: &crate::config::FarmConfig) -> Result<(), Self::Error> {
        let shell: Box<dyn Shell> = infer_shell().ok_or(FarmError::CantInferShell)?;
        let farm_path = create_symlink(&config);
        let binary_path = if cfg!(windows) {
            farm_path.clone()
        } else {
            farm_path.join("bin")
        };
        println!("{}", shell.path(&binary_path));
        println!(
            "{}",
            shell.set_env_var("FARM_MULTISHELL_PATH", farm_path.to_str().unwrap())
        );
        println!(
            "{}",
            shell.set_env_var("FARM_DIR", config.base_dir().to_str().unwrap())
        );
        println!(
            "{}",
            shell.set_env_var("FARM_LOGLEVEL", config.log_level.clone().into())
        );
        println!(
            "{}",
            shell.set_env_var("FARM_RUBY_BUILD_MIRROR", config.ruby_build_mirror.as_str())
        );
        println!("{}", shell.use_on_cd(&config));
        Ok(())
    }
}

fn create_symlink(config: &crate::config::FarmConfig) -> std::path::PathBuf {
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
        "farm_{}_{}",
        std::process::id(),
        chrono::Utc::now().timestamp_millis(),
    );
    root.join(temp_dir_name)
}
