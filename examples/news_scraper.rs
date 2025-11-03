//! Realistic News Scraper Example
//!
//! Demonstrates scraping from Hacker News, a real public news aggregator.
//! This example shows:
//! - Scraping real news content from news.ycombinator.com
//! - Processing and validating scraped data
//! - Exporting to JSON and CSV formats
//! - Rate limiting and polite scraping practices
//!
//! Usage: cargo run --example news_scraper

use anyhow::Result;
use rust_scraper_pro::{
    core::{
        config::Config,
        scraper::ScraperEngine,
    },
    output::{
        json::JsonOutput,
        csv::CsvOutput,
    },
    processors::pipeline::ProcessingPipeline,
    sources::{NewsSource, Source},
    utils::{
        cache::HtmlCache,
        logger::setup_logger,
    },
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    setup_logger()?;

    println!("\n🗞️  Rust Scraper Pro - Hacker News Example");
    println!("==========================================\n");

    // Load configuration (with realistic rate limiting)
    let config = Config::load("config/settings.toml").await?;
    println!("✓ Configuration loaded");
    println!("  Rate limit: {}ms between requests", config.scraping.rate_limit_ms);
    println!("  User agent: {}", config.scraping.user_agent);

    // Initialize HTML cache for efficiency
    let cache = Arc::new(HtmlCache::new_html_cache(100, 3600));
    println!("✓ Cache initialized (100 items, 1h TTL)\n");

    // Create processing pipeline
    let pipeline = ProcessingPipeline::new();

    // Initialize scraper engine
    let mut engine = ScraperEngine::new(config, pipeline, Some(cache.clone()));

    // Configure Hacker News source with realistic selectors
    let hacker_news = NewsSource::new("https://news.ycombinator.com/")
        .with_name("Hacker News");

    println!("📡 Scraping from: {}", hacker_news.name());
    println!("   URL: {}\n", hacker_news.base_url());

    // Perform scraping
    match engine.scrape_source(hacker_news).await {
        Ok(data) => {
            println!("✓ Scraped {} raw items", data.len());

            // Process data through validation and deduplication pipeline
            println!("\n🔄 Processing data through pipeline...");
            let processed_data = engine.process_data(data).await?;
            println!("✓ Pipeline complete: {} items after processing\n", processed_data.len());

            if processed_data.is_empty() {
                println!("⚠️  No data to export (all filtered out or no matches found)");
                return Ok(());
            }

            // Show sample of scraped data
            println!("📊 Sample of scraped content:");
            println!("─────────────────────────────────────────────────────");
            for (idx, item) in processed_data.iter().take(3).enumerate() {
                println!("\n[{}] {}", idx + 1, item.title.as_ref().unwrap_or(&"(no title)".to_string()));
                if let Some(author) = &item.author {
                    println!("    By: {}", author);
                }
                println!("    URL: {}", item.url);
            }
            println!("\n─────────────────────────────────────────────────────\n");

            // Export to JSON
            println!("💾 Exporting to JSON...");
            let json_output = JsonOutput::new();
            json_output.export(&processed_data, "output/hacker_news.json").await?;
            println!("✓ Saved to: output/hacker_news.json");

            // Export to CSV
            println!("💾 Exporting to CSV...");
            let csv_output = CsvOutput::new();
            csv_output.export(&processed_data, "output/hacker_news.csv").await?;
            println!("✓ Saved to: output/hacker_news.csv");

            // Display cache statistics
            let stats = cache.stats();
            println!("\n📈 Cache Statistics:");
            println!("   Entries: {}", stats.entry_count);
            println!("   Hit rate: {:.1}%", stats.hit_rate * 100.0);

            println!("\n✅ News scraping completed successfully!");
            println!("   {} articles scraped and exported\n", processed_data.len());
        }
        Err(e) => {
            eprintln!("❌ Scraping failed: {}", e);
            eprintln!("   This might be due to network issues or site changes.");
            eprintln!("   Try again later or check the selectors in config/settings.toml");
        }
    }

    Ok(())
}
