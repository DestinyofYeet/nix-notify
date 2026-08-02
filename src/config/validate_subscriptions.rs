use crate::config::{ConfigError, Subscription, ValidatedSubscription};

impl Subscription {
    pub fn validate(self) -> Result<ValidatedSubscription, ConfigError> {
        let Subscription {
            via,
            recipient,
            feed_name,
            packages,
        } = self;

        Ok(ValidatedSubscription {
            via,
            recipient,
            feed_name,
            packages,
        })
    }
}
