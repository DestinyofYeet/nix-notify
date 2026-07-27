use std::sync::{Arc, Mutex, atomic::AtomicBool};

use django_rs::chrono::Duration;

#[derive(Debug)]
pub struct RssFeed {
    pub(super) url: String,
    pub(super) delay: Duration,

    pub(super) stop_signal: Arc<AtomicBool>,
}
