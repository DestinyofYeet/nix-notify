use django_rs::chrono::{DateTime, Utc};

use crate::rss::feeditem::RssItem;

impl RssItem {
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
