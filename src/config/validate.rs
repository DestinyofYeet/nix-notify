use crate::config::{ConfigError, RawConfig, ValidatedConfig};

impl RawConfig {
    pub fn validate(self) -> Result<ValidatedConfig, ConfigError> {
        let general = self.general.validate()?;
        let mut feeds = Vec::with_capacity(self.feeds.len());

        for feed in self.feeds {
            feeds.push(feed.validate()?);
        }

        Ok(ValidatedConfig { general, feeds })
    }
}
