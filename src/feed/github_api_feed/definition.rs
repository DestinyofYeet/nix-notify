use std::time::Duration;

pub struct GithubApiFeed {
    pub(super) feed_name: String,
    pub(super) repo_owner: String,
    pub(super) repo_name: String,
    pub(super) repo_branch: String,
    pub(super) api_token: Option<String>,

    pub(super) delay: Duration,
}
