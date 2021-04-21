use crate::shell::Shell;
use indoc::indoc;
use std::path::Path;

#[derive(Debug)]
pub struct Fish;

impl Shell for Fish {
    fn path(&self, path: &Path) -> String {
        format!("set -gx PATH {:?} $PATH;", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("set -gx {name} {value:?};", name = name, value = value)
    }

    fn use_on_cd(&self, _config: &crate::config::FrumConfig) -> String {
        indoc!(
            r#"
                function _frum_autoload_hook --on-variable PWD --description 'Change Ruby version on directory change'
                    status --is-command-substitution; and return
                    frum local
                end
            "#
        )
        .into()
    }

    fn into_clap_shell(&self) -> clap::Shell {
        clap::Shell::Fish
    }
}
