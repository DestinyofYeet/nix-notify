use std::{path::PathBuf, time::Duration};

#[derive(Debug, Clone)]
pub struct ValidatedConfig {
    pub general: ValidatedGeneral,
    pub feeds: Vec<ValidatedFeed>,
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
