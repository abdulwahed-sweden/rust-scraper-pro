use crate::core::models::ScrapedData;
use anyhow::Result;

pub struct Validator;

impl Validator {
    pub fn new() -> Self {
        Self
    }

    pub async fn validate(&self, data: Vec<ScrapedData>) -> Result<Vec<ScrapedData>> {
        let validated: Vec<ScrapedData> = data
            .into_iter()
            .filter(|item| self.is_valid_item(item))
            .collect();

        log::info!("Validation completed: {} valid items", validated.len());
        Ok(validated)
    }

    fn is_valid_item(&self, item: &ScrapedData) -> bool {
        // Check if item has at least a title or content
        if item.title.is_none() && item.content.is_none() {
            log::debug!("Item invalid: missing both title and content");
            return false;
        }

        // Check if URL is valid
        if !self.is_valid_url(&item.url) {
            log::debug!("Item invalid: invalid URL {}", item.url);
            return false;
        }

        // Check if price is reasonable if present
        if let Some(price) = item.price {
            if price < 0.0 || price > 1_000_000.0 {
                log::debug!("Item invalid: unreasonable price {}", price);
                return false;
            }
        }

        // Check content length if present
        if let Some(content) = &item.content {
            if content.trim().len() < 3 {
                log::debug!("Item invalid: content too short");
                return false;
            }
        }

        true
    }

    fn is_valid_url(&self, url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }
}