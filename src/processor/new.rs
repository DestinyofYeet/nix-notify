use crate::{feed::feeditem::FeedItem, processor::ProcessFeedItem};

impl ProcessFeedItem {
    pub fn new(item: FeedItem) -> Self {
        Self { item }
    }
}
