use crate::{
    core::models::ScrapedData,
    core::scraper::ScraperEngine,
    sources::source::{EcommerceSource, Source},
};
use anyhow::Result;
use regex::Regex;
use lazy_static::lazy_static;

impl EcommerceSource {
    pub fn new(base_url: &str) -> Self {
        Self {
            name: "Ecommerce Source".to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

#[async_trait::async_trait]
impl Source for EcommerceSource {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn scrape(&self, html: &str) -> Result<Vec<ScrapedData>> {
        let document = ScraperEngine::parse_html(html);
        let mut results = Vec::new();

        lazy_static! {
            static ref PRICE_REGEX: Regex = Regex::new(r#"\$?(\d+[.,]?\d*)"#).unwrap();
        }

        // E-commerce product selectors
        let product_selector = ".product, .item, [data-product], .card, .goods";
        let title_selector = ".title, .name, .product-name, h1, h2, h3";
        let price_selector = ".price, .cost, [data-price], .amount, .current-price";
        let image_selector = "img[src], .image, .product-image";
        let description_selector = ".description, .details, .product-desc";

        let products = ScraperEngine::select_element(&document, product_selector)?;

        for product in products {
            let mut data = ScrapedData::new(self.name().to_string(), self.base_url().to_string());
            
            // Extract product title
            if let Ok(titles) = ScraperEngine::select_element(&document, title_selector) {
                if let Some(title) = titles.get(0) {
                    data.title = Some(title.clone());
                }
            }

            // Extract price
            if let Ok(prices) = ScraperEngine::select_element(&document, price_selector) {
                if let Some(price_text) = prices.get(0) {
                    if let Some(captures) = PRICE_REGEX.captures(price_text) {
                        if let Some(price_match) = captures.get(1) {
                            let price_str = price_match.as_str().replace(',', "");
                            if let Ok(price) = price_str.parse::<f64>() {
                                data.price = Some(price);
                                data.metadata.insert("price_text".to_string(), price_text.clone());
                            }
                        }
                    }
                }
            }

            // Extract image URL
            if let Ok(images) = ScraperEngine::select_attribute(&document, image_selector, "src") {
                if let Some(image_url) = images.get(0) {
                    data.image_url = Some(image_url.clone());
                }
            }

            // Extract description
            if let Ok(descriptions) = ScraperEngine::select_element(&document, description_selector) {
                if let Some(description) = descriptions.get(0) {
                    data.content = Some(description.clone());
                }
            }

            results.push(data);
        }

        log::info!("Scraped {} products from {}", results.len(), self.name());
        Ok(results)
    }
}