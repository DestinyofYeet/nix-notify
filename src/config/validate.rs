use std::{path::PathBuf, str::FromStr};

use itertools::Itertools;

use crate::config::{ConfigError, RawConfig, ValidatedConfig};

impl RawConfig {
    pub fn validate(self) -> Result<ValidatedConfig, ConfigError> {
        let general = self.general.validate()?;

        let feeds = self
            .feeds
            .into_iter()
            .map(|e| e.validate())
            .process_results(|res| res.collect_vec())?;

        let notifications = self
            .notifications
            .into_iter()
            .map(|e| e.validate())
            .process_results(|res| res.collect_vec())?;

        let subscriptions = self
            .subscriptions
            .into_iter()
            .map(|sub| sub.validate())
            .process_results(|result| result.collect_vec())?;

        Ok(ValidatedConfig {
            general,
            feeds,
            notifications,
            subscriptions,
        })
    }

    pub fn validate_maybe_file_input(input: String) -> Result<String, ConfigError> {
        match input.starts_with("@:") {
            false => Ok(input),
            true => {
                let path = match PathBuf::from_str(input.strip_prefix("@:").unwrap()) {
                    Ok(path) => {
                        if !path.exists() {
                            return Err(ConfigError::Validate(format!(
                                "Path {path:?} does not exist!"
                            )));
                        }

                        path
                    }
                    Err(e) => {
                        return Err(ConfigError::Validate(format!(
                            "Failed to construct PathBuf: {e}"
                        )));
                    }
                };

                let content = std::fs::read_to_string(&path)
                    .map_err(|e| ConfigError::Validate(format!("Failed to read {path:?}: {e}")))?;

                let content = content.trim().to_string();

                Ok(content)
            }
        }
    }

    pub fn validate_set_and_not_emtpy(
        input: Option<String>,
        name: &str,
        name_kind: &str,
        field_name: &str,
    ) -> Result<String, ConfigError> {
        match input {
            Some(value) => {
                if value.is_empty() {
                    return Err(ConfigError::Validate(format!(
                        "{name_kind} {name}: {field_name} is empty!"
                    )));
                }

                Ok(value)
            }
            None => Err(ConfigError::Validate(format!(
                "{name_kind} {name}: {field_name} is not set!"
            ))),
        }
    }
}
