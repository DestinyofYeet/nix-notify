use std::time::Duration;

use crate::feed::atomfeed::AtomFeed;

impl AtomFeed {
    pub fn new(name: String, url: String, delay: Duration) -> Self {
        Self { name, url, delay }
    }
}
