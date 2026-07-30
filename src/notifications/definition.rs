use std::{collections::HashMap, sync::OnceLock};

use crate::config::ValidatedNotificationKind;

pub static NOTIFICATION_CONFIGS: OnceLock<HashMap<String, ValidatedNotificationKind>> =
    OnceLock::new();

pub struct SendNotification {
    pub(super) name: String,
    pub(super) recipient: String,

    pub(super) title: String,
    pub(super) text: String,
}
