pub mod bash;
pub mod fish;
pub mod infer;
pub mod zsh;

use std::fmt::Debug;
use std::path::Path;

pub use bash::Bash;
pub use fish::Fish;
pub use zsh::Zsh;
pub trait Shell: Debug {
    fn path(&self, path: &Path) -> String;
    fn set_env_var(&self, name: &str, value: &str) -> String;
    fn use_on_cd(&self, config: &crate::config::FarmConfig) -> String;
}

pub fn infer_shell() -> Option<Box<dyn Shell>> {
    infer::unix::infer_shell()
}
