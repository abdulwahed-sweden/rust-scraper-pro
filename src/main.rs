use anyhow::Result;
use rust_scraper_pro::{
    core::config::Config,
    core::scraper::ScraperEngine,
    output::{
        api::{ApiServer, SharedData},
        csv::CsvOutput,
        database::{DatabaseOutput, PostgresOutput},
        json::JsonOutput,
    },
    processors::pipeline::ProcessingPipeline,
    sources::{
        EcommerceSource,
        Source,
        SourceType,
    },
    utils::{
        cache::HtmlCache,
        logger::setup_logger,
    },
};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    // Load environment variables from .env file if it exists
    dotenvy::dotenv().ok();

    // Initialize logger and tracing
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

    // Initialize PostgreSQL database (with graceful fallback)
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/rust_scraper_db".to_string());

    let db_output = match PostgresOutput::new(&database_url, None).await {
        Ok(db) => {
            log::info!("Connected to PostgreSQL database");
            match db.init().await {
                Ok(_) => {
                    log::info!("Database schema initialized successfully");
                    Some(db)
                }
                Err(e) => {
                    log::warn!("Failed to initialize database schema: {}", e);
                    log::warn!("Continuing without database persistence");
                    None
                }
            }
        }
        Err(e) => {
            log::warn!("Failed to connect to PostgreSQL: {}", e);
            log::warn!("Continuing without database persistence. Data will only be stored in-memory.");
            None
        }
    };

    // Initialize API server with port from environment or default to 3000
    let port = std::env::var("SERVER_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);

    let api_data: SharedData = Arc::new(tokio::sync::RwLock::new(Vec::new()));
    let db_arc = db_output.map(Arc::new);
    let api_server = ApiServer::new(api_data.clone(), db_arc.clone(), Some(port));
    
    // Start API server in background
    tokio::spawn(async move {
        if let Err(e) = api_server.run().await {
            log::error!("API server error: {}", e);
        }
    });
    
    // Scrape from legal public test sources
    // books.toscrape.com is specifically designed for scraping practice
    let sources = vec![
        SourceType::Ecommerce(
            EcommerceSource::new("https://books.toscrape.com")
                .with_name("Books to Scrape")
        ),
        SourceType::Ecommerce(
            EcommerceSource::new("https://books.toscrape.com/catalogue/category/books/science_22/index.html")
                .with_name("Science Books")
        ),
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

    // Save to database if available
    if let Some(db) = &db_arc {
        use rust_scraper_pro::output::database::DatabaseOutput;
        match db.save(&processed_data).await {
            Ok(count) => log::info!("Saved {} items to PostgreSQL database", count),
            Err(e) => log::error!("Failed to save to database: {}", e),
        }
    }

    // Update API in-memory data
    {
        let mut api_data_guard = api_data.write().await;
        *api_data_guard = processed_data.clone();
    }
    
    // Display cache statistics
    let cache_stats = cache.stats();
    log::info!("Cache statistics: {:?}", cache_stats);

    log::info!("Scraping completed! Data available via API at http://localhost:{}", port);
    log::info!("Press Ctrl+C to exit...");

    // Keep the application running
    tokio::signal::ctrl_c().await?;

    Ok(())
}