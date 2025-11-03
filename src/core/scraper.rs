use crate::{
    core::models::{ScrapedData, ScrapingConfig},
    processors::pipeline::ProcessingPipeline,
    sources::source::Source,
    utils::{error::ScraperError, rate_limiter::RateLimiter, cache::HtmlCache},
};
use anyhow::Result;
use scraper::{Html, Selector};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct ScraperEngine {
    config: ScrapingConfig,
    pipeline: ProcessingPipeline,
    rate_limiter: Arc<Mutex<RateLimiter>>,
    client: reqwest::Client,
    cache: Option<Arc<HtmlCache>>,
}

impl ScraperEngine {
    pub fn new(
        config: crate::core::config::Config,
        pipeline: ProcessingPipeline,
        cache: Option<Arc<HtmlCache>>,
    ) -> Self {
        let scraping_config = config.scraping;
        let client = reqwest::Client::builder()
            .user_agent(&scraping_config.user_agent)
            .timeout(std::time::Duration::from_secs(scraping_config.timeout_seconds))
            .build()
            .unwrap();

        let rate_limit_ms = scraping_config.rate_limit_ms;

        Self {
            config: scraping_config,
            pipeline,
            rate_limiter: Arc::new(Mutex::new(RateLimiter::new(rate_limit_ms))),
            client,
            cache,
        }
    }

    pub async fn scrape_source(&mut self, source: impl Source) -> Result<Vec<ScrapedData>> {
        log::info!("Starting to scrape from: {}", source.name());
        
        // Apply rate limiting
        self.rate_limiter.lock().await.wait().await;
        
        let html_content = self.fetch_url_with_cache(source.base_url()).await?;
        let scraped_data = source.scrape(&html_content).await?;
        
        Ok(scraped_data)
    }

    pub async fn process_data(&self, data: Vec<ScrapedData>) -> Result<Vec<ScrapedData>> {
        self.pipeline.process(data).await
    }

    async fn fetch_url_with_cache(&self, url: &str) -> Result<String> {
        // Check cache first
        if let Some(cache) = &self.cache {
            if let Some(cached_html) = cache.get_html(url).await {
                log::debug!("Cache hit for URL: {}", url);
                return Ok(cached_html);
            }
        }

        log::debug!("Fetching URL: {}", url);
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(ScraperError::RequestError)?;

        if !response.status().is_success() {
            return Err(ScraperError::HttpError(response.status()).into());
        }

        let content = response
            .text()
            .await
            .map_err(ScraperError::RequestError)?;

        // Store in cache
        if let Some(cache) = &self.cache {
            if let Err(e) = cache.set_html(url, &content).await {
                log::warn!("Failed to cache HTML for {}: {}", url, e);
            }
        }

        Ok(content)
    }

    pub fn parse_html(html: &str) -> Html {
        Html::parse_document(html)
    }

    pub fn select_element<'a>(html: &'a Html, selector: &str) -> Result<Vec<String>> {
        let selector = Selector::parse(selector).map_err(|e| ScraperError::SelectorError(e.to_string()))?;
        let elements = html.select(&selector);
        
        let results: Vec<String> = elements
            .filter_map(|element| {
                let text = element.text().collect::<Vec<&str>>().join(" ");
                if text.trim().is_empty() {
                    None
                } else {
                    Some(text.trim().to_string())
                }
            })
            .collect();
            
        Ok(results)
    }

    pub fn select_attribute<'a>(html: &'a Html, selector: &str, attribute: &str) -> Result<Vec<String>> {
        let selector = Selector::parse(selector).map_err(|e| ScraperError::SelectorError(e.to_string()))?;
        let elements = html.select(&selector);
        
        let results: Vec<String> = elements
            .filter_map(|element| {
                element.value().attr(attribute).map(|attr| attr.to_string())
            })
            .collect();
            
        Ok(results)
    }
}