use std::time::Duration;

use crate::rss::feed::RssFeed;

impl RssFeed {
    pub fn new(url: String, delay: Duration) -> Self {
        Self { url, delay }
    }
}
