use anyhow::Result;
use rust_scraper_pro::prelude::*;
use rust_scraper_pro::{
    output::api::{ApiServer, SharedData},
    sources::{
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
    
    log::info!("Starting Advanced Pipeline Example");
    
    // Enhanced configuration
    let config = Config::load("config/settings.toml").await?;
    
    // Advanced caching with hybrid setup
    let cache = Arc::new(HtmlCache::new_hybrid_cache(2000, 7200, "cache/html")); // 2h TTL
    
    // Custom processing pipeline
    let mut pipeline = ProcessingPipeline::new();
    // Additional custom processors can be added here
    
    // Initialize scraper with cache
    let mut engine = ScraperEngine::new(config, pipeline, Some(cache.clone()));
    
    // Database setup
    let db_output = SqliteOutput::new("sqlite:advanced_data.db", None).await?;
    db_output.init().await?;
    
    // API setup
    let api_data: SharedData = Arc::new(tokio::sync::RwLock::new(Vec::new()));
    let api_server = ApiServer::new(api_data.clone(), Some(8080));
    
    // Start API in background
    let api_handle = tokio::spawn(async move {
        log::info!("🌐 Starting API server on port 8080");
        if let Err(e) = api_server.run().await {
            log::error!("API server error: {}", e);
        }
    });
    
    // Define sources for continuous monitoring
    let sources = vec![
        SourceType::News(NewsSource::new("https://news.ycombinator.com").with_name("Hacker News")),
        SourceType::Social(SocialSource::twitter()),
        // Add more sources for monitoring
    ];
    
    log::info!("🔄 Starting continuous scraping loop");
    log::info!("   Monitoring {} sources", sources.len());
    log::info!("   API available at: http://localhost:8080");
    log::info!("   Press Ctrl+C to stop");
    
    let mut cycle_count = 0;
    
    loop {
        cycle_count += 1;
        log::info!("--- Scraping Cycle {} ---", cycle_count);
        
        let mut cycle_data = Vec::new();
        
        for source in &sources {
            match engine.scrape_source(source.clone()).await {
                Ok(data) => {
                    log::info!("✅ {}: {} items", source.name(), data.len());
                    cycle_data.extend(data);
                }
                Err(e) => log::error!("❌ {}: {}", source.name(), e),
            }
            
            sleep(Duration::from_secs(5)).await; // 5 seconds between sources
        }
        
        if !cycle_data.is_empty() {
            // Process the data
            let processed_data = engine.process_data(cycle_data).await?;
            
            // Save to database
            let saved_count = db_output.save(&processed_data).await?;
            
            // Update API
            {
                let mut api_data_guard = api_data.write().await;
                *api_data_guard = processed_data;
            }
            
            log::info!("💾 Cycle {}: Saved {} items to database", cycle_count, saved_count);
            
            // Display cache statistics
            let cache_stats = cache.stats();
            log::info!("📦 Cache stats: {} entries, {:.2}% hit rate", 
                cache_stats.entry_count, cache_stats.hit_rate * 100.0);
        } else {
            log::warn!("⚠️  Cycle {}: No data collected", cycle_count);
        }
        
        log::info!("⏰ Waiting 10 minutes until next cycle...");
        sleep(Duration::from_secs(600)).await; // 10 minutes between cycles
    }
    
    // Note: This example runs indefinitely, use Ctrl+C to stop
    // The API handle will keep running in the background
}
