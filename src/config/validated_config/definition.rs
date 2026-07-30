use std::{num::NonZero, path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
pub struct ValidatedConfig {
    pub general: ValidatedGeneral,
    pub feeds: Vec<ValidatedFeed>,
    pub notifications: Vec<ValidatedNotification>,
}

#[derive(Debug, Clone)]
pub struct ValidatedGeneral {
    pub database_path: PathBuf,
    pub github_api_token: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ValidatedFeed {
    pub name: String,
    pub branch: String,
    pub delay: Duration,
    pub kind: ValidatedFeedKind,
}

#[derive(Debug, Clone)]
pub enum ValidatedFeedKind {
    Atom {
        url: String,
    },
    GithubApi {
        repo_owner: String,
        repo_name: String,
        branch: String,
    },
}

#[derive(Debug, Clone)]
pub struct ValidatedNotification {
    pub name: String,
    pub kind: ValidatedNotificationKind,
}

#[derive(Debug, Clone)]
pub enum ValidatedNotificationKind {
    Email(ValidatedEmailConfig),
}

#[derive(Debug, Clone)]
pub struct ValidatedEmailConfig {
    pub smtp_host: String,
    pub smtp_port: NonZero<u64>,
    pub envelope_from: String,
    pub login_username: String,
    pub login_password: String,
}
