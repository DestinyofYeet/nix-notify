use std::num::{NonZero, NonZeroU64};

use django_rs::{
    chrono::TimeDelta,
    models::search::{SearchOrderByOptions, SearchQuery},
    server::database_strategy::DatabaseStrategy,
    tasks::taskrunnable::{TaskResultable, TaskRunnable},
};

use crate::{
    feed::{feeditem::FeedItem, github_api_feed::GithubApiFeed},
    processor::ProcessFeedItem,
};

impl<D> TaskRunnable<D> for GithubApiFeed
where
    D: DatabaseStrategy,
{
    fn run(
        &mut self,
        info: django_rs::tasks::runnable_info::RunnableInfo<D>,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        let ret_value = Box::new(());

        let logger = info.get_logger();
        let db = info.get_database();

        if self.api_token.is_some() {
            logger.info("Github api token is set.");
        }

        let mut last_item = {
            let item = match db.search_single_model::<FeedItem>(
                &db.get_connection(),
                SearchQuery::empty()
                    .add_constraint(("feed_name", &self.feed_name))
                    .add_order_by(vec![("updated", Some(SearchOrderByOptions::Desc))]),
            ) {
                Ok(value) => {
                    logger.info("Pulled base state from db");
                    value
                }
                Err(e) => {
                    logger.error(&format!(
                        "Could not initialize base state: Failed to query database: {e}"
                    ));
                    return ret_value;
                }
            };

            match item {
                Some(item) => item,
                None => {
                    logger.info("Initializing base state from api");

                    match self.fetch(
                        NonZeroU64::new(1).unwrap(),
                        NonZeroU64::new(1).unwrap(),
                        None,
                        logger,
                    ) {
                        Ok(values) => match values.first() {
                            Some(item) => item.clone(),
                            None => {
                                logger.error("Could not initialize base state: Github Api returned no results");
                                return ret_value;
                            }
                        },
                        Err(e) => {
                            logger.error(&format!(
                                "Could not initialize base state: Failed to fetch Github api: {e}"
                            ));
                            return ret_value;
                        }
                    }
                }
            }
        };

        logger.trace(&format!("last_item init: {last_item:?}"));

        match info.spawn_task(ProcessFeedItem::new(last_item.clone())) {
            Ok(_) => {}
            Err(e) => {
                logger.warn(&format!("Failed to queue processing for last_item: {e}"));
            }
        };

        loop {
            let mut page: NonZeroU64 = NonZero::new(1).unwrap();
            let items_per_page: NonZeroU64 = NonZero::new(100).unwrap();

            loop {
                logger.debug(&format!(
                    "Fetching Github api for feed '{}'",
                    self.feed_name
                ));
                let values = match self.fetch(
                    page,
                    items_per_page,
                    Some(
                        &last_item
                            .get_updated_at()
                            .checked_add_signed(TimeDelta::seconds(1))
                            .expect("to be able to add a second"),
                    ),
                    logger,
                ) {
                    Ok(values) => values,
                    Err(e) => {
                        logger.warn(&format!("Failed to fetch github api: {e}"));
                        break;
                    }
                };

                logger.debug(&format!("fetched {} commits | page: {page}", values.len()));

                if values.is_empty() {
                    break;
                }

                let length = values.len();

                for value in values.into_iter() {
                    if value.get_updated_at() > last_item.get_updated_at() {
                        last_item = value.clone();
                    }

                    match info.spawn_task(ProcessFeedItem::new(value)) {
                        Ok(_) => {}
                        Err(e) => {
                            logger.warn(&format!("Failed to queue processing: {e}"));
                        }
                    }
                }

                if length as u64 != items_per_page.get() {
                    logger.trace("early break");
                    break;
                }

                page = page.checked_add(1).unwrap();
            }

            logger.trace(&format!("last_item post: {last_item:?}"));

            logger.debug(&format!("Sleeping {}s", self.delay.as_secs()));
            std::thread::sleep(self.delay);
        }
    }
}

impl TaskResultable for GithubApiFeed {
    type Result = ();

    fn downcast(_: django_rs::tasks::task::TaskResult) -> Self::Result {}
}
