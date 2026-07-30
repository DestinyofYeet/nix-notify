use django_rs::{
    server::database_strategy::DatabaseStrategy,
    tasks::taskrunnable::{TaskResultable, TaskRunnable},
};

use crate::{
    config::ValidatedNotificationKind,
    notifications::{NOTIFICATION_CONFIGS, SendNotification},
};

impl<D> TaskRunnable<D> for SendNotification
where
    D: DatabaseStrategy,
{
    fn run(
        &mut self,
        info: django_rs::tasks::runnable_info::RunnableInfo<D>,
    ) -> Box<dyn std::any::Any + Send + Sync> {
        let ret_value = Box::new(());

        let logger = info.get_logger();

        let notification = match NOTIFICATION_CONFIGS
            .get()
            .expect("to have setup OnceLock")
            .get(&self.name)
        {
            Some(value) => value,
            None => {
                logger.error(&format!(
                    "Failed to find notification handler with name '{}'",
                    self.name
                ));

                return ret_value;
            }
        };

        match notification {
            ValidatedNotificationKind::Email(_validated_email_config) => {
                logger.info("Sending email");
            }
        }

        ret_value
    }
}

impl TaskResultable for SendNotification {
    type Result = ();

    fn downcast(_: django_rs::tasks::task::TaskResult) -> Self::Result {}
}
