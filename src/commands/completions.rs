use crate::cli::build_cli;
use crate::command::Command;
use crate::config::FrumConfig;
use crate::outln;
use crate::shell::{infer_shell, AVAILABLE_SHELLS};
use crate::version::{is_dotfile, Version};
use clap::Shell;
use thiserror::Error;

const USE_COMMAND_REGEX: &str = r#"opts=" -h -V  --help --version  "#;
const INSTALL_COMMAND_REGEX: &str = r#"opts=" -l -h -V  --list --help --version  "#;
const UNINSTALL_COMMAND_REGEX: &str = r#"opts=" -h -V  --help --version  "#;
const LOCAL_COMMAND_REGEX: &str = r#"opts=" -q -h -V  --quiet --help --version  "#;

#[derive(Debug)]
enum FrumCommand {
    Install,
    Uninstall,
    Local,
    Global,
    None,
}

#[derive(Error, Debug)]
pub enum FrumError {
    #[error(
        "{}\n{}\n{}\n{}",
        "Can't infer shell!",
        "frum can't infer your shell based on the process tree.",
        "Maybe it is unsupported? we support the following shells:",
        shells_as_string()
    )]
    CantInferShell,
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SemverError(#[from] semver::SemVerError),
}

pub struct Completions {
    pub shell: Option<Shell>,
    pub list: bool,
}

impl Command for Completions {
    type Error = FrumError;

