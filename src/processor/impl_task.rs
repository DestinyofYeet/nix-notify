use django_rs::tasks::taskrunnable::{TaskResultable, TaskRunnable};
use tracing::{debug, trace};

use crate::processor::{Processor, commands::ProcessorCommand};

impl TaskRunnable for Processor {
    fn run(
        &mut self,
        logger: django_rs::tasks::worker_logger::WorkerLogger,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        let tx = match self.recv.lock() {
            Ok(lock) => lock,
            Err(e) => {
                logger.error(&format!("Failed to unlock mutex: {e}"));
                return Box::new(());
            }
        };

        while let Some(item) = tx.iter().next() {
            match item {
                ProcessorCommand::Process(rss_items) => logger.debug("processing"),
            }
        }

        Box::new(())
    }
}

impl TaskResultable for Processor {
    type Result = ();

    fn downcast(_: django_rs::tasks::task::TaskResult) -> Self::Result {}
}
