use std::{num::NonZero, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawConfig {
    pub(super) feeds: Vec<Feed>,
    pub(super) general: General,
    pub(super) notifications: Vec<Notification>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct General {
    pub database_path: PathBuf,
    pub github_api_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct Feed {
    pub(super) name: String,
    pub(super) branch: Option<String>,
    pub(super) delay_minutes: u64,
    pub(super) source: FeedSource,
    pub(super) kind: FeedKind,

    pub(super) repo_owner: Option<String>,
    pub(super) repo_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) enum FeedSource {
    #[serde(rename = "nixpkgs")]
    Nixpkgs,
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) enum FeedKind {
    #[serde(rename = "github_atom")]
    GithubAtom,
    #[serde(rename = "github_api")]
    GithubApi,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) struct Notification {
    pub(super) kind: NotificationKind,
    pub(super) name: String,
    pub(super) smtp_host: Option<String>,
    pub(super) smtp_port: Option<NonZero<u64>>,
    pub(super) envelope_from: Option<String>,
    pub(super) login_username: Option<String>,
    pub(super) login_password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(super) enum NotificationKind {
    #[serde(rename = "email")]
    Email,
}
