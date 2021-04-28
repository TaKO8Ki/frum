use crate::shell::Shell;
use std::path::Path;

#[derive(Debug)]
pub struct Bash;

impl Shell for Bash {
    fn path(&self, path: &Path) -> String {
        format!("export PATH={:?}:$PATH", path.to_str().unwrap())
    }

    fn set_env_var(&self, name: &str, value: &str) -> String {
        format!("export {}={:?}", name, value)
    }

    fn use_on_cd(&self, _config: &crate::config::FrumConfig) -> String {
        indoc::indoc!(
            r#"
                __frumcd() {
                    \cd "$@" || return $?
                    frum --log-level quiet local
                }

                alias cd=__frumcd
            "#
        )
        .into()
    }

    fn as_clap_shell(&self) -> clap::Shell {
        clap::Shell::Bash
    }
}
