use std::{sync::atomic::Ordering, thread};

use django_rs::tasks::{
    task::TaskResult,
    taskrunnable::{TaskResultable, TaskRunnable},
};

use crate::rss::feed::RssFeed;

impl TaskRunnable for RssFeed {
    fn run(
        &mut self,
        logger: django_rs::tasks::worker_logger::WorkerLogger,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        loop {
            logger.trace("Fetching rss feed");

            match self.fetch() {
                Ok(value) => {
                    logger.debug(&format!("{:?}", value));
                }
                Err(e) => {
                    logger.error(&format!("Failed to fetch feed: {e}"));
                }
            };

            logger.trace(&format!("Sleeping {}s", self.delay.as_secs()));
            thread::sleep(self.delay);
        }
    }
}

impl TaskResultable for RssFeed {
    type Result = ();

    fn downcast(_: TaskResult) -> Self::Result {}
}
