use std::time::Duration;

type Type = String;

pub struct AtomFeed {
    pub(super) name: String,
    pub(super) url: Type,
    pub(super) delay: Duration,
}
