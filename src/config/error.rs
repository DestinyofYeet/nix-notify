use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to validate config: {0}")]
    Validate(String),
}
