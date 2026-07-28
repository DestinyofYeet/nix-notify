use std::collections::HashMap;

use django_rs::tasks::taskrunnable::{TaskResultable, TaskRunnable};
use tracing::{debug, trace};

use crate::{
    feed::feeditem::FeedItem,
    processor::{Processor, commands::ProcessorCommand},
};

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

        // Subscriptions from package to Vec<Subscribers>
        let mut subscribers: HashMap<String, Vec<String>> = HashMap::new();

        let mut feed_items: Vec<FeedItem> = Vec::new();

        while let Some(item) = tx.iter().next() {
            match item {
                ProcessorCommand::Process(new_items) => {
                    for item in new_items {
                        if feed_items.contains(&item) {
                            continue;
                        }

                        logger.debug(&format!("New item:\n{:?}", item));

                        if let Some(subs) = subscribers.get(item.get_package()) {
                            logger.info(&format!("Notifying subscribers: {subs:?}"));
                        }

                        feed_items.push(item);
                    }
                }

                ProcessorCommand::Subscribe {
                    package,
                    notify_information,
                } => match subscribers.get_mut(&package) {
                    Some(values) => {
                        values.push(notify_information);
                    }
                    None => {
                        subscribers.insert(package, vec![notify_information]);
                    }
                },
            }
        }

        Box::new(())
    }
}

impl TaskResultable for Processor {
    type Result = ();

    fn downcast(_: django_rs::tasks::task::TaskResult) -> Self::Result {}
}
