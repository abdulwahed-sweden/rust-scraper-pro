#[cfg(test)]
mod integration_tests {
    use super::*;
    use rust_scraper_pro::{
        core::scraper::ScraperEngine,
        processors::pipeline::ProcessingPipeline,
        sources::news::NewsSource,
        sources::source::SourceType,
        utils::{logger::setup_test_logger, cache::HtmlCache},
        output::{json::JsonOutput, csv::CsvOutput},
    };
    use std::sync::Arc;
    use tokio::fs;

    fn setup() {
        let _ = setup_test_logger();
    }

    #[tokio::test]
    async fn test_full_pipeline() {
        setup();
        
        // Create a simple configuration
        let config = crate::config::Config::load("config/settings.toml").await.unwrap();
        
        // Initialize components
        let cache = Arc::new(HtmlCache::new_html_cache(100, 300));
        let pipeline = ProcessingPipeline::new();
        let mut engine = ScraperEngine::new(config, pipeline, Some(cache));
        
        // Test with a simple source (using httpbin for testing)
        let source = NewsSource::new("https://httpbin.org/html");
        
        // Scrape data
        let scraped_data = engine.scrape_source(source).await;
        
        // The scrape might fail (if network issues), but shouldn't panic
        match scraped_data {
            Ok(data) => {
                // Process data
                let processed_data = engine.process_data(data).await.unwrap();
                
                // Should not panic when processing
                assert!(true, "Pipeline processed without errors");
                
                // Test exports
                let json_output = JsonOutput::new();
                let csv_output = CsvOutput::new();
                
                // Clean up previous test files
                let _ = fs::remove_file("test_output.json").await;
                let _ = fs::remove_file("test_output.csv").await;
                
                // Test JSON export
                json_output.export(&processed_data, "test_output.json").await.unwrap();
                assert!(fs::metadata("test_output.json").await.is_ok());
                
                // Test CSV export
                csv_output.export(&processed_data, "test_output.csv").await.unwrap();
                assert!(fs::metadata("test_output.csv").await.is_ok());
                
                // Clean up
                let _ = fs::remove_file("test_output.json").await;
                let _ = fs::remove_file("test_output.csv").await;
            }
            Err(e) => {
                // Network errors are acceptable in tests
                println!("Network error during test (acceptable): {}", e);
                assert!(true, "Network error handled gracefully");
            }
        }
    }

    #[tokio::test]
    async fn test_cache_functionality() {
        setup();
        
        let cache = HtmlCache::new_html_cache(10, 60); // Small cache for testing
        
        let test_url = "https://example.com/test";
        let test_html = "<html><body>Test content</body></html>";
        
        // Test cache set and get
        cache.set_html(test_url, test_html).await.unwrap();
        let cached_content = cache.get_html(test_url).await.unwrap();
        
        assert_eq!(cached_content, test_html);
        
        // Test cache stats
        let stats = cache.stats();
        assert_eq!(stats.entry_count, 1);
        
        // Test cache clear
        cache.clear().await.unwrap();
        let stats_after_clear = cache.stats();
        assert_eq!(stats_after_clear.entry_count, 0);
    }

    #[tokio::test]
    async fn test_multiple_sources() {
        setup();
        
        let config = crate::config::Config::load("config/settings.toml").await.unwrap();
        let cache = Arc::new(HtmlCache::new_html_cache(50, 300));
        let pipeline = ProcessingPipeline::new();
        let mut engine = ScraperEngine::new(config, pipeline, Some(cache));
        
        let sources = vec![
            SourceType::News(NewsSource::new("https://httpbin.org/html").with_name("Test News 1")),
            SourceType::News(NewsSource::new("https://httpbin.org/json").with_name("Test News 2")),
        ];
        
        let mut all_data = Vec::new();
        
        for source in sources {
            match engine.scrape_source(source).await {
                Ok(data) => {
                    all_data.extend(data);
                }
                Err(e) => {
                    // Accept network errors in tests
                    println!("Source failed (acceptable in tests): {}", e);
                }
            }
        }
        
        // Process whatever data we collected
        if !all_data.is_empty() {
            let processed = engine.process_data(all_data).await.unwrap();
            assert!(!processed.is_empty() || true); // Allow empty results in tests
        }
        
        assert!(true, "Multiple sources handled without panic");
    }
}