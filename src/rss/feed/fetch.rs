use django_rs::chrono::{DateTime, Utc};
use itertools::Itertools;

use crate::rss::{error::RssError, feed::RssFeed, feeditem::RssItem};

impl RssFeed {
    pub fn fetch(&self) -> Result<Vec<RssItem>, RssError> {
        let content =
            reqwest::blocking::get(&self.url).map_err(|e| RssError::Fetch(e.to_string()))?;

        let content = content
            .error_for_status()
            .map_err(|e| RssError::Fetch(e.to_string()))?;

        let bytes = content
            .bytes()
            .map_err(|e| RssError::Bytes(e.to_string()))?;

        let feed =
            feed_rs::parser::parse(&bytes[..]).map_err(|e| RssError::Channel(e.to_string()))?;

        let maybe_items: Vec<Result<RssItem, RssError>> = feed
            .entries
            .into_iter()
            .map(|entry| {
                let author = match entry.authors.first() {
                    None => return Err(RssError::Item("No author found".to_string())),
                    Some(value) => value.name.clone(),
                };

                let message = match entry.title {
                    Some(value) => value.content,
                    None => return Err(RssError::Item("No title found".to_string())),
                };

                let commit = match entry.id.strip_prefix("tag:github.com,2008:Grit::Commit/") {
                    Some(value) => value.to_string(),
                    None => return Err(RssError::Item("Failed to strip id".to_string())),
                };

                let updated = match entry.updated {
                    Some(value) => value,
                    None => return Err(RssError::Item("No updated time found".to_string())),
                };

                let link = match entry.links.first() {
                    Some(value) => value.href.to_string(),
                    None => return Err(RssError::Item("No link found".to_string())),
                };

                Ok(RssItem::new(message, commit, updated, author, link))
            })
            .collect_vec();

        let mut items: Vec<RssItem> = Vec::new();

        for item in maybe_items.into_iter() {
            items.push(item?);
        }

        Ok(items)
    }
}
