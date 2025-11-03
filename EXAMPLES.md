# 📚 Realistic Examples Guide

This directory contains **production-quality examples** using real public data sources. All examples demonstrate polite scraping practices with proper rate limiting, error handling, and data processing.

---

## 🗂️ Available Examples

### 1. **news_scraper.rs** - News Aggregation
**Source:** Hacker News (news.ycombinator.com)
**Purpose:** Demonstrates HTML-based news scraping

```bash
cargo run --example news_scraper
```

**What it does:**
- Scrapes top stories from Hacker News
- Extracts titles, authors, and links
- Validates and processes data through pipeline
- Exports to JSON and CSV formats
- Shows cache statistics

**Output:**
- `output/hacker_news.json`
- `output/hacker_news.csv`

---

### 2. **ecommerce_scraper.rs** - Product Cataloging
**Source:** Books to Scrape (books.toscrape.com)
**Purpose:** E-commerce product data extraction

```bash
cargo run --example ecommerce_scraper
```

**What it does:**
- Scrapes book listings from a training e-commerce site
- Extracts titles, prices, ratings, and images
- Calculates statistics (average price, total value)
- Demonstrates handling of structured product data
- Exports to multiple formats

**Output:**
- `output/books_catalog.json`
- `output/books_catalog.csv`

---

### 3. **social_scraper.rs** - JSON API Scraping
**Source:** Reddit Public JSON API (reddit.com/r/worldnews.json)
**Purpose:** API-based data collection (no HTML parsing)

```bash
cargo run --example social_scraper
```

**What it does:**
- Fetches data from Reddit's public JSON endpoint
- Processes social media posts with metadata
- Extracts scores, comments, authors
- Demonstrates JSON API scraping pattern
- Shows statistics and trends

**Output:**
- `output/reddit_worldnews.json`
- `output/reddit_worldnews.csv`

---

### 4. **multi_source_pipeline.rs** - Unified Data Pipeline
**Sources:** Hacker News + Books to Scrape
**Purpose:** Complete end-to-end scraping workflow

```bash
cargo run --example multi_source_pipeline
```

**What it does:**
- Scrapes from multiple sources concurrently
- Unified data processing pipeline
- Validation, normalization, and deduplication
- Cross-source statistics and analysis
- Demonstrates professional workflow

**Output:**
- `output/multi_source_data.json`
- `output/multi_source_data.csv`

---

## ⚙️ Configuration

All examples use configuration from:
- `config/settings.toml` - Global settings (rate limits, user agent, etc.)
- `config/sources.toml` - Source-specific configurations

### Key Settings:

```toml
[scraping]
rate_limit_ms = 2000  # 2 seconds between requests (polite!)
timeout_seconds = 30
user_agent = "Mozilla/5.0 (compatible; RustScraperPro/1.0; Educational)"
follow_robots_txt = true
```

---

## 🎯 Real-World Data Sources

### ✅ Safe & Educational Sources Used:

| Source | Type | Purpose | Rate Limit |
|--------|------|---------|------------|
| **Hacker News** | News aggregator | Public tech news | 2s |
| **Books to Scrape** | E-commerce | Training site for scraping | 2s |
| **Reddit JSON API** | Social media | Public posts (no auth) | 3s |

All sources are:
- ✅ Publicly accessible
- ✅ Appropriate for educational use
- ✅ Designed for or tolerant of scraping
- ✅ No authentication required
- ✅ Well-documented structures

---

## 🚀 Running Examples

### Basic Usage:
```bash
# Run specific example
cargo run --example news_scraper
cargo run --example ecommerce_scraper
cargo run --example social_scraper
cargo run --example multi_source_pipeline

# Build all examples
cargo build --examples

# Run with release optimizations
cargo run --release --example multi_source_pipeline
```

### Expected Runtime:
- **news_scraper**: ~5-10 seconds
- **ecommerce_scraper**: ~5-10 seconds
- **social_scraper**: ~3-5 seconds (JSON API)
- **multi_source_pipeline**: ~15-30 seconds

---

## 📊 Output Format

### JSON Example:
```json
{
  "id": "uuid-here",
  "source": "Hacker News",
  "url": "https://...",
  "title": "Example Title",
  "content": "Article content...",
  "author": "username",
  "timestamp": "2025-11-03T21:00:00Z",
  "metadata": {},
  "category": "news"
}
```

### CSV Example:
```csv
id,source,url,title,content,price,author,timestamp,category
uuid,Hacker News,https://...,Title,Content,,,2025-11-03,news
```

---

## 🛡️ Best Practices Demonstrated

### ✅ Polite Scraping:
- **Rate limiting** (2-3s between requests)
- **Proper user agent** identification
- **Timeout handling** (30s max)
- **Error recovery** with retries
- **Cache utilization** to reduce load

### ✅ Data Quality:
- **Validation** pipeline
- **Deduplication** of results
- **Normalization** of formats
- **Error handling** for missing data
- **Comprehensive logging**

### ✅ Code Quality:
- **Async/await** with Tokio
- **Type safety** with Rust
- **Memory efficient** caching
- **Clear error messages**
- **Production-ready** patterns

---

## 🔧 Customization

### Adjusting Selectors:

Edit `config/settings.toml`:

```toml
[[sources]]
name = "My Custom Site"
url = "https://example.com"
type = "news"
rate_limit_ms = 3000

[sources.selectors]
container = ".article-wrapper"
title = "h1.headline"
content = "div.body p"
author = "span.author-name"
```

### Adding New Sources:

1. Add source to `config/sources.toml`
2. Choose appropriate source type (News/Ecommerce/Social)
3. Test selectors manually first
4. Adjust rate limits appropriately

---

## 📝 Notes

### Selector Tips:
- Use browser DevTools to identify CSS selectors
- Test selectors with `scraper` crate directly first
- Website structures change - examples may need selector updates
- Start broad, then narrow down selectors

### Troubleshooting:
- **No data scraped?** Check selectors with browser DevTools
- **Rate limited?** Increase `rate_limit_ms` in config
- **Timeout errors?** Increase `timeout_seconds`
- **Network errors?** Check internet connection

### Legal/Ethical:
- ✅ These examples use public, educational sources
- ✅ Respect robots.txt and rate limits
- ✅ Don't overload servers
- ⚠️ Always verify ToS before scraping new sites

---

## 🎓 Learning Path

1. Start with **news_scraper.rs** (simplest HTML scraping)
2. Try **ecommerce_scraper.rs** (structured data with prices)
3. Explore **social_scraper.rs** (JSON API pattern)
4. Master **multi_source_pipeline.rs** (complete workflow)

Each example builds on the previous concepts!

---

## 📚 Further Reading

- [Rust Scraper Pro Documentation](../README.md)
- [Configuration Guide](../config/README.md)
- [API Documentation](https://docs.rs/rust-scraper-pro)
- [Web Scraping Ethics](https://www.scraperapi.com/blog/web-scraping-ethics/)

---

**Built with Edition 2024** 🚀
All examples tested and verified with Rust 1.91.0+
