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

    fn use_on_cd(&self, _config: &crate::config::FarmConfig) -> String {
        indoc::indoc!(
            r#"
                __farm_use_if_file_found() {
                    if [[ -f .ruby-version ]]; then
                        farm local
                    fi
                }

                __farmcd() {
                    \cd "$@" || return $?
                    __farm_use_if_file_found
                }

                alias cd=__farmcd
                __farm_use_if_file_found
            "#
        )
        .into()
    }
}
