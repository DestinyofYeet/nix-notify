use django_rs::chrono::{DateTime, Utc};

use crate::feed::feeditem::FeedItem;

impl FeedItem {
    pub fn new(
        feed_name: String,
        message: String,
        commit: String,
        updated: DateTime<Utc>,
        author: String,
        link: String,
        package: String,
    ) -> Self {
        Self {
            feed_name,
            package,
            message,
            commit,
            updated,
            author,
            link,
        }
    }
}
