use crate::{
    core::models::ScrapedData,
    core::scraper::ScraperEngine,
    sources::source::{NewsSource, Source},
};
use anyhow::Result;
use scraper::Html;

impl NewsSource {
    pub fn new(base_url: &str) -> Self {
        Self {
            name: "News Source".to_string(),
            base_url: base_url.to_string(),
        }
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

#[async_trait::async_trait]
impl Source for NewsSource {
    fn name(&self) -> &str {
        &self.name
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn scrape(&self, html: &str) -> Result<Vec<ScrapedData>> {
        let document = ScraperEngine::parse_html(html);
        let mut results = Vec::new();

        // Common news website selectors
        let article_selector = "article, .story, .news-item, .post";
        let title_selector = "h1, h2, h3, .title, .headline, h1 > a, h2 > a, h3 > a";
        let content_selector = "p, .content, .article-body, .summary";
        let author_selector = ".author, .byline, .writer";
        let date_selector = ".date, .time, .published";

        let articles = ScraperEngine::select_element(&document, article_selector)?;

        for article in articles {
            let mut data = ScrapedData::new(self.name().to_string(), self.base_url().to_string());
            
            // Extract title
            if let Ok(titles) = ScraperEngine::select_element(&document, title_selector) {
                if let Some(title) = titles.get(0) {
                    data.title = Some(title.clone());
                }
            }

            // Extract content
            if let Ok(contents) = ScraperEngine::select_element(&document, content_selector) {
                if let Some(content) = contents.get(0) {
                    data.content = Some(content.clone());
                }
            }

            // Extract author
            if let Ok(authors) = ScraperEngine::select_element(&document, author_selector) {
                if let Some(author) = authors.get(0) {
                    data.author = Some(author.clone());
                }
            }

            // Extract date if available
            if let Ok(dates) = ScraperEngine::select_element(&document, date_selector) {
                if let Some(date) = dates.get(0) {
                    data.metadata.insert("publish_date".to_string(), date.clone());
                }
            }

            results.push(data);
        }

        log::info!("Scraped {} news articles from {}", results.len(), self.name());
        Ok(results)
    }
}