use std::{path::PathBuf, str::FromStr};

use tracing::info;

use crate::config::{ConfigError, General, ValidatedGeneral};

impl General {
    pub fn validate(self) -> Result<ValidatedGeneral, ConfigError> {
        let General {
            database_path,
            mut github_api_token,
        } = self;

        if let Some(token) = github_api_token {
            if token.starts_with("@:") {
                let path = match PathBuf::from_str(token.strip_prefix("@:").unwrap()) {
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

                let content = content
                    .strip_suffix("\n")
                    .expect("to trim \\n from the end of the string")
                    .to_string();

                github_api_token = Some(content);

                dbg!(&github_api_token);

                info!("Read token from file");
            } else {
                github_api_token = Some(token);
            }
        }

        Ok(ValidatedGeneral {
            database_path,
            github_api_token,
        })
    }
}
