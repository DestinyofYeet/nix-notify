use std::time::Duration;

type Type = String;

#[derive(Debug)]
pub struct RssFeed {
    pub(super) url: Type,
    pub(super) delay: Duration,
}
