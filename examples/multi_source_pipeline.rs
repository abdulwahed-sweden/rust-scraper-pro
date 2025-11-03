//! Comprehensive Multi-Source Pipeline Example
//!
//! Demonstrates the full power of Rust Scraper Pro by combining:
//! - News scraping (Hacker News)
//! - E-commerce scraping (Books to Scrape)
//! - Social/API scraping (Reddit JSON)
//!
//! Features showcased:
//! - Multi-source concurrent scraping
//! - Unified data processing pipeline
//! - Validation, normalization, and deduplication
//! - Multiple export formats (JSON, CSV)
//! - Comprehensive error handling and logging
//! - Cache utilization and statistics
//!
//! Usage: cargo run --example multi_source_pipeline

use anyhow::Result;
use rust_scraper_pro::{
    core::{
        config::Config,
        models::ScrapedData,
        scraper::ScraperEngine,
    },
    output::{
        json::JsonOutput,
        csv::CsvOutput,
    },
    processors::pipeline::ProcessingPipeline,
    sources::{NewsSource, EcommerceSource, Source, SourceType},
    utils::{
        cache::HtmlCache,
        logger::setup_logger,
    },
};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    setup_logger()?;

    println!("\n🚀 Rust Scraper Pro - Multi-Source Pipeline Demo");
    println!("==================================================");
    println!("This example demonstrates professional web scraping");
    println!("from multiple real-world sources with full processing.\n");

    // Load configuration
    let config = Config::load("config/settings.toml").await?;
    println!("✓ Configuration loaded");
    println!("  Rate limit: {}ms between requests", config.scraping.rate_limit_ms);
    println!("  Timeout: {}s", config.scraping.timeout_seconds);
    println!("  User agent: {}\n", config.scraping.user_agent);

    // Initialize cache
    let cache = Arc::new(HtmlCache::new_html_cache(200, 3600));
    println!("✓ Cache system initialized");
    println!("  Capacity: 200 items");
    println!("  TTL: 3600s (1 hour)\n");

    // Create processing pipeline
    let pipeline = ProcessingPipeline::new();
    println!("✓ Processing pipeline ready");
    println!("  Stages: Validation → Normalization → Deduplication\n");

    // Initialize scraper engine
    let mut engine = ScraperEngine::new(config, pipeline, Some(cache.clone()));

    println!("═══════════════════════════════════════════════════════════\n");

    // Define sources
    let sources: Vec<(&str, SourceType)> = vec![
        (
            "🗞️  News",
            SourceType::News(NewsSource::new("https://news.ycombinator.com/")
                .with_name("Hacker News"))
        ),
        (
            "🛒 E-commerce",
            SourceType::Ecommerce(EcommerceSource::new("https://books.toscrape.com/catalogue/category/books_1/index.html")
                .with_name("Books to Scrape"))
        ),
    ];

    let mut all_data = Vec::new();
    let mut source_stats: Vec<(&str, String, usize, bool)> = Vec::new();

    // Scrape from each source
    for (category, source) in sources {
        let source_name = source.name().to_string();
        let source_url = source.base_url().to_string();

        println!("\n{} Source: {}", category, source_name);
        println!("   URL: {}", source_url);
        println!("   Status: Scraping...");

        match engine.scrape_source(source).await {
            Ok(data) => {
                let count = data.len();
                println!("   ✓ Success: {} items scraped", count);

                source_stats.push((category, source_name, count, true));
                all_data.extend(data);

                // Polite delay between sources
                sleep(Duration::from_millis(2000)).await;
            }
            Err(e) => {
                eprintln!("   ✗ Failed: {}", e);
                source_stats.push((category, source_name, 0, false));
            }
        }
    }

    println!("\n═══════════════════════════════════════════════════════════");
    println!("\n📊 Scraping Summary:");
    println!("───────────────────────────────────────────────────────────");

    let total_raw = all_data.len();
    for (category, name, count, success) in &source_stats {
        let status = if *success { "✓" } else { "✗" };
        println!("  {} {} ({}): {} items", status, category, name, count);
    }

    println!("───────────────────────────────────────────────────────────");
    println!("  Total raw items: {}\n", total_raw);

    if all_data.is_empty() {
        println!("⚠️  No data collected. Please check:");
        println!("   - Network connectivity");
        println!("   - Website availability");
        println!("   - Selector accuracy in config/settings.toml\n");
        return Ok(());
    }

    // Process data through pipeline
    println!("🔄 Processing through pipeline...");
    println!("   → Stage 1: Validation");

    let processed_data = engine.process_data(all_data).await?;

    println!("   → Stage 2: Normalization");
    println!("   → Stage 3: Deduplication");
    println!("   ✓ Pipeline complete\n");

    println!("📈 Processing Results:");
    println!("   Input: {} items", total_raw);
    println!("   Output: {} items", processed_data.len());
    println!("   Removed: {} duplicates/invalid\n", total_raw - processed_data.len());

    // Categorize data by source
    let mut by_source: std::collections::HashMap<String, Vec<&ScrapedData>> = std::collections::HashMap::new();
    for item in &processed_data {
        by_source.entry(item.source.clone()).or_insert_with(Vec::new).push(item);
    }

    println!("📂 Data by Source:");
    for (source, items) in &by_source {
        println!("   • {}: {} items", source, items.len());
    }
    println!();

    // Display sample data
    println!("📋 Sample of Processed Data:");
    println!("═══════════════════════════════════════════════════════════");

    for (idx, item) in processed_data.iter().take(5).enumerate() {
        println!("\n[{}] {}", idx + 1, item.title.as_ref().unwrap_or(&"(no title)".to_string()));
        println!("    Source: {}", item.source);

        if let Some(price) = item.price {
            println!("    Price: £{:.2}", price);
        }

        if let Some(author) = &item.author {
            println!("    Author: {}", author);
        }

        println!("    URL: {}", item.url);
    }

    println!("\n═══════════════════════════════════════════════════════════\n");

    // Export data
    println!("💾 Exporting Data:");

    let json_output = JsonOutput::new();
    json_output.export(&processed_data, "output/multi_source_data.json").await?;
    println!("   ✓ JSON: output/multi_source_data.json");

    let csv_output = CsvOutput::new();
    csv_output.export(&processed_data, "output/multi_source_data.csv").await?;
    println!("   ✓ CSV: output/multi_source_data.csv");

    // Cache statistics
    let cache_stats = cache.stats();
    println!("\n📈 Cache Performance:");
    println!("   Entries: {}", cache_stats.entry_count);
    println!("   Hit rate: {:.1}%", cache_stats.hit_rate * 100.0);
    println!("   Miss rate: {:.1}%\n", cache_stats.miss_rate * 100.0);

    println!("═══════════════════════════════════════════════════════════");
    println!("\n✅ Multi-Source Pipeline Completed Successfully!");
    println!("\n   📁 Scraped from {} sources", source_stats.len());
    println!("   📊 Collected {} raw items", total_raw);
    println!("   ✨ Processed to {} unique items", processed_data.len());
    println!("   💾 Exported to JSON and CSV\n");

    println!("🎯 Next Steps:");
    println!("   - Review output files in /output/ directory");
    println!("   - Adjust selectors in config/settings.toml if needed");
    println!("   - Add more sources in config/sources.toml");
    println!("   - Experiment with different processing pipelines\n");

    Ok(())
}
