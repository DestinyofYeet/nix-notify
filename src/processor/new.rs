use std::sync::{Mutex, mpsc::channel};

use crate::processor::Processor;

impl Processor {
    pub fn new() -> Self {
        let (sender, recv) = channel();

        Self {
            recv: Mutex::new(recv),
            sender,
        }
    }
}
