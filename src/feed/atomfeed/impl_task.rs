use std::thread;

use django_rs::tasks::{
    task::TaskResult,
    taskrunnable::{TaskResultable, TaskRunnable},
};

use crate::{feed::atomfeed::AtomFeed, processor::commands::ProcessorCommand};

impl TaskRunnable for AtomFeed {
    fn run(
        &mut self,
        logger: django_rs::tasks::worker_logger::WorkerLogger,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        loop {
            logger.trace(&format!("Fetching rss feed '{}'", self.name));

            match self.fetch() {
                Ok(value) => match self.processor_tx.send(ProcessorCommand::Process(value)) {
                    Ok(_) => {}
                    Err(e) => {
                        logger.error(&format!("Failed to send command to processor: {e}"));
                    }
                },
                Err(e) => {
                    logger.error(&format!("Failed to fetch feed: {e}"));
                }
            };

            logger.trace(&format!("Sleeping {}s", self.delay.as_secs()));
            thread::sleep(self.delay);
        }
    }
}

impl TaskResultable for AtomFeed {
    type Result = ();

    fn downcast(_: TaskResult) -> Self::Result {}
}
