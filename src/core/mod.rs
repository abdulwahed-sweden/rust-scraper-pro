pub mod config;
pub mod models;
pub mod scraper;

pub use config::{AppConfig, Config, SourceConfig, Selectors};
pub use models::{ScrapedData, ScrapingConfig};
pub use scraper::ScraperEngine;
