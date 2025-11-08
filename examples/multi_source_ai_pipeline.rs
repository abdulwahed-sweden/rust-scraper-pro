//! Multi-Source AI Normalization Pipeline Demo
//!
//! This example demonstrates:
//! 1. Scraping from multiple book sources
//! 2. Storing raw data per source
//! 3. AI-powered normalization with DeepSeek
//! 4. Unified schema generation
//! 5. Production-ready data output
//!
//! Run with: cargo run --example multi_source_ai_pipeline

use anyhow::Result;
use rust_scraper_pro::{
    core::{config::Config, scraper::ScraperEngine},
    processors::pipeline::ProcessingPipeline,
    sources::{EcommerceSource, SourceType},
    ai::{DeepSeekClient, DataNormalizer},
    utils::cache::HtmlCache,
};
use std::sync::Arc;
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenvy::dotenv().ok();

    // Initialize logger
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("\n🚀 Multi-Source AI Normalization Pipeline");
    println!("═══════════════════════════════════════════════════\n");

    // Step 1: Initialize scraping engine
    println!("📦 Step 1: Initializing Scraper Engine...");
    let config = Config::load("config/settings.toml").await?;
    let pipeline = ProcessingPipeline::new();
    let cache = Arc::new(HtmlCache::new_html_cache(1000, 3600));
    let mut engine = ScraperEngine::new(config, pipeline, Some(cache));
    println!("✅ Scraper engine ready\n");

    // Step 2: Define multiple book sources
    println!("📚 Step 2: Configuring Multiple Data Sources...");
    let sources = vec![
        (
            "Books to Scrape - Main",
            SourceType::Ecommerce(
                EcommerceSource::new("https://books.toscrape.com")
                    .with_name("Books to Scrape - Main")
            ),
        ),
        (
            "Books to Scrape - Travel",
            SourceType::Ecommerce(
                EcommerceSource::new("https://books.toscrape.com/catalogue/category/books/travel_2/index.html")
                    .with_name("Books to Scrape - Travel")
            ),
        ),
        (
            "Books to Scrape - Mystery",
            SourceType::Ecommerce(
                EcommerceSource::new("https://books.toscrape.com/catalogue/category/books/mystery_3/index.html")
                    .with_name("Books to Scrape - Mystery")
            ),
        ),
        (
            "Books to Scrape - Historical Fiction",
            SourceType::Ecommerce(
                EcommerceSource::new("https://books.toscrape.com/catalogue/category/books/historical-fiction_4/index.html")
                    .with_name("Books to Scrape - Historical Fiction")
            ),
        ),
    ];

    println!("✅ Configured {} data sources:", sources.len());
    for (name, _) in &sources {
        println!("   • {}", name);
    }
    println!();

    // Step 3: Scrape from all sources
    println!("🔍 Step 3: Scraping Data from Multiple Sources...");
    let mut all_raw_data = Vec::new();
    let mut source_stats = Vec::new();

    for (source_name, source) in sources {
        print!("   Scraping from {}... ", source_name);
        
        match engine.scrape_source(source).await {
            Ok(data) => {
                let count = data.len();
                println!("✅ {} items", count);
                
                // Save raw data per source
                let filename = format!("data/raw/{}.json", 
                    source_name.replace(" ", "_").replace("-", "_").to_lowercase());
                
                let json = serde_json::to_string_pretty(&data)?;
                tokio::fs::write(&filename, json).await?;
                println!("      💾 Saved to {}", filename);
                
                source_stats.push((source_name, count));
                all_raw_data.extend(data);
                
                // Polite delay between sources
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
            Err(e) => {
                println!("❌ Error: {}", e);
            }
        }
    }

    println!("\n📊 Raw Data Collection Summary:");
    println!("   Total sources scraped: {}", source_stats.len());
    println!("   Total raw items: {}", all_raw_data.len());
    for (source, count) in &source_stats {
        println!("      • {}: {} items", source, count);
    }
    println!();

    // Step 4: Process through pipeline
    println!("⚙️  Step 4: Processing Through Pipeline...");
    let processed_data = engine.process_data(all_raw_data).await?;
    println!("✅ Processed {} items (validated, deduplicated)\n", processed_data.len());

    // Step 5: AI Normalization
    println!("🤖 Step 5: AI-Powered Data Normalization...");
    println!("   Initializing DeepSeek AI client...");
    
    match DeepSeekClient::new() {
        Ok(client) => {
            println!("   ✅ DeepSeek client connected");
            
            // Test connection first
            match client.test_connection().await {
                Ok(_) => {
                    println!("   ✅ API connection verified\n");
                    
                    // Initialize normalizer
                    let normalizer = DataNormalizer::new(client)
                        .with_batch_size(25); // Smaller batches for demo
                    
                    println!("   🔄 Normalizing {} items with AI...", processed_data.len());
                    println!("   (This may take 30-60 seconds depending on API response time)\n");
                    
                    match normalizer.normalize_all(processed_data.clone()).await {
                        Ok((normalized_data, stats)) => {
                            println!("   ✅ AI Normalization Complete!");
                            println!("\n   📈 Normalization Statistics:");
                            println!("      Input items:        {}", stats.total_input);
                            println!("      Output items:       {}", stats.total_output);
                            println!("      Duplicates removed: {}", stats.duplicates_removed);
                            println!("      Fields standardized: ~{}", stats.fields_standardized);
                            
                            // Save normalized data
                            let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
                            let normalized_path = format!("data/normalized/final_{}.json", timestamp);
                            
                            normalizer.save_to_json(&normalized_data, &normalized_path).await?;
                            
                            // Also save as final.json (latest)
                            normalizer.save_to_json(&normalized_data, "data/normalized/final.json").await?;
                            
                            println!("\n   💾 Saved normalized data:");
                            println!("      • data/normalized/final.json (latest)");
                            println!("      • {} (timestamped)", normalized_path);
                            
                            // Show sample of normalized data
                            if let Some(first) = normalized_data.first() {
                                println!("\n   📄 Sample Normalized Record:");
                                println!("      ID:        {}", first.id);
                                println!("      Title:     {}", first.title);
                                println!("      Price USD: ${:.2}", first.price_usd.unwrap_or(0.0));
                                println!("      Category:  {:?}", first.category);
                                println!("      Source:    {}", first.source);
                            }
                        }
                        Err(e) => {
                            println!("   ⚠️  AI normalization failed: {}", e);
                            println!("   Using simple normalization instead...\n");
                            
                            let simple_normalized = DataNormalizer::normalize_simple(processed_data);
                            
                            let json = serde_json::to_string_pretty(&simple_normalized)?;
                            tokio::fs::write("data/normalized/final.json", json).await?;
                            
                            println!("   ✅ Simple normalization completed: {} items", simple_normalized.len());
                        }
                    }
                }
                Err(e) => {
                    println!("   ⚠️  API connection test failed: {}", e);
                    println!("   Using simple normalization instead...\n");
                    
                    let simple_normalized = DataNormalizer::normalize_simple(processed_data);
                    
                    let json = serde_json::to_string_pretty(&simple_normalized)?;
                    tokio::fs::write("data/normalized/final.json", json).await?;
                    
                    println!("   ✅ Simple normalization completed: {} items", simple_normalized.len());
                }
            }
        }
        Err(e) => {
            println!("   ⚠️  DeepSeek client initialization failed: {}", e);
            println!("   Make sure DEEPSEEK_API_KEY is set in .env");
            println!("   Using simple normalization instead...\n");
            
            let simple_normalized = DataNormalizer::normalize_simple(processed_data);
            
            let json = serde_json::to_string_pretty(&simple_normalized)?;
            tokio::fs::write("data/normalized/final.json", json).await?;
            
            println!("   ✅ Simple normalization completed: {} items", simple_normalized.len());
        }
    }

    // Final summary
    println!("\n═══════════════════════════════════════════════════");
    println!("🎉 Multi-Source AI Pipeline Complete!");
    println!("═══════════════════════════════════════════════════\n");
    
    println!("📁 Output Files:");
    println!("   Raw Data:");
    for (source, _) in &source_stats {
        let filename = format!("data/raw/{}.json", 
            source.replace(" ", "_").replace("-", "_").to_lowercase());
        println!("      • {}", filename);
    }
    println!("\n   Normalized Data:");
    println!("      • data/normalized/final.json (unified schema)");
    
    println!("\n💡 Next Steps:");
    println!("   1. View data/normalized/final.json to see unified output");
    println!("   2. Restart the server to load normalized data");
    println!("   3. Visit http://localhost:3000 to see results in UI");
    println!("   4. Check logs for AI normalization decisions\n");

    Ok(())
}
