use std::thread;

use django_rs::{
    server::database_strategy::DatabaseStrategy,
    tasks::{
        runnable_info::RunnableInfo,
        task::TaskResult,
        taskrunnable::{TaskResultable, TaskRunnable},
    },
};

use crate::{feed::atomfeed::AtomFeed, processor::ProcessFeedItem};

impl<D> TaskRunnable<D> for AtomFeed
where
    D: DatabaseStrategy,
{
    fn run(&mut self, info: RunnableInfo<D>) -> Box<dyn std::any::Any + Send + Sync> {
        let logger = info.get_logger();
        loop {
            logger.debug(&format!("Fetching atom feed '{}'", self.name));

            match self.fetch() {
                Ok(value) => {
                    for item in value {
                        match info.spawn_task(ProcessFeedItem::new(item)) {
                            Ok(_) => {}
                            Err(e) => {
                                logger.error(&format!("Failed to spawn processing task: {e}"))
                            }
                        }
                    }
                }
                Err(e) => {
                    logger.error(&format!("Failed to fetch feed: {e}"));
                }
            };

            logger.debug(&format!("Sleeping {}s", self.delay.as_secs()));
            thread::sleep(self.delay);
        }
    }
}

impl TaskResultable for AtomFeed {
    type Result = ();

    fn downcast(_: TaskResult) -> Self::Result {}
}
