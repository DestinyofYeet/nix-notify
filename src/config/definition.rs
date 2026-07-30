use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawConfig {
    pub(super) feeds: Vec<Feed>,
    pub(super) general: General,
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

    /// Used when source == FeedSource::Custom && kind == FeedKind::Atom
    pub(super) url: Option<String>,

    /// Used when kind == FeedKind::GithubApi
    pub(super) repo_owner: Option<String>,
    pub(super) repo_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FeedSource {
    #[serde(rename = "nixpkgs")]
    Nixpkgs,
    #[serde(rename = "custom")]
    Custom,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FeedKind {
    #[serde(rename = "github_atom")]
    GithubAtom,
    #[serde(rename = "github_api")]
    GithubApi,
}
