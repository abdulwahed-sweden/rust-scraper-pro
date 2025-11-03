use crate::{
    core::models::ScrapedData,
    core::scraper::ScraperEngine,
    sources::source::{CustomSource, Source},
};
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CustomConfig {
    pub name: String,
    pub base_url: String,
    pub selectors: CustomSelectors,
}

#[derive(Debug, Deserialize)]
pub struct CustomSelectors {
    pub container: String,
    pub fields: Vec<FieldSelector>,
}

#[derive(Debug, Deserialize)]
pub struct FieldSelector {
    pub name: String,
    pub selector: String,
    pub attribute: Option<String>,
}

impl CustomSource {
    pub fn new(base_url: &str, name: &str) -> Self {
        Self {
            name: name.to_string(),
            base_url: base_url.to_string(),
            selectors: Vec::new(),
        }
    }

    pub fn with_selectors(mut self, selectors: Vec<String>) -> Self {
        self.selectors = selectors;
        self
    }

    pub fn from_config(config: CustomConfig) -> Self {
        Self {
            name: config.name,
            base_url: config.base_url,
            selectors: Vec::new(), // Would map from config
        }
    }
}

#[async_trait::async_trait]
impl Source for CustomSource {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn scrape(&self, html: &str) -> Result<Vec<ScrapedData>> {
        let document = ScraperEngine::parse_html(html);
        let mut results = Vec::new();

        // Use custom selectors if provided, otherwise use generic approach
        if !self.selectors.is_empty() {
            // Custom selector logic would go here
            for selector in &self.selectors {
                if let Ok(elements) = ScraperEngine::select_element(&document, selector) {
                    for element in elements {
                        let data = ScrapedData::new(self.name().to_string(), self.base_url().to_string())
                            .with_content(element);
                        results.push(data);
                    }
                }
            }
        } else {
            // Generic scraping approach
            let generic_selectors = vec!["article", "div", "section", "main"];
            
            for selector in generic_selectors {
                if let Ok(elements) = ScraperEngine::select_element(&document, selector) {
                    for element in elements {
                        if element.len() > 10 { // Basic content length filter
                            let data = ScrapedData::new(self.name().to_string(), self.base_url().to_string())
                                .with_content(element);
                            results.push(data);
                        }
                    }
                }
            }
        }

        log::info!("Scraped {} items from custom source {}", results.len(), self.name());
        Ok(results)
    }
}