use std::{
    sync::{Mutex, mpsc::Sender},
    time::Duration,
};

use crate::{feed::atomfeed::AtomFeed, processor::commands::ProcessorCommand};

impl AtomFeed {
    pub fn new(url: String, delay: Duration, processor_tx: Sender<ProcessorCommand>) -> Self {
        Self {
            url,
            delay,
            processor_tx,
        }
    }
}
