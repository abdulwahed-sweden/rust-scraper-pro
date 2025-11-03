use crate::core::models::ScrapedData;
use anyhow::Result;
use std::collections::HashSet;

pub struct Deduplicator;

impl Deduplicator {
    pub fn new() -> Self {
        Self
    }

    pub async fn deduplicate(&self, data: Vec<ScrapedData>) -> Result<Vec<ScrapedData>> {
        let mut seen_urls = HashSet::new();
        let mut seen_titles = HashSet::new();
        let mut seen_contents = HashSet::new();
        let mut deduplicated = Vec::new();

        for item in data {
            let url_key = item.url.to_lowercase();
            let title_key = item.title.as_ref().map(|t| t.to_lowercase());
            let content_key = item.content.as_ref().map(|c| c.to_lowercase());

            // Skip if we've seen this URL, title, or content before
            if seen_urls.contains(&url_key) {
                continue;
            }

            if let Some(ref title) = title_key {
                if seen_titles.contains(title) {
                    continue;
                }
            }

            if let Some(ref content) = content_key {
                if content.len() > 50 && seen_contents.contains(content) { // Only check longer contents
                    continue;
                }
            }

            seen_urls.insert(url_key);
            if let Some(title) = title_key {
                seen_titles.insert(title);
            }
            if let Some(content) = content_key {
                if content.len() > 50 {
                    seen_contents.insert(content);
                }
            }

            deduplicated.push(item);
        }

        log::info!("Deduplication completed: {} unique items", deduplicated.len());
        Ok(deduplicated)
    }
}