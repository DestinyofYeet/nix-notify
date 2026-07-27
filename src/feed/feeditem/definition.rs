use django_rs::chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct FeedItem {
    pub(super) message: String,
    pub(super) commit: String,
    pub(super) updated: DateTime<Utc>,
    pub(super) author: String,
    pub(super) link: String,
}
