use crate::config::FarmConfig;

pub trait Command {
    type Error: std::error::Error;

    fn apply(&self, config: &FarmConfig) -> Result<(), Self::Error>;

    fn handle_error(err: Self::Error, _config: &FarmConfig) {
        eprintln!("farm: {}", err)
    }

    fn call(&self, config: &FarmConfig) {
        match self.apply(&config) {
            Ok(()) => (),
            Err(err) => Self::handle_error(err, &config),
        }
    }
}
