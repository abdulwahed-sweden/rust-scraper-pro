use anyhow::Result;
use rust_scraper_pro::prelude::*;
use rust_scraper_pro::{
    sources::ecommerce::EcommerceSource,
    sources::source::SourceType,
    utils::logger::setup_logger,
};

#[tokio::main]
async fn main() -> Result<()> {
    setup_logger()?;
    
    log::info!("Starting E-commerce Scraper Example");
    
    // Load configuration
    let config = Config::load("config/settings.toml").await?;
    
    // Initialize cache specifically for e-commerce
    let cache = Arc::new(HtmlCache::new_html_cache(200, 3600)); // 200 items, 1 hour TTL
    
    // Create processing pipeline
    let pipeline = ProcessingPipeline::new();
    
    // Initialize scraper engine
    let mut engine = ScraperEngine::new(config, pipeline, Some(cache.clone()));
    
    // Define e-commerce sources (using example URLs for demonstration)
    let ecommerce_sources = vec![
        EcommerceSource::new("https://httpbin.org/html").with_name("Example Store 1"),
        EcommerceSource::new("https://httpbin.org/json").with_name("Example Store 2"),
        // Add real e-commerce URLs here:
        // EcommerceSource::new("https://example-store.com/products").with_name("Real Store"),
    ];
    
    let mut all_products = Vec::new();
    
    for source in ecommerce_sources {
        let source_type = SourceType::Ecommerce(source);
        log::info!("🛒 Scraping products from: {}", source_type.name());
        
        match engine.scrape_source(source_type).await {
            Ok(mut products) => {
                log::info!("✅ Found {} products", products.len());
                
                // Add e-commerce specific metadata
                for product in &mut products {
                    product.add_metadata("product_type".to_string(), "general".to_string());
                    product.add_metadata("currency".to_string(), "USD".to_string());
                    
                    // Categorize by price if available
                    if let Some(price) = product.price {
                        let category = if price < 50.0 {
                            "budget"
                        } else if price < 200.0 {
                            "mid-range"
                        } else {
                            "premium"
                        };
                        product.category = Some(category.to_string());
                    }
                }
                
                all_products.extend(products);
            }
            Err(e) => {
                log::error!("❌ Failed to scrape products: {}", e);
            }
        }
        
        // Rate limiting between stores
        tokio::time::sleep(Duration::from_secs(3)).await;
    }
    
    // Process products
    log::info!("🔄 Processing {} products", all_products.len());
    let processed_products = engine.process_data(all_products).await?;
    
    // Filter products with prices
    let products_with_prices: Vec<ScrapedData> = processed_products
        .iter()
        .filter(|p| p.price.is_some())
        .cloned()
        .collect();
    
    log::info!("💰 Products with prices: {}", products_with_prices.len());
    
    // Calculate price statistics
    let total_value: f64 = products_with_prices.iter()
        .filter_map(|p| p.price)
        .sum();
    
    let avg_price = if !products_with_prices.is_empty() {
        total_value / products_with_prices.len() as f64
    } else {
        0.0
    };
    
    log::info!("📈 Price Statistics:");
    log::info!("   Total value: ${:.2}", total_value);
    log::info!("   Average price: ${:.2}", avg_price);
    
    // Export e-commerce data
    let json_output = JsonOutput::new();
    json_output.export(&processed_products, "output/ecommerce_products.json").await?;
    
    let csv_output = CsvOutput::new();
    csv_output.export_with_metadata(&processed_products, "output/ecommerce_products.csv").await?;
    
    // Database export
    let db_output = SqliteOutput::new("sqlite:ecommerce.db", Some("products")).await?;
    db_output.init().await?;
    let saved_count = db_output.save(&processed_products).await?;
    
    log::info!("🎉 E-commerce scraping completed!");
    log::info!("💾 Saved {} products to database", saved_count);
    
    Ok(())
}
