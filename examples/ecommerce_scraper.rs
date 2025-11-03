//! Realistic E-Commerce Scraper Example
//!
//! Demonstrates scraping from Books to Scrape (https://books.toscrape.com),
//! a real training website designed for web scraping practice.
//!
//! This example shows:
//! - Scraping product listings (books with titles, prices, ratings)
//! - Handling e-commerce specific data (prices, availability, images)
//! - Data validation and normalization
//! - Export to multiple formats
//!
//! Usage: cargo run --example ecommerce_scraper

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
    sources::{EcommerceSource, Source},
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

    println!("\n🛒 Rust Scraper Pro - Books E-Commerce Example");
    println!("===============================================\n");

    // Load configuration
    let config = Config::load("config/settings.toml").await?;
    println!("✓ Configuration loaded");
    println!("  Target: Books to Scrape (Educational site)");
    println!("  Rate limit: {}ms\n", config.scraping.rate_limit_ms);

    // Initialize cache
    let cache = Arc::new(HtmlCache::new_html_cache(50, 1800));
    println!("✓ Cache initialized\n");

    // Create processing pipeline
    let pipeline = ProcessingPipeline::new();

    // Initialize scraper engine
    let mut engine = ScraperEngine::new(config, pipeline, Some(cache));

    // Configure Books to Scrape source
    // This is a real website specifically designed for scraping practice
    let books_source = EcommerceSource::new("https://books.toscrape.com/catalogue/category/books_1/index.html")
        .with_name("Books to Scrape");

    println!("📡 Scraping from: {}", books_source.name());
    println!("   Category: All Books");
    println!("   URL: {}\n", books_source.base_url());

    println!("⏳ Fetching product data...");

    // Perform scraping
    match engine.scrape_source(books_source).await {
        Ok(data) => {
            println!("✓ Scraped {} products\n", data.len());

            // Process data
            println!("🔄 Processing through validation pipeline...");
            let processed_data = engine.process_data(data).await?;
            println!("✓ {} products after processing\n", processed_data.len());

            if processed_data.is_empty() {
                println!("⚠️  No products found. The site structure may have changed.");
                return Ok(());
            }

            // Display sample products
            println!("📚 Sample of scraped books:");
            println!("═══════════════════════════════════════════════════════════");
            for (idx, item) in processed_data.iter().take(5).enumerate() {
                println!("\n[{}] {}", idx + 1, item.title.as_ref().unwrap_or(&"Unknown".to_string()));

                if let Some(price) = item.price {
                    println!("    💰 Price: £{:.2}", price);
                }

                if let Some(img) = &item.image_url {
                    println!("    🖼️  Image: {}", img);
                }

                println!("    🔗 URL: {}", item.url);
            }
            println!("\n═══════════════════════════════════════════════════════════\n");

            // Calculate statistics
            let total_value: f64 = processed_data.iter()
                .filter_map(|item| item.price)
                .sum();
            let avg_price = if !processed_data.is_empty() {
                total_value / processed_data.len() as f64
            } else {
                0.0
            };

            println!("📊 Statistics:");
            println!("   Total products: {}", processed_data.len());
            println!("   Average price: £{:.2}", avg_price);
            println!("   Total value: £{:.2}\n", total_value);

            // Export to JSON
            println!("💾 Exporting data...");
            let json_output = JsonOutput::new();
            json_output.export(&processed_data, "output/books_catalog.json").await?;
            println!("✓ JSON: output/books_catalog.json");

            // Export to CSV
            let csv_output = CsvOutput::new();
            csv_output.export(&processed_data, "output/books_catalog.csv").await?;
            println!("✓ CSV: output/books_catalog.csv");

            println!("\n✅ E-commerce scraping completed successfully!");
            println!("   {} books scraped and cataloged\n", processed_data.len());
        }
        Err(e) => {
            eprintln!("❌ Scraping failed: {}", e);
            eprintln!("   Possible causes:");
            eprintln!("   - Network connectivity issues");
            eprintln!("   - Website structure changed");
            eprintln!("   - Rate limiting or blocking");
            eprintln!("\n   Try adjusting rate_limit_ms in config/settings.toml");
        }
    }

    Ok(())
}
