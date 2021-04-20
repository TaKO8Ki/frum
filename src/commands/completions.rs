use crate::cli::build_cli;
use crate::command::Command;
use crate::config::FarmConfig;
use crate::shell::{infer_shell, AVAILABLE_SHELLS};
use thiserror::Error;

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
    pub shell: Option<clap::Shell>,
}

impl Command for Completions {
    type Error = FarmError;

    fn apply(&self, _config: &FarmConfig) -> Result<(), Self::Error> {
        let mut stdio = std::io::stdout();
        let shell = self
            .shell
            .or_else(|| infer_shell().map(Into::into))
            .ok_or(FarmError::CantInferShell)?;
        build_cli().gen_completions_to(env!("CARGO_PKG_NAME"), shell, &mut stdio);
        Ok(())
    }
}

fn shells_as_string() -> String {
    AVAILABLE_SHELLS
        .iter()
        .map(|x| format!("* {}", x))
        .collect::<Vec<_>>()
        .join("\n")
}
