#[cfg(test)]
mod tests {
    use super::*;
    use rust_scraper_pro::{
        core::models::ScrapedData,
        processors::{normalizer::Normalizer, validator::Validator, deduplicator::Deduplicator},
        utils::logger::setup_test_logger,
    };
    use chrono::Utc;

    fn setup() {
        let _ = setup_test_logger();
    }

    #[tokio::test]
    async fn test_normalizer() {
        setup();
        
        let normalizer = Normalizer::new();
        
        let mut test_data = ScrapedData::new("test".to_string(), "https://example.com".to_string());
        test_data.title = Some("  Hello   World  \n\n".to_string());
        test_data.content = Some("This is a test content.   ".to_string());
        test_data.price = Some(123.456789);
        
        let data = vec![test_data];
        let normalized = normalizer.normalize(data).await.unwrap();
        
        assert_eq!(normalized[0].title, Some("Hello World".to_string()));
        assert_eq!(normalized[0].content, Some("This is a test content.".to_string()));
        assert_eq!(normalized[0].price, Some(123.46)); // Rounded to 2 decimal places
    }

    #[tokio::test]
    async fn test_validator() {
        setup();
        
        let validator = Validator::new();
        
        let valid_data = ScrapedData::new("test".to_string(), "https://example.com".to_string())
            .with_title("Valid Title".to_string());
        
        let invalid_data_no_content = ScrapedData::new("test".to_string(), "https://example.com".to_string());
        
        let invalid_data_bad_url = ScrapedData::new("test".to_string(), "invalid-url".to_string())
            .with_title("Bad URL".to_string());
        
        let test_data = vec![valid_data, invalid_data_no_content, invalid_data_bad_url];
        let validated = validator.validate(test_data).await.unwrap();
        
        assert_eq!(validated.len(), 1);
        assert_eq!(validated[0].title, Some("Valid Title".to_string()));
    }

    #[tokio::test]
    async fn test_deduplicator() {
        setup();
        
        let deduplicator = Deduplicator::new();
        
        let data1 = ScrapedData::new("source".to_string(), "https://example.com/1".to_string())
            .with_title("Unique Title 1".to_string());
        
        let data2 = ScrapedData::new("source".to_string(), "https://example.com/1".to_string())
            .with_title("Unique Title 2".to_string()); // Same URL, different title
        
        let data3 = ScrapedData::new("source".to_string(), "https://example.com/2".to_string())
            .with_title("Unique Title 1".to_string()); // Same title, different URL
        
        let test_data = vec![data1, data2, data3];
        let deduplicated = deduplicator.deduplicate(test_data).await.unwrap();
        
        // Should remove one duplicate (same URL)
        assert_eq!(deduplicated.len(), 2);
    }

    #[tokio::test]
    async fn test_scraped_data_creation() {
        setup();
        
        let data = ScrapedData::new("test_source".to_string(), "https://example.com".to_string())
            .with_title("Test Title".to_string())
            .with_content("Test content".to_string())
            .with_author("Test Author".to_string())
            .with_price(99.99);
        
        assert_eq!(data.source, "test_source");
        assert_eq!(data.url, "https://example.com");
        assert_eq!(data.title, Some("Test Title".to_string()));
        assert_eq!(data.content, Some("Test content".to_string()));
        assert_eq!(data.author, Some("Test Author".to_string()));
        assert_eq!(data.price, Some(99.99));
        assert!(!data.id.is_empty());
    }

    #[tokio::test]
    async fn test_metadata_operations() {
        setup();
        
        let mut data = ScrapedData::new("test".to_string(), "https://example.com".to_string());
        
        data.add_metadata("key1".to_string(), "value1".to_string());
        data.add_metadata("key2".to_string(), "value2".to_string());
        
        assert_eq!(data.metadata.get("key1"), Some(&"value1".to_string()));
        assert_eq!(data.metadata.get("key2"), Some(&"value2".to_string()));
        assert_eq!(data.metadata.len(), 2);
    }
}