use std::sync::{Arc, Mutex, atomic::AtomicBool};

use django_rs::chrono::Duration;

use crate::rss::feed::RssFeed;

impl RssFeed {
    pub fn new(url: String, delay: Duration, stop_signal: Arc<AtomicBool>) -> Self {
        Self {
            url,
            delay,
            stop_signal,
        }
    }
}
