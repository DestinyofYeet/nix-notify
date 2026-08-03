use django_rs::{
    models::search::SearchQuery,
    server::database_strategy::DatabaseStrategy,
    tasks::{
        runnable_info::RunnableInfo,
        taskrunnable::{TaskResultable, TaskRunnable},
    },
};

use crate::{
    feed::feeditem::FeedItem,
    notifications::SendNotification,
    processor::{NOTIFICATION_SUBSCRIPTIONS, ProcessFeedItem},
};

impl<D> TaskRunnable<D> for ProcessFeedItem
where
    D: DatabaseStrategy,
{
    fn run(&mut self, info: RunnableInfo<D>) -> Box<dyn std::any::Any + Send + Sync> {
        let ret_value = Box::new(());

        let logger = info.get_logger();
        let db = info.get_database();

        let item = match db.search_single_model::<FeedItem>(
            &db.get_connection(),
            SearchQuery::empty()
                .add_constraint(("commithash", self.item.get_commit()))
                .add_constraint(("feed_name", self.item.get_feed_name())),
        ) {
            Ok(value) => value,
            Err(e) => {
                logger.error(&format!("Failed to check if model exists: {e}"));
                return ret_value;
            }
        };

        if item.is_some() {
            logger.trace(&format!("Item {:?} already in db", self.item));
            return ret_value;
        }

        logger.info(&format!(
            "New item on '{}': [{}] {}",
            self.item.get_feed_name(),
            self.item.get_package(),
            self.item.get_message()
        ));

        match db.save_model(&db.get_connection(), &mut self.item) {
            Ok(_) => {}
            Err(e) => {
                logger.error(&format!("Failed to save model: {e}"));
            }
        };

        let subs = NOTIFICATION_SUBSCRIPTIONS
            .get()
            .expect("for oncelock to be set up");

        for sub in subs.iter() {
            if sub.packages.contains(&self.item.get_package().to_string()) {
                match info.spawn_task(SendNotification::new(
                    sub.via.clone(),
                    sub.recipient.clone(),
                    format!(
                        "[nix-notify] '{}': {}",
                        self.item.get_feed_name(),
                        self.item.get_package()
                    ),
                    self.item.get_message().to_string(),
                )) {
                    Ok(_) => {}
                    Err(e) => {
                        logger.error(&format!("Failed to queue Notification task: {e}!"));
                    }
                }
            }
        }

        ret_value
    }
}

impl TaskResultable for ProcessFeedItem {
    type Result = ();

    fn downcast(_: django_rs::tasks::task::TaskResult) -> Self::Result {}
}
