use crate::core::models::ScrapingConfig;
use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    pub url: String,
    pub selectors: Selectors,
    pub rate_limit_ms: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct Selectors {
    pub container: String,
    pub title: Option<String>,
    pub content: Option<String>,
    pub price: Option<String>,
    pub image: Option<String>,
    pub author: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub scraping: ScrapingConfig,
    pub sources: Vec<SourceConfig>,
}

impl AppConfig {
    pub async fn load<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

// Alias for backward compatibility
pub type Config = AppConfig;