use thiserror::Error;

#[derive(Error, Debug)]
pub enum AtomError {
    #[error("Failed to fetch feed: {0}")]
    Fetch(String),

    #[error("Failed to get bytes from feed: {0}")]
    Bytes(String),

    #[error("Failed to construct atom channel: {0}")]
    Channel(String),

    #[error("Failed to construct atom item: {0}")]
    Item(String),
}
