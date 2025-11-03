use async_trait::async_trait;
use crate::core::models::ScrapedData;
use anyhow::Result;

#[async_trait]
pub trait Source: Send + Sync {
    fn name(&self) -> &str;
    fn base_url(&self) -> &str;
    async fn scrape(&self, html: &str) -> Result<Vec<ScrapedData>>;
}

pub enum SourceType {
    News(NewsSource),
    Ecommerce(EcommerceSource),
    Social(SocialSource),
    Custom(CustomSource),
}

impl Source for SourceType {
    fn name(&self) -> &str {
        match self {
            SourceType::News(source) => source.name(),
            SourceType::Ecommerce(source) => source.name(),
            SourceType::Social(source) => source.name(),
            SourceType::Custom(source) => source.name(),
        }
    }

    fn base_url(&self) -> &str {
        match self {
            SourceType::News(source) => source.base_url(),
            SourceType::Ecommerce(source) => source.base_url(),
            SourceType::Social(source) => source.base_url(),
            SourceType::Custom(source) => source.base_url(),
        }
    }

    async fn scrape(&self, html: &str) -> Result<Vec<ScrapedData>> {
        match self {
            SourceType::News(source) => source.scrape(html).await,
            SourceType::Ecommerce(source) => source.scrape(html).await,
            SourceType::Social(source) => source.scrape(html).await,
            SourceType::Custom(source) => source.scrape(html).await,
        }
    }
}

// These will be implemented in their respective modules
pub struct NewsSource {
    name: String,
    base_url: String,
}

pub struct EcommerceSource {
    name: String,
    base_url: String,
}

pub struct SocialSource {
    name: String,
    base_url: String,
}

pub struct CustomSource {
    name: String,
    base_url: String,
    selectors: Vec<String>,
}