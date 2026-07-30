use crate::{config::ValidatedEmailConfig, notifications::email::SendEmailTask};

impl SendEmailTask {
    pub fn new(recipient: String, email_config: ValidatedEmailConfig) -> Self {
        Self {
            recipient,
            email_config,
        }
    }
}
