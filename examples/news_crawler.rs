use anyhow::Result;
use rust_scraper_pro::prelude::*;
use rust_scraper_pro::{
    sources::news::NewsSource,
    sources::source::SourceType,
    utils::logger::setup_logger,
};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;
    
    log::info!("Starting News Crawler Example");
    
    // Load configuration
    let config = Config::load("config/settings.toml").await?;
    
    // Initialize cache for news (shorter TTL for fresh content)
    let cache = Arc::new(HtmlCache::new_html_cache(300, 900)); // 300 items, 15 min TTL
    
    // Create processing pipeline
    let pipeline = ProcessingPipeline::new();
    
    // Initialize scraper engine
    let mut engine = ScraperEngine::new(config, pipeline, Some(cache.clone()));
    
    // Define news sources (using example URLs - replace with real news sites)
    let news_sources = vec![
        NewsSource::new("https://news.ycombinator.com").with_name("Hacker News"),
        NewsSource::new("https://httpbin.org/html").with_name("Example News"),
        // Add real news URLs here:
        // NewsSource::new("https://example-news.com").with_name("Real News Site"),
    ];
    
    let mut all_articles = Vec::new();
    
    for source in news_sources {
        let source_type = SourceType::News(source);
        log::info!("📰 Scraping news from: {}", source_type.name());
        
        match engine.scrape_source(source_type).await {
            Ok(mut articles) => {
                log::info!("✅ Found {} articles", articles.len());
                
                // Add news-specific metadata
                for article in &mut articles {
                    article.add_metadata("content_type".to_string(), "news".to_string());
                    article.add_metadata("scraped_timestamp".to_string(), Utc::now().to_rfc3339());
                    
                    // Estimate article length
                    if let Some(content) = &article.content {
                        let word_count = content.split_whitespace().count();
                        article.add_metadata("word_count".to_string(), word_count.to_string());
                        
                        // Categorize by length
                        let length_category = if word_count < 100 {
                            "brief"
                        } else if word_count < 500 {
                            "standard"
                        } else {
                            "detailed"
                        };
                        article.add_metadata("length_category".to_string(), length_category.to_string());
                    }
                }
                
                all_articles.extend(articles);
            }
            Err(e) => {
                log::error!("❌ Failed to scrape news: {}", e);
            }
        }
        
        // Respectful delay between news sources
        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    
    // Process articles
    log::info!("🔄 Processing {} articles", all_articles.len());
    let processed_articles = engine.process_data(all_articles).await?;
    
    // Analyze news data
    analyze_news_data(&processed_articles);
    
    // Export news data
    let json_output = JsonOutput::new();
    json_output.export(&processed_articles, "output/news_articles.json").await?;
    
    let csv_output = CsvOutput::new();
    csv_output.export(&processed_articles, "output/news_articles.csv").await?;
    
    // Database export with news-specific table
    let db_output = SqliteOutput::new("sqlite:news.db", Some("articles")).await?;
    db_output.init().await?;
    let saved_count = db_output.save(&processed_articles).await?;
    
    log::info!("🎉 News crawling completed!");
    log::info!("💾 Saved {} articles to database", saved_count);
    
    Ok(())
}

fn analyze_news_data(articles: &[ScrapedData]) {
    log::info!("📊 News Data Analysis:");
    log::info!("   Total articles: {}", articles.len());
    
    // Count articles by source
    let mut source_counts: HashMap<String, usize> = HashMap::new();
    let mut articles_with_content = 0;
    let mut articles_with_author = 0;
    
    for article in articles {
        *source_counts.entry(article.source.clone()).or_insert(0) += 1;
        
        if article.content.is_some() {
            articles_with_content += 1;
        }
        
        if article.author.is_some() {
            articles_with_author += 1;
        }
    }
    
    log::info!("   Articles with content: {}", articles_with_content);
    log::info!("   Articles with author: {}", articles_with_author);
    
    for (source, count) in source_counts {
        log::info!("   {}: {} articles", source, count);
    }
    
    // Calculate average content length
    let total_words: usize = articles
        .iter()
        .filter_map(|a| a.content.as_ref())
        .map(|content| content.split_whitespace().count())
        .sum();
    
    let avg_word_count = if articles_with_content > 0 {
        total_words / articles_with_content
    } else {
        0
    };
    
    log::info!("   Average word count: {}", avg_word_count);
}