    fn apply(&self, config: &FrumConfig) -> Result<(), Self::Error> {
        if self.list {
            for entry in config
                .versions_dir()
                .read_dir()
                .map_err(FrumError::IoError)?
            {
                let entry = entry.map_err(FrumError::IoError)?;
                if is_dotfile(&entry) {
                    continue;
                }

                let path = entry.path();
                let filename = path
                    .file_name()
                    .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
                    .map_err(FrumError::IoError)?
                    .to_str()
                    .ok_or_else(|| std::io::Error::from(std::io::ErrorKind::NotFound))
                    .map_err(FrumError::IoError)?;
                let version = Version::parse(filename).map_err(FrumError::SemverError)?;
                outln!(config#Info, "{} {}", " ", version);
            }
            return Ok(());
        }

        let shell = self
            .shell
            .or_else(|| infer_shell().map(Into::into))
            .ok_or(FrumError::CantInferShell)?;

        print!(
            "{}",
            customize_completions(shell).expect("invalid completions")
        );
        Ok(())
    }
}

fn customize_completions(shell: Shell) -> Option<String> {
    use std::io::BufWriter;
    let mut buffer = BufWriter::new(Vec::new());
    build_cli().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut buffer);
    let bytes = buffer.into_inner().unwrap();
    let string = String::from_utf8(bytes).unwrap();
    let string_split = string.split('\n');
    let mut completions = String::new();
    let mut subcommand = FrumCommand::None;
    let use_command_regex =
        regex::Regex::new(format!(r#"(\s+){}{} "#, USE_COMMAND_REGEX, "<version>").as_str())
            .unwrap();
    let install_command_regex =
        regex::Regex::new(format!(r#"(\s+){}{} "#, INSTALL_COMMAND_REGEX, "<version>").as_str())
            .unwrap();
    let uninstall_command_regex =
        regex::Regex::new(format!(r#"(\s+){}{} "#, UNINSTALL_COMMAND_REGEX, "<version>").as_str())
            .unwrap();
    let local_command_regex =
        regex::Regex::new(format!(r#"(\s+){}{} "#, LOCAL_COMMAND_REGEX, "<version>").as_str())
            .unwrap();
    match shell {
        Shell::Zsh => {
            for (index, line) in string_split.clone().enumerate() {
                if index == string_split.clone().count() - 1 {
                    break;
                }
                subcommand = match line {
                    "(local)" => FrumCommand::Local,
                    "(global)" => FrumCommand::Global,
                    "(install)" => FrumCommand::Install,
                    "(uninstall)" => FrumCommand::Uninstall,
                    _ => subcommand,
                };
                completions.push_str(
                    format!(
                        "{}\n",
                        match subcommand {
                            FrumCommand::Local => match line {
                                r#"'::version:_files' \"# =>
                                    r#"'::version:_values 'version' $(frum completions --list)' \"#
                                        .to_string(),
                                _ => line.to_string(),
                            },
                            FrumCommand::Global => match line {
                                r#"':version:_files' \"# =>
                                    r#"':version:_values 'version' $(frum completions --list)' \"#
                                        .to_string(),
                                _ => line.to_string(),
                            },
                            FrumCommand::Install => match line {
                                r#"'::version:_files' \"# =>
                                    r#"'::version:_values 'version' $(frum install -l)' \"#
                                        .to_string(),
                                _ => line.to_string(),
                            },
                            FrumCommand::Uninstall => match line {
                                r#"':version:_files' \"# =>
                                    r#"':version:_values 'version' $(frum install -l)' \"#
                                        .to_string(),
                                _ => line.to_string(),
                            },
                            FrumCommand::None => line.to_string(),
                        }
                    )
                    .as_str(),
                );
            }
            Some(completions)
        }
        Shell::Bash => {
            for (index, line) in string_split.clone().enumerate() {
                if index == string_split.clone().count() - 1 {
                    break;
                }
                subcommand = if line.ends_with("frum__local)") {
                    FrumCommand::Local
                } else if line.ends_with("frum__global)") {
                    FrumCommand::Global
                } else if line.ends_with("frum__install)") {
                    FrumCommand::Install
                } else if line.ends_with("frum__uninstall)") {
                    FrumCommand::Uninstall
                } else {
                    subcommand
                };
                completions.push_str(
                    format!(
                        "{}\n",
                        match subcommand {
                            FrumCommand::Local =>
                                if local_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(frum completions --list) ""#,
                                        local_command_regex
                                            .captures(line)
                                            .unwrap()
                                            .get(1)
                                            .unwrap()
                                            .as_str(),
                                        LOCAL_COMMAND_REGEX
                                    )
                                } else {
                                    line.to_string()
                                },
                            FrumCommand::Global =>
                                if use_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(frum completions --list) ""#,
                                        use_command_regex
                                            .captures(line)
                                            .unwrap()
                                            .get(1)
                                            .unwrap()
                                            .as_str(),
                                        USE_COMMAND_REGEX
                                    )
                                } else {
                                    line.to_string()
                                },
                            FrumCommand::Install =>
                                if install_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(frum install -l) ""#,
                                        install_command_regex
                                            .captures(line)
                                            .unwrap()
                                            .get(1)
                                            .unwrap()
                                            .as_str(),
                                        INSTALL_COMMAND_REGEX
                                    )
                                } else {
                                    line.to_string()
                                },
                            FrumCommand::Uninstall =>
                                if uninstall_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(frum install -l) ""#,
                                        uninstall_command_regex
                                            .captures(line)
                                            .unwrap()
                                            .get(1)
                                            .unwrap()
                                            .as_str(),
                                        UNINSTALL_COMMAND_REGEX
                                    )
                                } else {
                                    line.to_string()
                                },
                            FrumCommand::None => line.to_string(),
                        }
                    )
                    .as_str(),
                );
            }
            Some(completions)
        }
        _ => {
            for (index, line) in string_split.clone().enumerate() {
                if index == string_split.clone().count() - 1 {
                    break;
                }
                completions.push_str(format!("{}\n", line).as_str())
            }
            Some(completions)
        }
    }
}

fn shells_as_string() -> String {
    AVAILABLE_SHELLS
        .iter()
        .map(|x| format!("* {}", x))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::customize_completions;
    use crate::config::FrumConfig;
    use clap::Shell;
    use difference::assert_diff;
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;
    use tempfile::tempdir;

    #[test]
    fn test_zsh_completions() {
        let mut config = FrumConfig::default();
        config.base_dir = Some(tempdir().unwrap().path().to_path_buf());

        let file = File::open("completions/frum.zsh").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut expected = String::new();
        buf_reader.read_to_string(&mut expected).unwrap();
        let actual = customize_completions(Shell::Zsh).unwrap();
        assert_diff!(actual.as_str(), expected.as_str(), "\n", 0);
    }

    #[test]
    fn test_bash_completions() {
        let mut config = FrumConfig::default();
        config.base_dir = Some(tempdir().unwrap().path().to_path_buf());

        let file = File::open("completions/frum.bash").unwrap();
        let mut buf_reader = BufReader::new(file);
        let mut expected = String::new();
        buf_reader.read_to_string(&mut expected).unwrap();
        let actual = customize_completions(Shell::Bash).unwrap();
        assert_diff!(actual.as_str(), expected.as_str(), "\n", 0);
    }
}
