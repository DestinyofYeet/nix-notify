use django_rs::chrono::{DateTime, Utc};

use crate::feed::feeditem::FeedItem;

impl FeedItem {
    pub fn new(
        feed_name: String,
        message: String,
        commithash: String,
        updated: DateTime<Utc>,
        author: String,
        link: String,
        package: String,
    ) -> Self {
        Self {
            id: None,
            feed_name,
            package,
            message,
            commithash,
            updated,
            author,
            link,
        }
    }
}
