use django_rs::chrono::{DateTime, Utc};

use crate::feed::feeditem::FeedItem;

impl FeedItem {
    pub fn new(
        message: String,
        commit: String,
        updated: DateTime<Utc>,
        author: String,
        link: String,
    ) -> Self {
        Self {
            message,
            commit,
            updated,
            author,
            link,
        }
    }
}
