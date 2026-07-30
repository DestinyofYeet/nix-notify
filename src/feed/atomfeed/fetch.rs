use itertools::Itertools;

use crate::feed::{
    atomfeed::{AtomError, AtomFeed},
    feeditem::FeedItem,
};

impl AtomFeed {
    pub fn fetch(&self) -> Result<Vec<FeedItem>, AtomError> {
        let content =
            reqwest::blocking::get(&self.url).map_err(|e| AtomError::Fetch(e.to_string()))?;

        let content = content
            .error_for_status()
            .map_err(|e| AtomError::Fetch(e.to_string()))?;

        let bytes = content
            .bytes()
            .map_err(|e| AtomError::Bytes(e.to_string()))?;

        let feed =
            feed_rs::parser::parse(&bytes[..]).map_err(|e| AtomError::Channel(e.to_string()))?;

        let maybe_items: Vec<Result<FeedItem, AtomError>> = feed
            .entries
            .into_iter()
            .map(|entry| {
                let title = match entry.title {
                    Some(value) => value.content,
                    None => return Err(AtomError::Item("No title found".to_string())),
                };

                let author = match entry.authors.first() {
                    None => return Err(AtomError::Item("No author found".to_string())),
                    Some(value) => value.name.clone(),
                };

                let commit = match entry.id.strip_prefix("tag:github.com,2008:Grit::Commit/") {
                    Some(value) => value.to_string(),
                    None => return Err(AtomError::Item("Failed to strip id".to_string())),
                };

                let updated = match entry.updated {
                    Some(value) => value,
                    None => return Err(AtomError::Item("No updated time found".to_string())),
                };

                let link = match entry.links.first() {
                    Some(value) => value.href.to_string(),
                    None => return Err(AtomError::Item("No link found".to_string())),
                };

                let split = title.split(":").collect_vec();

                let message = match split.last() {
                    Some(value) => value.to_string(),
                    None => return Err(AtomError::Item("Failed to find message".to_string())),
                };

                let package = match split.first() {
                    Some(value) => value.to_string(),
                    None => return Err(AtomError::Item("Failed to find package".to_string())),
                };

                Ok(FeedItem::new(
                    self.name.clone(),
                    message,
                    commit,
                    updated,
                    author,
                    link,
                    package,
                ))
            })
            .collect_vec();

        let mut items: Vec<FeedItem> = Vec::new();

        for item in maybe_items.into_iter() {
            items.push(item?);
        }

        Ok(items)
    }
}
