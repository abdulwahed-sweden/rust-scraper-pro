use crate::core::models::ScrapedData;
use anyhow::Result;

pub struct Normalizer;

impl Normalizer {
    pub fn new() -> Self {
        Self
    }

    pub async fn normalize(&self, data: Vec<ScrapedData>) -> Result<Vec<ScrapedData>> {
        let mut normalized = Vec::new();

        for mut item in data {
            // Normalize title
            if let Some(title) = &item.title {
                item.title = Some(self.normalize_text(title));
            }

            // Normalize content
            if let Some(content) = &item.content {
                item.content = Some(self.normalize_text(content));
            }

            // Normalize author
            if let Some(author) = &item.author {
                item.author = Some(self.normalize_text(author));
            }

            // Normalize price format
            if let Some(price) = item.price {
                item.price = Some((price * 100.0).round() / 100.0); // Round to 2 decimal places
            }

            // Normalize URL if needed
            if !item.url.starts_with("http") {
                item.url = format!("https://{}", item.url);
            }

            normalized.push(item);
        }

        Ok(normalized)
    }

    fn normalize_text(&self, text: &str) -> String {
        text.trim()
            .chars()
            .filter(|c| c.is_ascii() || c.is_whitespace())
            .collect::<String>()
            .replace('\n', " ")
            .replace('\t', " ")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}