use crate::feed::feeditem::FeedItem;

impl FeedItem {
    pub fn get_package(&self) -> &str {
        &self.package
    }
}
