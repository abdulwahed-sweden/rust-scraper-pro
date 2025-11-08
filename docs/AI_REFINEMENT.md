# AI Refinement Layer - DeepSeek Integration

**Status:** ✅ Production Ready  
**API:** DeepSeek Chat API  
**Purpose:** Intelligent data normalization and selector detection

---

## Overview

The AI Refinement Layer transforms `rust-scraper-pro` from a traditional web scraper into an **AI-Powered Web Intelligence Framework**. It leverages DeepSeek's advanced language model to:

1. **Automatically detect CSS selectors** for new websites
2. **Normalize and refine** multi-source data before storage
3. **Remove duplicates** and invalid entries intelligently
4. **Standardize field formats** across different data sources

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Web Sources                               │
│   (books.toscrape.com, news sites, e-commerce, etc.)        │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              Scraper Engine (Rust + Adaptive Delay)         │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                 AI Selector Assistant                       │
│  ┌──────────────────────────────────────────────┐          │
│  │  1. Analyze HTML structure                   │          │
│  │  2. Send to DeepSeek API                     │          │
│  │  3. Receive recommended selectors             │          │
│  │  4. Save to selectors/<domain>.json          │          │
│  └──────────────────────────────────────────────┘          │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                   Raw Scraped Data                          │
│        (JSON/CSV with inconsistent formats)                 │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              AI Data Normalizer (DeepSeek)                  │
│  ┌──────────────────────────────────────────────┐          │
│  │  1. Aggregate multi-source data              │          │
│  │  2. Send to DeepSeek for normalization       │          │
│  │  3. Standardize field names & formats        │          │
│  │  4. Convert currencies (GBP→USD, EUR→USD)    │          │
│  │  5. Remove duplicates & invalid records      │          │
│  │  6. Return unified JSON schema                │          │
│  └──────────────────────────────────────────────┘          │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│              Unified & Clean Data                           │
│           (data/normalized/final.json)                      │
└────────────────────────┬────────────────────────────────────┘
                         │
                         ▼
┌─────────────────────────────────────────────────────────────┐
│                PostgreSQL Database                          │
│    (Production-ready, standardized schema)                  │
└─────────────────────────────────────────────────────────────┘
```

---

## Components

### 1. DeepSeek API Client

**File:** `src/ai/deepseek_client.rs`

**Features:**
- Secure API key management from `.env`
- Automatic error handling and retries
- Token usage tracking
- Connection testing

**Usage:**

```rust
use rust_scraper_pro::ai::DeepSeekClient;

// Initialize client (reads DEEPSEEK_API_KEY from .env)
let client = DeepSeekClient::new()?;

// Test connection
client.test_connection().await?;

// Simple prompt
let response = client.ask("What is web scraping?").await?;

// With system instructions
let response = client.ask_with_system(
    "You are an expert CSS selector generator.",
    "Analyze this HTML and suggest selectors..."
).await?;
```

### 2. Selector Assistant

**File:** `src/ai/selector_assistant.rs`

**Purpose:** Automatically detect optimal CSS selectors for new websites using AI.

**Features:**
- Analyzes HTML structure intelligently
- Recommends stable, maintainable selectors
- Caches selectors per domain (saves API calls)
- Provides confidence scores

**Usage:**

```rust
use rust_scraper_pro::ai::{DeepSeekClient, SelectorAssistant};

let client = DeepSeekClient::new()?;
let assistant = SelectorAssistant::new(client);

// Analyze a new website
let html_sample = "<html>...</html>";
let selectors = assistant.detect_selectors(
    "example.com",
    html_sample
).await?;

// Save to selectors/example.com.selectors.json
assistant.save_selectors(&selectors).await?;

// Later: load from cache
let cached = assistant.load_selectors("example.com").await?;
```

**Output Format:**

```json
{
  "domain": "example.com",
  "title": "h1.product-title",
  "price": "span.price-value",
  "image": "img.product-image",
  "category": "div.category-name",
  "confidence": 0.92,
  "generated_at": "2025-11-08T12:00:00Z"
}
```

### 3. Data Normalizer

**File:** `src/ai/normalizer.rs`

**Purpose:** AI-powered data cleaning and standardization before database insertion.

**Features:**
- Batch processing (configurable batch size)
- Currency conversion (GBP→USD, EUR→USD)
- Deduplication based on title + source
- Field name standardization
- Invalid record removal

**Usage:**

```rust
use rust_scraper_pro::ai::{DeepSeekClient, DataNormalizer};

