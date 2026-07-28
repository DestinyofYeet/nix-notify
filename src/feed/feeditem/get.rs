use crate::feed::feeditem::FeedItem;

impl FeedItem {
    pub fn get_package(&self) -> &str {
        &self.package
    }

    pub fn get_commit(&self) -> &str {
        &self.commithash
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    pub fn get_feed_name(&self) -> &str {
        &self.feed_name
    }
}
