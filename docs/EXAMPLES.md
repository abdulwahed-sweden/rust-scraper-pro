# 📘 Examples

This directory contains **realistic, production-ready examples** using real public data sources. Each example demonstrates professional web scraping patterns with proper rate limiting and error handling.

---

## 🗂️ Available Examples

### 1. News Scraper - Hacker News

**File:** `examples/news_scraper.rs`
**Source:** [Hacker News](https://news.ycombinator.com/)
**Type:** HTML scraping with CSS selectors

```bash
cargo run --example news_scraper
```

**What it demonstrates:**
- Basic HTML parsing and CSS selector usage
- Extracting news headlines, authors, and links
- Processing data through validation pipeline
- Exporting to JSON and CSV

**Output files:**
- `output/hacker_news.json`
- `output/hacker_news.csv`

---

### 2. E-Commerce Scraper - Books Catalog

**File:** `examples/ecommerce_scraper.rs`
**Source:** [Books to Scrape](https://books.toscrape.com/)
**Type:** Product data extraction

```bash
cargo run --example ecommerce_scraper
```

**What it demonstrates:**
- Scraping structured product listings
- Extracting titles, prices, ratings, and images
- Calculating statistics (average price, total value)
- Handling e-commerce specific data

**Output files:**
- `output/books_catalog.json`
- `output/books_catalog.csv`

---

### 3. Social Media Scraper - Reddit API

**File:** `examples/social_scraper.rs`
**Source:** [Reddit JSON API](https://www.reddit.com/r/worldnews.json)
**Type:** JSON API scraping (no HTML parsing)

```bash
cargo run --example social_scraper
```

**What it demonstrates:**
- Consuming public JSON APIs
- Processing social media posts with metadata
- Extracting scores, comments, and authors
- Working with API responses directly

**Output files:**
- `output/reddit_worldnews.json`
- `output/reddit_worldnews.csv`

---

### 4. Multi-Source Pipeline - Combined Workflow

**File:** `examples/multi_source_pipeline.rs`
**Sources:** Hacker News + Books to Scrape
**Type:** Full end-to-end scraping pipeline

```bash
cargo run --example multi_source_pipeline
```

**What it demonstrates:**
- Scraping from multiple sources in one run
- Unified data processing pipeline
- Validation, normalization, and deduplication
- Cross-source data aggregation
- Professional workflow patterns

**Output files:**
- `output/multi_source_data.json`
- `output/multi_source_data.csv`

---

## 🎯 Data Sources

All examples use **real, publicly accessible sources**:

| Source | Type | Purpose | Rate Limit |
|--------|------|---------|------------|
| **Hacker News** | News | Tech news aggregator | 2s |
| **Books to Scrape** | E-commerce | Training website for scraping | 2s |
| **Reddit JSON API** | Social | Public posts via JSON | 3s |

These sources are:
- ✅ Publicly accessible (no authentication)
- ✅ Appropriate for educational use
- ✅ Designed for or tolerant of scraping
- ✅ Well-documented and maintained

---

## ⚙️ Configuration

Examples use settings from:
- `config/settings.toml` - Global configuration (rate limits, user agent)
- `config/sources.toml` - Source-specific selectors and settings

### Key Configuration:

```toml
[scraping]
rate_limit_ms = 2000  # Polite scraping: 2 seconds between requests
timeout_seconds = 30
user_agent = "Mozilla/5.0 (compatible; RustScraperPro/1.0; Educational)"
```

---

## 🚀 Quick Start

### Run an example:
```bash
cargo run --example news_scraper
```

### Build all examples:
```bash
cargo build --examples
```

### Run with optimizations:
```bash
cargo run --release --example multi_source_pipeline
```

---

## 📊 Expected Output

Each example generates:
- **JSON file** - Structured data for programmatic use
- **CSV file** - Spreadsheet-compatible format
- **Console logs** - Progress updates and statistics

Example output structure:

```json
{
  "id": "uuid-here",
  "source": "Hacker News",
  "url": "https://news.ycombinator.com/item?id=123",
  "title": "Example Article Title",
  "author": "username",
  "timestamp": "2025-11-03T21:00:00Z",
  "metadata": {},
  "category": "news"
}
```

---

## 🛡️ Best Practices

All examples follow ethical scraping practices:

- ✅ **Rate limiting** (2-3 seconds between requests)
- ✅ **Proper user agent** identification
- ✅ **Timeout handling** (30s max)
- ✅ **Error recovery** with retries
- ✅ **Cache utilization** to reduce server load
- ✅ **Respects robots.txt** (configurable)

---

## 🔧 Customization

### To scrape different subreddits:

Edit `examples/social_scraper.rs`:
```rust
let subreddit = "programming"; // Change from "worldnews"
let url = format!("https://www.reddit.com/r/{}.json", subreddit);
```

### To adjust rate limits:

Edit `config/settings.toml`:
```toml
rate_limit_ms = 3000  # Increase to 3 seconds
```

### To modify selectors:

Edit `config/settings.toml` or `config/sources.toml`:
```toml
[sources.selectors]
container = ".your-selector"
title = "h2.title"
```

---

## 📝 Notes

- **No authentication required** for any example
- **Real data** is scraped (not mocked)
- **Website structures may change** - selectors might need updates
- **Respectful scraping** - won't overload servers
- All examples tested with **Rust 1.91.0+** and **Edition 2024**

---

## 🎓 Learning Path

1. **Start with** `news_scraper.rs` (simplest HTML scraping)
2. **Move to** `ecommerce_scraper.rs` (structured product data)
3. **Explore** `social_scraper.rs` (JSON API pattern)
4. **Master** `multi_source_pipeline.rs` (complete workflow)

Each example builds on concepts from the previous one!

---

## 📚 Further Reading

- [Main README](../README.md)
- [Configuration Guide](../config/settings.toml)
- [API Documentation](https://docs.rs/scraper)

---

**Built with Edition 2024** 🦀🚀
