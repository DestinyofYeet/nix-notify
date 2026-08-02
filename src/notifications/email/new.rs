use crate::{config::ValidatedEmailConfig, notifications::email::SendEmail};

impl<'a> SendEmail<'a> {
    pub fn new(
        recipient: &'a str,
        email_config: &'a ValidatedEmailConfig,
        title: &'a str,
        content: &'a str,
    ) -> Self {
        Self {
            recipient,
            email_config,
            title,
            content,
        }
    }
}
