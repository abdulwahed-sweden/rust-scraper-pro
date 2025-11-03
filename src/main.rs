use anyhow::Result;
use rust_scraper_pro::{
    config::Config,
    core::scraper::ScraperEngine,
    output::{
        api::{ApiServer, SharedData},
        csv::CsvOutput,
        database::SqliteOutput,
        json::JsonOutput,
    },
    processors::pipeline::ProcessingPipeline,
    sources::{
        ecommerce::EcommerceSource,
        news::NewsSource,
        social::SocialSource,
        source::{Source, SourceType},
    },
    utils::{
        cache::HtmlCache,
        logger::setup_logger,
    },
};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;
    
    log::info!("Starting Rust Scraper Pro");
    
    // Load configuration
    let config = Config::load("config/settings.toml").await?;
    
    // Initialize cache
    let cache = Arc::new(HtmlCache::new_html_cache(1000, 3600));
    
    // Create processing pipeline
    let pipeline = ProcessingPipeline::new();
    
    // Initialize scraper engine with cache
    let mut engine = ScraperEngine::new(config, pipeline, Some(cache.clone()));
    
    // Initialize database output
    let db_output = SqliteOutput::new("sqlite:scraped_data.db", None).await?;
    db_output.init().await?;
    
    // Initialize API server
    let api_data: SharedData = Arc::new(tokio::sync::RwLock::new(Vec::new()));
    let api_server = ApiServer::new(api_data.clone(), Some(3000));
    
    // Start API server in background
    tokio::spawn(async move {
        if let Err(e) = api_server.run().await {
            log::error!("API server error: {}", e);
        }
    });
    
    // Scrape from multiple sources
    let sources = vec![
        SourceType::News(NewsSource::new("https://news.ycombinator.com")),
        SourceType::Ecommerce(EcommerceSource::new("https://httpbin.org/html")),
        SourceType::Social(SocialSource::twitter()),
        SourceType::Social(SocialSource::reddit().with_name("Reddit Frontpage")),
    ];
    
    let mut all_scraped_data = Vec::new();
    
    for source in sources {
        log::info!("Scraping from: {}", source.name());
        
        match engine.scrape_source(source).await {
            Ok(data) => {
                log::info!("Successfully scraped {} items", data.len());
                all_scraped_data.extend(data);
                
                // Rate limiting between sources
                sleep(Duration::from_millis(2000)).await;
            }
            Err(e) => log::error!("Failed to scrape: {}", e),
        }
    }
    
    // Process all data through pipeline
    let processed_data = engine.process_data(all_scraped_data).await?;
    
    // Export to various formats
    log::info!("Exporting {} processed items", processed_data.len());
    
    // JSON export
    let json_output = JsonOutput::new();
    json_output.export(&processed_data, "output/data.json").await?;
    
    // CSV export
    let csv_output = CsvOutput::new();
    csv_output.export(&processed_data, "output/data.csv").await?;
    
    // Database export
    db_output.save(&processed_data).await?;
    
    // Update API data
    {
        let mut api_data_guard = api_data.write().await;
        *api_data_guard = processed_data.clone();
    }
    
    // Display cache statistics
    let cache_stats = cache.stats();
    log::info!("Cache statistics: {:?}", cache_stats);
    
    log::info!("Scraping completed! Data available via API at http://localhost:3000");
    log::info!("Press Ctrl+C to exit...");
    
    // Keep the application running
    tokio::signal::ctrl_c().await?;
    
    Ok(())
}