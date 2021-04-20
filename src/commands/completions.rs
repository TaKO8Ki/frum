use crate::cli::build_cli;
use crate::command::Command;
use crate::config::FarmConfig;
use crate::shell::{infer_shell, AVAILABLE_SHELLS};
use clap::Shell;
use thiserror::Error;

const USE_COMMAND_REGEX: &str = r#"opts=" -h -V  --help --version  "#;
const INSTALL_COMMAND_REGEX: &str =
    r#"opts=" -l -w -h -V  --list --with-openssl-dir --help --version  "#;
const UNINSTALL_COMMAND_REGEX: &str = r#"opts=" -h -V  --help --version  "#;

#[derive(Debug)]
enum FarmCommand {
    Install,
    Uninstall,
    Local,
    Global,
    None,
}

#[derive(Error, Debug)]
pub enum FarmError {
    #[error(
        "{}\n{}\n{}\n{}",
        "Can't infer shell!",
        "fnm can't infer your shell based on the process tree.",
        "Maybe it is unsupported? we support the following shells:",
        shells_as_string()
    )]
    CantInferShell,
}

pub struct Completions {
    pub shell: Option<Shell>,
}

impl Command for Completions {
    type Error = FarmError;

    fn apply(&self, config: &FarmConfig) -> Result<(), Self::Error> {
        use std::io::BufWriter;
        let mut buffer = BufWriter::new(Vec::new());
        let shell = self
            .shell
            .or_else(|| infer_shell().map(Into::into))
            .ok_or(FarmError::CantInferShell)?;
        build_cli().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut buffer);
        let bytes = buffer.into_inner().unwrap();
        let string = String::from_utf8(bytes).unwrap();
        println!(
            "{}",
            customize_completions(shell, string, config.versions_dir())
                .expect("invalid completions")
        );
        Ok(())
    }
}

fn customize_completions(
    shell: Shell,
    string: String,
    version_dir: std::path::PathBuf,
) -> Option<String> {
    let string_split = string.split('\n');
    let mut completions = String::new();
    let mut subcommand = FarmCommand::None;
    let use_command_regex =
        regex::Regex::new(format!(r#"(\s+){}{} "#, USE_COMMAND_REGEX, "<version>").as_str())
            .unwrap();
    let install_command_regex =
        regex::Regex::new(format!(r#"(\s+){}{} "#, INSTALL_COMMAND_REGEX, "<version>").as_str())
            .unwrap();
    let uninstall_command_regex =
        regex::Regex::new(format!(r#"(\s+){}{} "#, UNINSTALL_COMMAND_REGEX, "<version>").as_str())
            .unwrap();
    match shell {
        Shell::Zsh => {
            for (index, line) in string_split.clone().enumerate() {
                if index == string_split.clone().count() {
                    continue;
                }
                subcommand = match line {
                    "(local)" => FarmCommand::Local,
                    "(global)" => FarmCommand::Global,
                    "(install)" => FarmCommand::Install,
                    "(uninstall)" => FarmCommand::Uninstall,
                    _ => subcommand,
                };
                completions.push_str(
                    format!(
                        "{}\n",
                        match subcommand {
                            FarmCommand::Local => match line {
                                r#"'::version:_files' \"# => format!(
                                    r#"'::version:_values 'version' $(ls {})' \"#,
                                    version_dir.to_str().unwrap()
                                ),
                                _ => line.to_string(),
                            },
                            FarmCommand::Global => match line {
                                r#"':version:_files' \"# => format!(
                                    r#"':version:_values 'version' $(ls {})' \"#,
                                    version_dir.to_str().unwrap()
                                ),
                                _ => line.to_string(),
                            },
                            FarmCommand::Install => match line {
                                r#"'::version:_files' \"# =>
                                    r#"'::version:_values 'version' $(farm install -l)' \"#
                                        .to_string(),
                                _ => line.to_string(),
                            },
                            FarmCommand::Uninstall => match line {
                                r#"':version:_files' \"# =>
                                    r#"':version:_values 'version' $(farm install -l)' \"#
                                        .to_string(),
                                _ => line.to_string(),
                            },
                            FarmCommand::None => line.to_string(),
                        }
                    )
                    .as_str(),
                );
            }
            Some(completions)
        }
        Shell::Bash => {
            for (index, line) in string_split.clone().enumerate() {
                if index == string_split.clone().count() {
                    continue;
                }
                subcommand = if line.ends_with("farm__local)") {
                    FarmCommand::Local
                } else if line.ends_with("farm__global)") {
                    FarmCommand::Global
                } else if line.ends_with("farm__install)") {
                    FarmCommand::Install
                } else if line.ends_with("farm__uninstall)") {
                    FarmCommand::Uninstall
                } else {
                    subcommand
                };
                completions.push_str(
                    format!(
                        "{}\n",
                        match subcommand {
                            FarmCommand::Local =>
                                if use_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(ls {}) ""#,
                                        use_command_regex
                                            .captures(line)
                                            .unwrap()
                                            .get(1)
                                            .unwrap()
                                            .as_str(),
                                        USE_COMMAND_REGEX,
                                        version_dir.to_str().unwrap()
                                    )
                                } else {
                                    line.to_string()
                                },
                            FarmCommand::Global =>
                                if use_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(ls {}) ""#,
                                        use_command_regex
                                            .captures(line)
                                            .unwrap()
                                            .get(1)
                                            .unwrap()
                                            .as_str(),
                                        USE_COMMAND_REGEX,
                                        version_dir.to_str().unwrap()
                                    )
                                } else {
                                    line.to_string()
                                },
                            FarmCommand::Install =>
                                if install_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(farm install -l) ""#,
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
                            FarmCommand::Uninstall =>
                                if uninstall_command_regex.is_match(line) {
                                    format!(
                                        r#"{}{}$(farm install -l) ""#,
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
                            FarmCommand::None => line.to_string(),
                        }
                    )
                    .as_str(),
                );
            }
            Some(completions)
        }
        _ => None,
    }
}

fn shells_as_string() -> String {
    AVAILABLE_SHELLS
        .iter()
        .map(|x| format!("* {}", x))
        .collect::<Vec<_>>()
        .join("\n")
}
