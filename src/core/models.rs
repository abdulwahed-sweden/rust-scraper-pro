use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ScrapedData {
    pub id: String,
    pub source: String,
    pub url: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub price: Option<f64>,
    pub image_url: Option<String>,
    pub author: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub metadata: HashMap<String, String>,
    pub category: Option<String>,
}

impl ScrapedData {
    pub fn new(source: String, url: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            source,
            url,
            title: None,
            content: None,
            price: None,
            image_url: None,
            author: None,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
            category: None,
        }
    }

    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    pub fn with_price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrapingConfig {
    pub rate_limit_ms: u64,
    pub timeout_seconds: u64,
    pub max_retries: u32,
    pub user_agent: String,
    pub follow_robots_txt: bool,
}

impl Default for ScrapingConfig {
    fn default() -> Self {
        Self {
            rate_limit_ms: 1000,
            timeout_seconds: 30,
            max_retries: 3,
            user_agent: "RustScraperPro/1.0".to_string(),
            follow_robots_txt: true,
        }
    }
}