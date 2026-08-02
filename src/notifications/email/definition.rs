use crate::config::ValidatedEmailConfig;

#[derive(Debug, Clone)]
pub struct SendEmail<'a> {
    pub(super) recipient: &'a str,
    pub(super) title: &'a str,
    pub(super) content: &'a str,
    pub(super) email_config: &'a ValidatedEmailConfig,
}
