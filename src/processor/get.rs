use std::sync::mpsc::Sender;

use crate::processor::{Processor, commands::ProcessorCommand};

impl Processor {
    pub fn get_channel(&self) -> Sender<ProcessorCommand> {
        self.sender.clone()
    }
}
