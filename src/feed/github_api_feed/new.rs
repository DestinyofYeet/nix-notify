use std::time::Duration;

use crate::feed::github_api_feed::GithubApiFeed;

impl GithubApiFeed {
    pub fn new(
        feed_name: String,
        repo_owner: String,
        repo_name: String,
        repo_branch: String,
        delay: Duration,
        api_token: Option<String>,
    ) -> Self {
        Self {
            feed_name,
            repo_owner,
            repo_name,
            repo_branch,
            delay,
            api_token,
        }
    }
}
