use std::time::Duration;

use crate::config::{ConfigError, Feed, FeedKind, FeedSource, ValidatedFeed, ValidatedFeedKind};

impl Feed {
    pub fn validate(self) -> Result<ValidatedFeed, ConfigError> {
        let Feed {
            name,
            branch,
            delay_minutes,
            source,
            kind,
            url,
            repo_owner,
            repo_name,
        } = self;

        let branch = branch.unwrap_or(name.clone());

        let duration = Duration::from_mins(delay_minutes);

        let kind = match kind {
            FeedKind::GithubAtom => match source {
                FeedSource::Nixpkgs => ValidatedFeedKind::Atom {
                    url: format!("https://github.com/NixOS/nixpkgs/commits/{}.atom", branch),
                },
                FeedSource::Custom => {
                    let url = match url {
                        Some(url) => url,
                        None => {
                            return Err(ConfigError::Validate(format!(
                                "feed {}: url is not set, but source is custom!",
                                name
                            )));
                        }
                    };

                    if url.is_empty() {
                        return Err(ConfigError::Validate(format!(
                            "feed {}: url is empty, but source is custom!",
                            name
                        )));
                    }

                    ValidatedFeedKind::Atom { url }
                }
            },
            FeedKind::GithubApi => match source {
                FeedSource::Nixpkgs => ValidatedFeedKind::GithubApi {
                    repo_owner: "NixOS".to_string(),
                    repo_name: "nixpkgs".to_string(),
                    branch: branch.clone(),
                },
                FeedSource::Custom => {
                    let repo_owner = match repo_owner {
                        Some(value) => {
                            if value.is_empty() {
                                return Err(ConfigError::Validate(format!(
                                    "Feed: {}: repo_owner is empty, but source is custom!",
                                    name
                                )));
                            }

                            value
                        }
                        None => {
                            return Err(ConfigError::Validate(format!(
                                "Feed {}: repo_owner is not set, but source is custom!",
                                name
                            )));
                        }
                    };

                    let repo_name = match repo_name {
                        Some(value) => {
                            if value.is_empty() {
                                return Err(ConfigError::Validate(format!(
                                    "Feed {}: repo_name is empty, but source is custom!",
                                    name
                                )));
                            }

                            value
                        }
                        None => {
                            return Err(ConfigError::Validate(format!(
                                "Feed: {}: repo_name is not set, but source is custom!",
                                name
                            )));
                        }
                    };

                    ValidatedFeedKind::GithubApi {
                        repo_owner,
                        repo_name,
                        branch: branch.clone(),
                    }
                }
            },
        };

        Ok(ValidatedFeed {
            name,
            branch,
            delay: duration,
            kind,
        })
    }
}
