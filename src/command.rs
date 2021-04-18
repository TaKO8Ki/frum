use crate::config::FarmConfig;
use crate::outln;
use colored::Colorize;

pub trait Command {
    type Error: std::error::Error;

    fn apply(&self, config: &FarmConfig) -> Result<(), Self::Error>;

    fn handle_error(err: Self::Error, config: &FarmConfig) {
        outln!(config#Error, "{} {}", "error:".red().bold(), format!("{}", err).red());
        std::process::exit(1);
    }

    fn call(&self, config: &FarmConfig) {
        match self.apply(&config) {
            Ok(()) => (),
            Err(err) => Self::handle_error(err, &config),
        }
    }
}
