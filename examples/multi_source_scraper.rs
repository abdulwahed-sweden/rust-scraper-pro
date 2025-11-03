use anyhow::Result;
use rust_scraper_pro::prelude::*;
use rust_scraper_pro::{
    sources::{
        ecommerce::EcommerceSource,
        news::NewsSource, 
        social::SocialSource,
        source::SourceType,
    },
    utils::logger::setup_logger,
};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;
    
    log::info!("Starting Multi-Source Scraper Example");
    
    // Load configuration
    let config = Config::load("config/settings.toml").await?;
    
    // Initialize cache
    let cache = Arc::new(HtmlCache::new_html_cache(500, 1800)); // 500 items, 30 min TTL
    
    // Create processing pipeline
    let pipeline = ProcessingPipeline::new();
    
    // Initialize scraper engine
    let mut engine = ScraperEngine::new(config, pipeline, Some(cache.clone()));
    
    // Define multiple sources to scrape
    let sources = vec![
        SourceType::News(NewsSource::new("https://news.ycombinator.com").with_name("Hacker News")),
        SourceType::Ecommerce(EcommerceSource::new("https://httpbin.org/html").with_name("Example Store")),
        SourceType::Social(SocialSource::twitter()),
        SourceType::Social(SocialSource::reddit().with_name("Reddit Popular")),
    ];
    
    let mut all_results = Vec::new();
    
    // Scrape each source with error handling
    for source in sources {
        log::info!("🔍 Scraping from: {}", source.name());
        
        match engine.scrape_source(source).await {
            Ok(mut data) => {
                log::info!("✅ Successfully scraped {} items from {}", data.len(), source.name());
                
                // Add source-specific metadata
                for item in &mut data {
                    item.add_metadata("scraped_at".to_string(), chrono::Utc::now().to_rfc3339());
                }
                
                all_results.extend(data);
                
                // Respectful delay between sources
                sleep(Duration::from_secs(2)).await;
            }
            Err(e) => {
                log::error!("❌ Failed to scrape {}: {}", source.name(), e);
            }
        }
    }
    
    // Process all data through pipeline
    log::info!("🔄 Processing {} items through pipeline", all_results.len());
    let processed_data = engine.process_data(all_results).await?;
    
    // Export results
    log::info!("💾 Exporting {} processed items", processed_data.len());
    
    // JSON export
    let json_output = JsonOutput::new();
    json_output.export(&processed_data, "output/multi_source_data.json").await?;
    
    // CSV export
    let csv_output = CsvOutput::new();
    csv_output.export(&processed_data, "output/multi_source_data.csv").await?;
    
    // SQLite export
    let db_output = SqliteOutput::new("sqlite:multi_source.db", None).await?;
    db_output.init().await?;
    db_output.save(&processed_data).await?;
    
    // Display summary
    log::info!("🎉 Multi-source scraping completed!");
    log::info!("📊 Total items processed: {}", processed_data.len());
    
    // Group by source
    use std::collections::HashMap;
    let mut source_counts: HashMap<String, usize> = HashMap::new();
    for item in &processed_data {
        *source_counts.entry(item.source.clone()).or_insert(0) += 1;
    }
    
    for (source, count) in source_counts {
        log::info!("   {}: {} items", source, count);
    }
    
    Ok(())
}
