mod cors;
mod environment;
mod logger;

pub struct Config {}

impl Config {
    pub fn init() -> Result<(), dotenvy::Error> {
        environment::load_environment();
        Self::logger_init();
        Ok(())
    }
}
