use django_rs::models::traits::model::Model;
use django_rs::{
    chrono::{DateTime, Utc},
    django_rs_macro::{FromIter, SaveData},
};

#[derive(Debug, Clone, PartialEq, FromIter, SaveData)]
pub struct FeedItem {
    pub(super) id: Option<i64>,
    pub(super) feed_name: String,
    pub(super) message: String,
    pub(super) package: String,
    pub(super) commithash: String,
    pub(super) updated: DateTime<Utc>,
    pub(super) author: String,
    pub(super) link: String,
}
