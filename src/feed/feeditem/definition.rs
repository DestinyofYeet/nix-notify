use django_rs::chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct FeedItem {
    pub(super) feed_name: String,
    pub(super) message: String,
    pub(super) package: String,
    pub(super) commit: String,
    pub(super) updated: DateTime<Utc>,
    pub(super) author: String,
    pub(super) link: String,
}