let client = DeepSeekClient::new()?;
let normalizer = DataNormalizer::new(client)
    .with_batch_size(50);

// Normalize scraped data
let (normalized, stats) = normalizer
    .normalize_all(scraped_data)
    .await?;

println!("Normalized {} → {} items", stats.total_input, stats.total_output);
println!("Removed {} duplicates", stats.duplicates_removed);

// Save to file
normalizer.save_to_json(&normalized, "data/normalized/final.json").await?;
```

**Normalization Rules:**

1. **Field Standardization:**
   - `cost`, `price_value`, `amount` → `price_usd`
   - `img`, `thumbnail`, `picture` → `image`
   - `name`, `heading` → `title`

2. **Currency Conversion:**
   - `£51.77` → `65.75` USD (GBP * 1.27)
   - `€100.00` → `108.00` USD (EUR * 1.08)

3. **Deduplication:**
   - Removes exact matches based on `title + source`

4. **Quality Checks:**
   - Removes items without titles
   - Validates URL formats
   - Trims whitespace

**Output Schema:**

```json
[
  {
    "id": "uuid-here",
    "title": "A Light in the Attic",
    "price_usd": 65.75,
    "image": "https://example.com/image.jpg",
    "category": "Books",
    "source": "Books to Scrape",
    "timestamp": "2025-11-08T12:00:00Z",
    "metadata": {
      "rating": "Five stars",
      "availability": "In stock"
    }
  }
]
```

---

## Configuration

**File:** `config/settings.toml`

```toml
[ai]
enabled = true
deepseek_model = "deepseek-chat"
enable_selector_assistant = true
enable_normalizer = true
normalizer_batch_size = 50
```

**Environment Variables:**

```bash
# .env file
DEEPSEEK_API_KEY=sk-your-api-key-here
```

**⚠️ Security:**
- Never commit `.env` to version control
- API key is already protected in `.gitignore`
- Use environment variables in production

---

## API Integration Examples

### Example 1: Detect Selectors for New Site

```rust
use rust_scraper_pro::ai::{DeepSeekClient, SelectorAssistant};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize
    let client = DeepSeekClient::new()?;
    let assistant = SelectorAssistant::new(client);

    // Fetch sample HTML from target site
    let html = reqwest::get("https://example-shop.com")
        .await?
        .text()
        .await?;

    // Detect selectors using AI
    let selectors = assistant
        .detect_selectors("example-shop.com", &html)
        .await?;

    println!("Detected selectors with {:.0}% confidence:",
        selectors.confidence * 100.0);
    println!("  Title: {:?}", selectors.title);
    println!("  Price: {:?}", selectors.price);
    println!("  Image: {:?}", selectors.image);

    // Save for future use
    assistant.save_selectors(&selectors).await?;

    Ok(())
}
```

### Example 2: Normalize Multi-Source Data

```rust
use rust_scraper_pro::{
    core::scraper::ScraperEngine,
    ai::{DeepSeekClient, DataNormalizer},
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Scrape from multiple sources
    let mut engine = ScraperEngine::new(...);
    let raw_data = vec![
        engine.scrape_source(source1).await?,
        engine.scrape_source(source2).await?,
        engine.scrape_source(source3).await?,
    ].concat();

    println!("Scraped {} raw items from multiple sources", raw_data.len());

    // Normalize with AI
    let client = DeepSeekClient::new()?;
    let normalizer = DataNormalizer::new(client);
    
    let (normalized, stats) = normalizer
        .normalize_all(raw_data)
        .await?;

    println!("Normalization complete:");
    println!("  Input:  {} items", stats.total_input);
    println!("  Output: {} items", stats.total_output);
    println!("  Duplicates removed: {}", stats.duplicates_removed);

    // Save to file
    normalizer
        .save_to_json(&normalized, "data/normalized/final.json")
        .await?;

    Ok(())
}
```

### Example 3: Complete AI Pipeline

```rust
use rust_scraper_pro::{
    core::{config::Config, scraper::ScraperEngine},
    processors::pipeline::ProcessingPipeline,
    ai::{DeepSeekClient, SelectorAssistant, DataNormalizer},
    output::database::PostgresOutput,
};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Setup
    let config = Config::load("config/settings.toml").await?;
    let pipeline = ProcessingPipeline::new();
    let mut engine = ScraperEngine::new(config, pipeline, None);
    
    let deepseek = DeepSeekClient::new()?;
    let assistant = SelectorAssistant::new(deepseek.clone());
    let normalizer = DataNormalizer::new(deepseek);

    // 2. Auto-detect selectors for new site
    let html = reqwest::get("https://newsite.com").await?.text().await?;
    let selectors = assistant
        .get_or_detect_selectors("newsite.com", &html)
        .await?;
    
    println!("Using selectors: {:?}", selectors);

    // 3. Scrape data
    let raw_data = engine.scrape_source(my_source).await?;
    
    // 4. AI normalization
    let (clean_data, stats) = normalizer.normalize_all(raw_data).await?;
    
    // 5. Save to database
    let db = PostgresOutput::new(&std::env::var("DATABASE_URL")?, None).await?;
    db.save(&clean_data).await?;

    println!("✅ AI pipeline complete!");
    println!("   {} items processed and saved", stats.total_output);

    Ok(())
}
```

---

## Performance & Cost

### DeepSeek API Pricing

- **Model:** `deepseek-chat`
- **Pricing:** ~$0.14 per 1M input tokens, ~$0.28 per 1M output tokens
- **Typical Usage:**
  - Selector detection: ~2000 tokens (~$0.0006 per site)
  - Data normalization: ~1000 tokens per 50 items (~$0.0003 per batch)

### Optimization Tips

1. **Cache Selectors:** Use `get_or_detect_selectors()` to avoid re-detecting
2. **Batch Processing:** Process data in batches of 50-100 items
3. **Fallback Mode:** Use `normalize_simple()` for simple normalization without AI
4. **Rate Limiting:** Built-in 500ms delay between batches

---

## Error Handling

All AI functions return `Result<T>` with descriptive errors:

```rust
match normalizer.normalize_all(data).await {
    Ok((normalized, stats)) => {
        println!("Success: {} items", stats.total_output);
    }
    Err(e) => {
        eprintln!("Normalization error: {}", e);
        // Fallback to simple normalization
        let simple = DataNormalizer::normalize_simple(data);
    }
}
```

**Common Errors:**
- `DEEPSEEK_API_KEY not set` - Add to `.env`
- `Failed to parse response` - Check API response format
- `Rate limit exceeded` - Increase delay between batches

---

## Testing

Test the DeepSeek connection:

```rust
use rust_scraper_pro::ai::DeepSeekClient;

