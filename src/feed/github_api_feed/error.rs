use thiserror::Error;

#[derive(Error, Debug)]
pub enum GithubApiError {
    #[error("Failed to build builder: {0}")]
    Builder(String),

    #[error("Failed to request api: {0}")]
    Request(String),

    #[error("Failed to parse response: {0}")]
    ParseResp(String),
}
