use crate::config::ValidatedEmailConfig;

pub struct SendEmailTask {
    pub(super) recipient: String,
    pub(super) email_config: ValidatedEmailConfig,
}