#[tokio::main]
async fn main() {
    let client = DeepSeekClient::new().expect("Failed to create client");
    
    match client.test_connection().await {
        Ok(_) => println!("✅ DeepSeek API connected successfully!"),
        Err(e) => eprintln!("❌ Connection failed: {}", e),
    }
}
```

**Run integration tests:**

```bash
# Make sure DEEPSEEK_API_KEY is set
export DEEPSEEK_API_KEY=sk-your-key

# Test AI features
cargo test --features ai_tests

# Test with verbose output
cargo test -- --nocapture
```

---

## Benefits

### 1. Automatic Selector Maintenance
- No manual CSS selector updates when sites change
- AI adapts to new HTML structures
- Confidence scores help identify when manual review is needed

### 2. Multi-Source Data Unification
- Scrape from 10 different e-commerce sites
- AI standardizes all data to a unified schema
- Removes duplicates across sources intelligently

### 3. Production-Ready Data
- Clean, normalized data ready for analytics
- Consistent field names and formats
- Currency conversions handled automatically

### 4. Reduced Maintenance
- Fewer manual interventions
- Automatic adaptation to site changes
- Self-documenting selector files

---

## Future Enhancements

- [ ] Add support for multiple AI providers (OpenAI, Claude, Gemini)
- [ ] Implement automatic selector validation and testing
- [ ] Add confidence threshold alerts
- [ ] Support for custom normalization rules
- [ ] Real-time selector adjustment based on scrape success rates

---

## Conclusion

The AI Refinement Layer transforms rust-scraper-pro into a next-generation web intelligence platform that:

- ✅ Intelligently adapts to website changes
- ✅ Automatically detects and optimizes selectors
- ✅ Unifies data from multiple sources
- ✅ Delivers clean, production-ready datasets

**Ready to get started? See:** `SCRAPER_ADAPTIVE_MODE.md` for adaptive delay configuration.

---

**Generated:** November 8, 2025  
**Version:** 1.0.0  
**Powered by:** DeepSeek API
