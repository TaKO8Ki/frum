pub mod zsh;

use std::fmt::Debug;
use std::path::Path;
pub trait Shell: Debug {
    fn path(&self, path: &Path) -> String;
    fn set_env_var(&self, name: &str, value: &str) -> String;
}
