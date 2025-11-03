# Rust Scraper Pro 🚀

A professional, high-performance web scraping library built in Rust. Designed for reliability, speed, and extensibility.

## Features

- **Multi-source Support**: News, E-commerce, Social Media, and Custom sources
- **Advanced Processing**: Validation, Normalization, and Deduplication pipeline  
- **Multiple Output Formats**: JSON, CSV, Database (SQLite/PostgreSQL), REST API
- **Intelligent Caching**: Memory and file-based caching with TTL
- **Rate Limiting**: Respectful scraping with configurable delays
- **Error Handling**: Comprehensive error handling and logging
- **Extensible Architecture**: Easy to add new sources and processors

## Quick Start

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rust-scraper-pro = "0.1.0"
```

### Basic Usage

```rust
use rust_scraper_pro::prelude::*;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::load("config/settings.toml").await?;
    
    // Initialize scraper
    let cache = Arc::new(HtmlCache::new_html_cache(1000, 3600));
    let pipeline = ProcessingPipeline::new();
    let mut engine = ScraperEngine::new(config, pipeline, Some(cache));
    
    // Scrape from a source
    let source = NewsSource::new("https://news.ycombinator.com");
    let data = engine.scrape_source(source).await?;
    
    // Process and export
    let processed_data = engine.process_data(data).await?;
    
    let json_output = JsonOutput::new();
    json_output.export(&processed_data, "output/data.json").await?;
    
    Ok(())
}
```

## Examples

Check the `examples/` directory for comprehensive examples:

- `multi_source_scraper.rs` - Scrape from multiple sources
- `ecommerce_scraper.rs` - E-commerce product scraping  
- `news_crawler.rs` - News article scraping and analysis
- `advanced_pipeline.rs` - Continuous scraping with API

Run examples with:
```bash
cargo run --example multi_source_scraper
```

## Configuration

Edit `config/settings.toml`:

```toml
[scraping]
rate_limit_ms = 1000
timeout_seconds = 30
user_agent = "RustScraperPro/1.0"

[api]
port = 3000

[[sources]]
name = "Hacker News"
url = "https://news.ycombinator.com"
type = "news"
```

## API Server

Start the built-in API server:

```rust
let api_data: SharedData = Arc::new(tokio::sync::RwLock::new(Vec::new()));
let api_server = ApiServer::new(api_data, Some(3000));
api_server.run().await?;
```

**API Endpoints:**
- `GET /api/data` - Get scraped data
- `GET /api/search` - Search data  
- `GET /api/stats` - Get statistics
- `GET /api/export/json` - Export as JSON
- `GET /api/export/csv` - Export as CSV

## Database Support

### SQLite
```rust
let db = SqliteOutput::new("sqlite:data.db", None).await?;
db.init().await?;
db.save(&data).await?;
```

### PostgreSQL
```rust
let db = PostgresOutput::new("postgres://user:pass@localhost/db", None).await?;
db.init().await?;
db.save(&data).await?;
```

## Project Structure

```
rust-scraper-pro/
├── Cargo.toml
├── README.md
├── config/
│   ├── settings.toml
│   └── sources.toml
├── examples/
│   ├── multi_source_scraper.rs
│   ├── ecommerce_scraper.rs
│   ├── news_crawler.rs
│   └── advanced_pipeline.rs
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── core/           # Core scraping logic
│   ├── sources/        # Source implementations  
│   ├── processors/     # Data processing pipeline
│   ├── output/         # Export formats
│   └── utils/          # Utilities
└── tests/              # Unit & integration tests
```

## Contributing

1. Fork the repository
2. Create a feature branch  
3. Add tests for new functionality
4. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Support

- Create an issue on GitHub
- Check the examples directory  
- Review the API documentation