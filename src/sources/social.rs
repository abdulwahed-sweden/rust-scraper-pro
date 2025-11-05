use crate::{
    core::models::ScrapedData,
    core::scraper::ScraperEngine,
    sources::source::{SocialSource, Source},
};
use anyhow::Result;
use chrono::{DateTime, Utc};
use regex::Regex;
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    static ref MENTION_REGEX: Regex = Regex::new(r"@(\w+)").unwrap();
    static ref HASHTAG_REGEX: Regex = Regex::new(r"#(\w+)").unwrap();
    static ref URL_REGEX: Regex = Regex::new(r"https?://[^\s]+").unwrap();
}

#[derive(Debug, Deserialize)]
pub struct SocialMediaConfig {
    pub platform: String,
    pub base_url: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
}

impl SocialSource {
    pub fn new(base_url: &str) -> Self {
        Self {
            name: "Social Media Source".to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub fn twitter() -> Self {
        Self {
            name: "Twitter".to_string(),
            base_url: "https://twitter.com".to_string(),
        }
    }

    pub fn reddit() -> Self {
        Self {
            name: "Reddit".to_string(),
            base_url: "https://reddit.com".to_string(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

#[async_trait::async_trait]
impl Source for SocialSource {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn scrape(&self, html: &str) -> Result<Vec<ScrapedData>> {
        let document = ScraperEngine::parse_html(html);
        let mut results = Vec::new();

        // Different scraping logic based on platform
        match self.name.as_str() {
            "Twitter" => self.scrape_twitter(&document, &mut results)?,
            "Reddit" => self.scrape_reddit(&document, &mut results)?,
            _ => self.scrape_generic_social(&document, &mut results)?,
        }

        log::info!("Scraped {} social posts from {}", results.len(), self.name());
        Ok(results)
    }
}

impl SocialSource {
    fn scrape_twitter(&self, document: &scraper::Html, results: &mut Vec<ScrapedData>) -> Result<()> {
        // Twitter-specific scraping logic
        let tweet_selector = "[data-testid=\"tweet\"]";
        let content_selector = "[data-testid=\"tweetText\"]";
        let author_selector = "[data-testid=\"User-Name\"]";
        let timestamp_selector = "time";

        let tweets = ScraperEngine::select_element(document, tweet_selector)?;

        for _tweet in tweets {
            let mut data = ScrapedData::new(self.name().to_string(), self.base_url().to_string());
            
            // Extract content
            if let Ok(content) = ScraperEngine::select_element(document, content_selector) {
                if let Some(first_content) = content.get(0) {
                    data.content = Some(first_content.clone());
                    
                    // Extract mentions and hashtags
                    let mentions: Vec<String> = MENTION_REGEX
                        .captures_iter(first_content)
                        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                        .collect();
                    
                    let hashtags: Vec<String> = HASHTAG_REGEX
                        .captures_iter(first_content)
                        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                        .collect();

                    data.metadata.insert("mentions".to_string(), mentions.join(","));
                    data.metadata.insert("hashtags".to_string(), hashtags.join(","));
                }
            }

            // Extract author
            if let Ok(authors) = ScraperEngine::select_element(document, author_selector) {
                if let Some(author) = authors.get(0) {
                    data.author = Some(author.clone());
                }
            }

            // Extract timestamp
            if let Ok(timestamps) = ScraperEngine::select_element(document, timestamp_selector) {
                if let Some(timestamp) = timestamps.get(0) {
                    if let Ok(parsed_time) = DateTime::parse_from_rfc3339(timestamp) {
                        data.timestamp = parsed_time.with_timezone(&Utc);
                    }
                }
            }

            results.push(data);
        }

        Ok(())
    }

    fn scrape_reddit(&self, document: &scraper::Html, results: &mut Vec<ScrapedData>) -> Result<()> {
        // Reddit-specific scraping logic
        let post_selector = "[data-testid=\"post-container\"]";
        let title_selector = "h3";
        let content_selector = "[data-testid=\"post-content\"]";
        let author_selector = "[data-testid=\"post_author_link\"]";
        let score_selector = "[data-testid=\"post-score\"]";

        let posts = ScraperEngine::select_element(document, post_selector)?;

        for _post in posts {
            let mut data = ScrapedData::new(self.name().to_string(), self.base_url().to_string());
            
            // Extract title
            if let Ok(titles) = ScraperEngine::select_element(document, title_selector) {
                if let Some(title) = titles.get(0) {
                    data.title = Some(title.clone());
                }
            }

            // Extract content
            if let Ok(contents) = ScraperEngine::select_element(document, content_selector) {
                if let Some(content) = contents.get(0) {
                    data.content = Some(content.clone());
                }
            }

            // Extract author
            if let Ok(authors) = ScraperEngine::select_element(document, author_selector) {
                if let Some(author) = authors.get(0) {
                    data.author = Some(author.clone());
                }
            }

            // Extract score
            if let Ok(scores) = ScraperEngine::select_element(document, score_selector) {
                if let Some(score) = scores.get(0) {
                    if let Ok(score_num) = score.parse::<i32>() {
                        data.metadata.insert("score".to_string(), score_num.to_string());
                    }
                }
            }

            data.metadata.insert("platform".to_string(), "reddit".to_string());
            results.push(data);
        }

        Ok(())
    }

    fn scrape_generic_social(&self, document: &scraper::Html, results: &mut Vec<ScrapedData>) -> Result<()> {
        // Generic social media scraping
        let post_selector = "article, .post, .card, .feed-item";
        let _content_selector = ".content, .text, .body";
        let author_selector = ".author, .user, .username";

        let posts = ScraperEngine::select_element(document, post_selector)?;

        for post in posts {
            let mut data = ScrapedData::new(self.name().to_string(), self.base_url().to_string());
            data.content = Some(post.clone());
            
            // Try to extract additional metadata
            if let Ok(authors) = ScraperEngine::select_element(document, author_selector) {
                if let Some(author) = authors.get(0) {
                    data.author = Some(author.clone());
                }
            }

            results.push(data);
        }

        Ok(())
    }
}