use std::num::NonZeroU64;

use django_rs::{
    chrono::{DateTime, SecondsFormat, Utc},
    tasks::worker_logger::WorkerLogger,
};
use itertools::Itertools;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::Value;

use crate::feed::{
    feeditem::FeedItem,
    github_api_feed::{
        GithubApiError::{self, ParseResp},
        GithubApiFeed,
    },
};

#[derive(Deserialize)]
struct ApiItem {
    html_url: String,
    sha: String,
    commit: ApiCommit,
}

#[derive(Deserialize)]
struct ApiCommit {
    message: String,
    author: ApiAuthor,
}

#[derive(Deserialize)]
struct ApiAuthor {
    name: String,
    date: DateTime<Utc>,
}

static APP_NAME: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

impl GithubApiFeed {
    pub fn fetch(
        &self,
        page: NonZeroU64,
        items_per_page: NonZeroU64,
        since: Option<&DateTime<Utc>>,
        logger: &WorkerLogger,
    ) -> Result<Vec<FeedItem>, GithubApiError> {
        let mut headers: HeaderMap<HeaderValue> = HeaderMap::from_iter([
            (
                HeaderName::from_lowercase(b"accept").expect("to parse header"),
                HeaderValue::from_static("application/vnd.github+json"),
            ),
            (
                HeaderName::from_lowercase(b"x-github-api-version").expect("to parse header"),
                HeaderValue::from_static("2026-03-10"),
            ),
        ]);

        if let Some(api_token) = self.api_token.as_ref() {
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {api_token}")
                    .parse()
                    .expect("to set api token"),
            );
        }

        let client = reqwest::blocking::ClientBuilder::new()
            .default_headers(headers)
            .user_agent(APP_NAME)
            .build()
            .map_err(|e| GithubApiError::Builder(e.to_string()))?;

        let request = client
            .get(format!(
                "https://api.github.com/repos/{}/{}/commits?sha={}&per_page={}&page={}{}",
                self.repo_owner,
                self.repo_name,
                self.repo_branch,
                items_per_page,
                page,
                match since.as_ref() {
                    Some(value) => {
                        let value = format!(
                            "&since={}",
                            value.to_rfc3339_opts(SecondsFormat::Secs, true)
                        );
                        logger.trace(&format!("Making request with {}", value));
                        value
                    }
                    None => "".to_string(),
                }
            ))
            .build()
            .map_err(|e| GithubApiError::Builder(format!("Failed to build request: {e}")))?;

        let response = client
            .execute(request)
            .map_err(|e| GithubApiError::Request(e.to_string()))?;

        if let Some(limit) = response.headers().get("x-ratelimit-limit")
            && limit
                .to_str()
                .expect("to get str")
                .parse::<i32>()
                .expect("to parse number")
                == 60
            && self.api_token.is_some()
        {
            logger.warn("A api token is configured, but only 60 requests per hour are available. Is the token correct?");
        }

        if let Some(available) = response.headers().get("x-ratelimit-remaining")
            && available
                .to_str()
                .expect("to get str")
                .parse::<i32>()
                .expect("to parse number")
                == 0
        {
            return Err(GithubApiError::Request(
                "Github api limit reached".to_string(),
            ));
        }

        let content = response
            .error_for_status()
            .map_err(|e| GithubApiError::Request(e.to_string()))?;

        let content = content
            .json::<Value>()
            .map_err(|e| ParseResp(e.to_string()))?;

        let mut api_items: Vec<ApiItem> = Vec::new();

        if content.is_object() {
            let item: ApiItem =
                serde_json::from_value(content).map_err(|e| ParseResp(e.to_string()))?;

            api_items.push(item);
        } else if content.is_array() {
            let items: Vec<ApiItem> =
                serde_json::from_value(content).map_err(|e| ParseResp(e.to_string()))?;

            api_items = items;
        }

        let items = api_items
            .into_iter()
            .map(|item| {
                let ApiItem {
                    html_url,
                    sha,
                    commit,
                } = item;
                let ApiCommit { message, author } = commit;
                let ApiAuthor {
                    name,
                    date: timestamp,
                } = author;

                let split = message.split(":").collect_vec();
                let package = match split.first() {
                    Some(value) => value,
                    None => {
                        return Err(ParseResp(
                            "Failed to find package in commit msg".to_string(),
                        ));
                    }
                };

                let message = match split.last() {
                    Some(value) => value,
                    None => return Err(ParseResp("Failed to find message in commit".to_string())),
                };

                Ok(FeedItem::new(
                    self.feed_name.clone(),
                    message.to_string(),
                    sha,
                    timestamp,
                    name,
                    html_url,
                    package.to_string(),
                ))
            })
            .process_results(|e| e.collect_vec())?;

        Ok(items)
    }
}
