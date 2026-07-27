use thiserror::Error;

#[derive(Error, Debug)]
pub enum RssError {
    #[error("Failed to fetch feed: {0}")]
    Fetch(String),

    #[error("Failed to get bytes from feed: {0}")]
    Bytes(String),

    #[error("Failed to construct rss channel: {0}")]
    Channel(String),

    #[error("Failed to construct rss item: {0}")]
    Item(String),
}
