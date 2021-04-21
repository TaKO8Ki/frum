use crate::config::FrumConfig;
use crate::outln;
use colored::Colorize;

pub trait Command {
    type Error: std::error::Error;

    fn apply(&self, config: &FrumConfig) -> Result<(), Self::Error>;

    fn handle_error(err: Self::Error, config: &FrumConfig) {
        outln!(config#Error, "{} {}", "error:".red().bold(), format!("{}", err).red());
        std::process::exit(1);
    }

    fn call(&self, config: &FrumConfig) {
        match self.apply(&config) {
            Ok(()) => (),
            Err(err) => Self::handle_error(err, &config),
        }
    }
}
