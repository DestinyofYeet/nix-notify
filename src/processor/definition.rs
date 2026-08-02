use std::sync::OnceLock;

use crate::{config::ValidatedSubscription, feed::feeditem::FeedItem};

pub static NOTIFICATION_SUBSCRIPTIONS: OnceLock<Vec<ValidatedSubscription>> = OnceLock::new();

pub struct ProcessFeedItem {
    pub(super) item: FeedItem,
}
