use crate::{
    core::models::ScrapedData,
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
        use scraper::{Html, Selector};

        let document = Html::parse_document(html);
        let mut results = Vec::new();

        lazy_static! {
            static ref PRICE_REGEX: Regex = Regex::new(r#"[\$£€](\d+\.?\d*)"#).unwrap();
        }

        // Selectors optimized for books.toscrape.com
        let product_selector = Selector::parse("article.product_pod").unwrap();
        let title_selector = Selector::parse("h3 a").unwrap();
        let price_selector = Selector::parse("p.price_color").unwrap();
        let image_selector = Selector::parse("div.image_container img").unwrap();
        let availability_selector = Selector::parse("p.availability").unwrap();
        let rating_selector = Selector::parse("p.star-rating").unwrap();

        for product in document.select(&product_selector) {
            let mut data = ScrapedData::new(self.name().to_string(), self.base_url().to_string());

            // Extract product title and URL
            if let Some(title_elem) = product.select(&title_selector).next() {
                if let Some(title) = title_elem.value().attr("title") {
                    data.title = Some(title.to_string());
                }

                // Extract product URL
                if let Some(href) = title_elem.value().attr("href") {
                    // Resolve relative URL
                    let full_url = if href.starts_with("http") {
                        href.to_string()
                    } else if href.starts_with("../") {
                        format!("https://books.toscrape.com/catalogue/{}", href.trim_start_matches("../"))
                    } else {
                        format!("{}/{}", self.base_url().trim_end_matches('/'), href.trim_start_matches('/'))
                    };
                    data.url = full_url;
                }
            }

            // Extract price
            if let Some(price_elem) = product.select(&price_selector).next() {
                let price_text = price_elem.text().collect::<String>();
                if let Some(captures) = PRICE_REGEX.captures(&price_text) {
                    if let Some(price_match) = captures.get(1) {
                        if let Ok(price) = price_match.as_str().parse::<f64>() {
                            data.price = Some(price);
                            data.metadata.insert("price_text".to_string(), price_text.clone());
                            data.metadata.insert("currency".to_string(), "GBP".to_string());
                        }
                    }
                }
            }

            // Extract image URL
            if let Some(img_elem) = product.select(&image_selector).next() {
                if let Some(src) = img_elem.value().attr("src") {
                    // Resolve relative URL
                    let image_url = if src.starts_with("http") {
                        src.to_string()
                    } else {
                        format!("https://books.toscrape.com/{}", src.trim_start_matches("../").trim_start_matches('/'))
                    };
                    data.image_url = Some(image_url);
                }
            }

            // Extract availability
            if let Some(avail_elem) = product.select(&availability_selector).next() {
                let availability = avail_elem.text().collect::<String>().trim().to_string();
                data.metadata.insert("availability".to_string(), availability);
            }

            // Extract rating
            if let Some(rating_elem) = product.select(&rating_selector).next() {
                if let Some(rating_class) = rating_elem.value().attr("class") {
                    // Extract rating from class like "star-rating Three"
                    let rating = rating_class
                        .split_whitespace()
                        .find(|&word| word != "star-rating")
                        .unwrap_or("Unknown");
                    data.metadata.insert("rating".to_string(), rating.to_string());
                }
            }

            // Set category from source name if it contains category info
            if self.name().to_lowercase().contains("science") {
                data.category = Some("Science".to_string());
            } else {
                data.category = Some("Books".to_string());
            }

            results.push(data);
        }

        log::info!("Scraped {} products from {}", results.len(), self.name());
        Ok(results)
    }
}