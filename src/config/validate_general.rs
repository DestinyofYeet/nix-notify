use crate::config::{ConfigError, General, RawConfig, ValidatedGeneral};

impl General {
    pub fn validate(self) -> Result<ValidatedGeneral, ConfigError> {
        let General {
            database_path,
            github_api_token,
        } = self;

        let github_api_token = match github_api_token {
            Some(value) => Some(RawConfig::validate_maybe_file_input(value)?),
            None => None,
        };

        Ok(ValidatedGeneral {
            database_path,
            github_api_token,
        })
    }
}
