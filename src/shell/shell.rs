use std::fmt::Debug;
use std::path::PathBuf;
pub trait Shell: Debug {
    fn path(&self, path: &PathBuf) -> String;
    fn set_env_var(&self, name: &str, value: &str) -> String;
}
