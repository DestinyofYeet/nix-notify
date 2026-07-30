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
            repo_owner,
            repo_name,
        } = self;

        let branch = branch.unwrap_or(name.clone());

        let duration = Duration::from_mins(delay_minutes);

        let (repo_owner, repo_name) = match source {
            FeedSource::Nixpkgs => ("NixOS".to_string(), "nixpkgs".to_string()),
            FeedSource::Custom => {
                let repo_owner = match repo_owner {
                    Some(value) => {
                        if value.is_empty() {
                            return Err(ConfigError::Validate(format!(
                                "Feed {}: repo_owner is empty, but source is custom!",
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
                            "Feed {}: repo_name is not set, but source is custom!",
                            name
                        )));
                    }
                };

                (repo_owner, repo_name)
            }
        };

        let kind = match kind {
            FeedKind::GithubAtom => ValidatedFeedKind::Atom {
                url: format!(
                    "https://github.com/{}/{}/commits/{}.atom",
                    repo_owner, repo_name, branch
                ),
            },
            FeedKind::GithubApi => ValidatedFeedKind::GithubApi {
                repo_owner,
                repo_name,
                branch: branch.clone(),
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
