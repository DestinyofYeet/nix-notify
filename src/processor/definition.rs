use std::sync::{
    Mutex,
    mpsc::{Receiver, Sender},
};

use crate::processor::commands::ProcessorCommand;

pub struct Processor {
    pub(super) recv: Mutex<Receiver<ProcessorCommand>>,
    pub(super) sender: Sender<ProcessorCommand>,
}
