use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub feeds: Vec<Feed>,
    pub general: General,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct General {
    pub database_path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feed {
    pub name: String,
    pub delay_minutes: u64,
    pub source: FeedSource,
    pub kind: FeedKind,

    /// Used when source == FeedSource::Custom
    pub url: Option<String>,
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
