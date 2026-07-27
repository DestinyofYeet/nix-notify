use std::{
    sync::{Mutex, mpsc::Sender},
    time::Duration,
};

use crate::processor::commands::ProcessorCommand;

type Type = String;

#[derive(Debug)]
pub struct AtomFeed {
    pub(super) url: Type,
    pub(super) delay: Duration,
    pub(super) processor_tx: Sender<ProcessorCommand>,
}
