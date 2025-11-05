# PostgreSQL Database Integration Documentation

## Overview

Rust Scraper Pro now includes full PostgreSQL database integration for persistent storage of scraped data. This document describes the implementation, setup, and usage of the database features.

## Table of Contents

1. [Architecture](#architecture)
2. [Database Schema](#database-schema)
3. [Setup & Configuration](#setup--configuration)
4. [API Changes](#api-changes)
5. [Usage Examples](#usage-examples)
6. [Testing](#testing)
7. [Troubleshooting](#troubleshooting)

---

## Architecture

### Data Flow

```
Scraping Engine
     |
     v
Processing Pipeline
     |
     v
+-----------------+
| Dual Storage    |
|  1. PostgreSQL  | ← Primary (persistent)
|  2. In-Memory   | ← Fallback (temporary)
+-----------------+
     |
     v
Axum API Server
```

### Key Components

1. **PostgresOutput** (`src/output/database.rs`)
   - Connection pooling via `sqlx::PgPool`
   - CRUD operations for scraped data
   - Auto-migration (creates tables if missing)

2. **AppState** (`src/output/api.rs`)
   - Holds both in-memory data and database connection
   - Graceful degradation if database unavailable

3. **Main Application** (`src/main.rs`)
   - Initializes database connection on startup
   - Saves all scraped data to both storage layers

---

## Database Schema

### Table: `scraped_data`

```sql
CREATE TABLE IF NOT EXISTS scraped_data (
    id VARCHAR(255) PRIMARY KEY,
    source VARCHAR(255) NOT NULL,
    url TEXT NOT NULL,
    title TEXT,
    content TEXT,
    price DECIMAL(10,2),
    image_url TEXT,
    author VARCHAR(255),
    timestamp TIMESTAMPTZ NOT NULL,
    category VARCHAR(255),
    metadata JSONB,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Field Descriptions

| Field | Type | Description |
|-------|------|-------------|
| `id` | VARCHAR(255) | UUID primary key |
| `source` | VARCHAR(255) | Name of the scraping source |
| `url` | TEXT | URL of the scraped page |
| `title` | TEXT | Extracted title (nullable) |
| `content` | TEXT | Extracted content/description (nullable) |
| `price` | DECIMAL(10,2) | Price for e-commerce items (nullable) |
| `image_url` | TEXT | URL to associated image (nullable) |
| `author` | VARCHAR(255) | Author/creator name (nullable) |
| `timestamp` | TIMESTAMPTZ | When the data was scraped |
| `category` | VARCHAR(255) | Category/classification (nullable) |
| `metadata` | JSONB | Additional key-value metadata |
| `created_at` | TIMESTAMPTZ | Database insertion timestamp |

### Indexes (Recommended for Production)

```sql
-- Speed up queries by source
CREATE INDEX idx_scraped_data_source ON scraped_data(source);

-- Speed up time-based queries
CREATE INDEX idx_scraped_data_timestamp ON scraped_data(timestamp DESC);

-- Full-text search on title and content
CREATE INDEX idx_scraped_data_search ON scraped_data USING GIN (
    to_tsvector('english', COALESCE(title, '') || ' ' || COALESCE(content, ''))
);
```

---

## Setup & Configuration

### Prerequisites

- PostgreSQL 12+ installed and running
- Database user with CREATE privileges

### Step 1: Create Database

```bash
# Connect to PostgreSQL
psql -U postgres

# Create database
CREATE DATABASE rust_scraper_db;

# Grant permissions (if needed)
GRANT ALL PRIVILEGES ON DATABASE rust_scraper_db TO postgres;

# Exit
\q
```

Or via command line:

```bash
psql -U postgres -c "CREATE DATABASE rust_scraper_db;"
```

### Step 2: Configure Environment Variables

Create or update `.env`:

```env
# PostgreSQL connection string
DATABASE_URL=postgres://postgres:postgres@localhost/rust_scraper_db

# Alternative formats:
# With custom port: DATABASE_URL=postgres://user:pass@localhost:5433/dbname
# With SSL: DATABASE_URL=postgres://user:pass@localhost/dbname?sslmode=require

# Server configuration
SERVER_PORT=3000
SERVER_HOST=127.0.0.1
RUST_LOG=info,rust_scraper_pro=debug
```

### Step 3: Update Configuration File

Edit `config/settings.toml`:

```toml
[database]
type = "postgres"
url = "postgres://postgres:postgres@localhost/rust_scraper_db"
table_name = "scraped_data"
enabled = true
```

### Step 4: Install Dependencies

The required dependencies are already in `Cargo.toml`:

```toml
[dependencies]
sqlx = { version = "0.8.6", features = ["runtime-tokio-rustls", "postgres", "sqlite"] }
```

### Step 5: Run the Application

```bash
# Build and run
cargo run

# Or use the release build
cargo build --release
./target/release/rust-scraper-pro
```

The application will:
1. Attempt to connect to PostgreSQL
2. Create the `scraped_data` table if it doesn't exist
3. Fall back to in-memory storage if database is unavailable

---

## API Changes

### Enhanced Endpoints

All API endpoints now automatically query the database when available, with graceful fallback to in-memory data.

#### `GET /api/data`

**Before:** Returned only in-memory data
**Now:** Returns data from PostgreSQL (or in-memory if DB unavailable)

**Query Parameters:**
- `limit` (optional): Maximum number of results (default: 100)
- `offset` (optional): Pagination offset (default: 0)
- `query` (optional): Search in title/content
- `source` (optional): Filter by source
- `category` (optional): Filter by category

**Example:**
```bash
curl "http://localhost:3000/api/data?limit=10&offset=0"
```

#### `GET /api/sources`

**Now:** Returns unique sources from database

**Example:**
```bash
curl "http://localhost:3000/api/sources"
```

**Response:**
```json
["Reddit Frontpage", "Hacker News", "Books to Scrape"]
```

#### `GET /api/health`

Enhanced with database status:

```bash
curl "http://localhost:3000/api/health"
```

**Response:**
```json
{
  "status": "healthy",
  "service": "rust-scraper-pro",
  "version": "1.0.0"
}
```

### New Database Methods

**PostgresOutput** provides these public methods:

```rust
// Get all data with pagination
pub async fn get_all(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<ScrapedData>>

// Search data
pub async fn search(&self, query: &str, source: Option<&str>, limit: Option<i64>) -> Result<Vec<ScrapedData>>

// Get record count
pub async fn count(&self) -> Result<i64>

// Get unique sources
pub async fn get_sources(&self) -> Result<Vec<String>>

// Save data (from DatabaseOutput trait)
async fn save(&self, data: &[ScrapedData]) -> Result<usize>

// Initialize database (from DatabaseOutput trait)
async fn init(&self) -> Result<()>

// Clear all data (from DatabaseOutput trait)
async fn clear(&self) -> Result<()>
```

---

## Usage Examples

### Example 1: Basic Scraping with Database Persistence

```rust
use rust_scraper_pro::{
    core::scraper::ScraperEngine,
    output::database::{DatabaseOutput, PostgresOutput},
    processors::pipeline::ProcessingPipeline,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Connect to database
    let db = PostgresOutput::new(
        "postgres://postgres:postgres@localhost/rust_scraper_db",
        None
    ).await?;

    // Initialize schema
    db.init().await?;

    // ... perform scraping ...
    let scraped_data = vec![/* your scraped data */];

    // Save to database
    let count = db.save(&scraped_data).await?;
    println!("Saved {} items to database", count);

    Ok(())
}
```

### Example 2: Querying Data from Database

```rust
use rust_scraper_pro::output::database::PostgresOutput;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = PostgresOutput::new(
        "postgres://postgres:postgres@localhost/rust_scraper_db",
        None
    ).await?;

    // Get latest 50 items
    let items = db.get_all(Some(50), Some(0)).await?;
    println!("Retrieved {} items", items.len());

    // Search for specific content
    let search_results = db.search("rust", Some("Hacker News"), Some(10)).await?;
    println!("Found {} matching items", search_results.len());

    // Get statistics
    let total = db.count().await?;
    let sources = db.get_sources().await?;
    println!("Total items: {}, Sources: {:?}", total, sources);

    Ok(())
}
```

### Example 3: Testing API Endpoints

```bash
# Get all data from database
curl "http://localhost:3000/api/data"

# Get paginated data
curl "http://localhost:3000/api/data?limit=20&offset=0"

# Search for content
curl "http://localhost:3000/api/search?query=rust&limit=10"

# Get sources
curl "http://localhost:3000/api/sources"

# Get statistics
curl "http://localhost:3000/api/stats"

# Export as JSON
curl "http://localhost:3000/api/export/json" > data.json

# Export as CSV
curl "http://localhost:3000/api/export/csv" > data.csv"
```

### Example 4: Direct SQL Queries

```sql
-- Get recent items
SELECT id, source, title, timestamp
FROM scraped_data
ORDER BY timestamp DESC
LIMIT 10;

-- Count items by source
SELECT source, COUNT(*) as count
FROM scraped_data
GROUP BY source
ORDER BY count DESC;

-- Search for specific content
SELECT title, content, url
FROM scraped_data
WHERE title ILIKE '%rust%' OR content ILIKE '%rust%'
LIMIT 10;

-- Get price statistics (for e-commerce)
SELECT
    source,
    COUNT(*) as total_items,
    AVG(price) as avg_price,
    MIN(price) as min_price,
    MAX(price) as max_price
FROM scraped_data
WHERE price IS NOT NULL
GROUP BY source;
```

---

## Testing

### Manual Testing Steps

1. **Verify Database Connection**

```bash
# Run the application
cargo run

# Check logs for:
# [INFO] Connected to PostgreSQL database
# [INFO] Database schema initialized successfully
```

2. **Verify Table Creation**

```bash
psql -U postgres -d rust_scraper_db -c "\dt"
# Should show: scraped_data table

psql -U postgres -d rust_scraper_db -c "\d scraped_data"
# Should show all columns
```

3. **Verify Data Insertion**

```bash
# After scraping completes, check the database:
psql -U postgres -d rust_scraper_db -c "SELECT COUNT(*) FROM scraped_data;"

# View sample data:
psql -U postgres -d rust_scraper_db -c "SELECT id, source, title FROM scraped_data LIMIT 5;"
```

4. **Verify API Access**

```bash
# Test data endpoint
curl -s http://localhost:3000/api/data | jq '.[0]'

# Test sources endpoint
curl -s http://localhost:3000/api/sources | jq '.'

# Test stats endpoint
curl -s http://localhost:3000/api/stats | jq '.'
```

### Automated Testing

Create `tests/database_integration_test.rs`:

```rust
#[cfg(test)]
mod database_tests {
    use rust_scraper_pro::{
        core::models::ScrapedData,
        output::database::{DatabaseOutput, PostgresOutput},
    };

    #[tokio::test]
    async fn test_database_save_and_retrieve() {
        // Setup
        let db = PostgresOutput::new(
            "postgres://postgres:postgres@localhost/rust_scraper_test",
            None
        ).await.expect("Failed to connect");

        db.init().await.expect("Failed to initialize");

        // Create test data
        let test_data = vec![
            ScrapedData::new("Test Source".to_string(), "https://example.com".to_string())
        ];

        // Save
        let count = db.save(&test_data).await.expect("Failed to save");
        assert_eq!(count, 1);

        // Retrieve
        let retrieved = db.get_all(Some(10), Some(0)).await.expect("Failed to retrieve");
        assert!(!retrieved.is_empty());

        // Cleanup
        db.clear().await.expect("Failed to clear");
    }
}
```

Run tests:
```bash
cargo test --test database_integration_test
```

---

## Troubleshooting

### Issue: Cannot Connect to PostgreSQL

**Symptoms:**
```
[WARN] Failed to connect to PostgreSQL: error connecting to server
[WARN] Continuing without database persistence
```

**Solutions:**

1. **Verify PostgreSQL is running:**
```bash
# macOS
brew services list | grep postgresql

# Linux
sudo systemctl status postgresql

# Check if PostgreSQL is listening
psql -U postgres -c "SELECT version();"
```

2. **Check connection string:**
```bash
# Test connection manually
psql -U postgres -d rust_scraper_db

# If that works, verify DATABASE_URL in .env
echo $DATABASE_URL
```

3. **Check PostgreSQL logs:**
```bash
# macOS
tail -f /usr/local/var/log/postgres.log

# Linux
sudo tail -f /var/log/postgresql/postgresql-*.log
```

---

### Issue: Table Already Exists Error

**Symptoms:**
```
[ERROR] Failed to initialize database schema: table "scraped_data" already exists
```

**Solutions:**

1. **Drop and recreate table:**
```sql
psql -U postgres -d rust_scraper_db -c "DROP TABLE IF EXISTS scraped_data;"
```

2. **Or modify table schema if needed:**
```sql
ALTER TABLE scraped_data ADD COLUMN IF NOT EXISTS new_column TEXT;
```

---

### Issue: Permission Denied

**Symptoms:**
```
[ERROR] permission denied for database rust_scraper_db
```

**Solutions:**

```sql
-- Grant all privileges
psql -U postgres -c "GRANT ALL PRIVILEGES ON DATABASE rust_scraper_db TO your_user;"

-- Grant table permissions
psql -U postgres -d rust_scraper_db -c "GRANT ALL ON ALL TABLES IN SCHEMA public TO your_user;"
```

---

### Issue: Slow Queries

**Symptoms:**
- API responses take >1 second
- High CPU usage from PostgreSQL

**Solutions:**

1. **Add indexes:**
```sql
CREATE INDEX idx_scraped_data_source ON scraped_data(source);
CREATE INDEX idx_scraped_data_timestamp ON scraped_data(timestamp DESC);
```

2. **Analyze query performance:**
```sql
EXPLAIN ANALYZE SELECT * FROM scraped_data WHERE source = 'Reddit' LIMIT 100;
```

3. **Increase connection pool:**
```rust
// In src/output/database.rs
let pool = PgPoolOptions::new()
    .max_connections(10)  // Increase from 5
    .connect(connection_string)
    .await?;
```

---

### Issue: Data Not Showing in API

**Symptoms:**
- Database has data but API returns empty array

**Solutions:**

1. **Check application logs:**
```
[INFO] Retrieved 10 items from database
```

2. **Verify API is using database:**
```bash
# Check the /api/data endpoint
curl -v http://localhost:3000/api/data

# Look for "Retrieved ... items from database" in logs
```

3. **Manually query database:**
```sql
psql -U postgres -d rust_scraper_db -c "SELECT COUNT(*) FROM scraped_data;"
```

---

## Performance Considerations

### Connection Pooling

The application uses `sqlx::PgPool` with 5 concurrent connections by default. Adjust if needed:

```rust
// src/output/database.rs
let pool = PgPoolOptions::new()
    .max_connections(10)
    .min_connections(2)
    .acquire_timeout(Duration::from_secs(3))
    .connect(connection_string)
    .await?;
```

### Query Optimization

1. **Use pagination:** Always use `LIMIT` and `OFFSET` for large datasets
2. **Add indexes:** Create indexes on frequently queried columns
3. **Avoid SELECT *:** Select only needed columns for better performance

### Caching Strategy

The application maintains both database and in-memory storage:
- **Database:** Persistent, survives restarts
- **In-Memory:** Fast access, temporary

For best performance:
- Use database for historical queries
- Use in-memory for latest/recent data

---

## Migration from File-Based Storage

If you were previously using JSON/CSV only:

1. **Keep existing export functionality:**
   - JSON export still works (`/api/export/json`)
   - CSV export still works (`/api/export/csv`)

2. **Import existing data:**

```bash
# Export from JSON
cat output/data.json | jq -c '.[]' | while read item; do
    curl -X POST http://localhost:3000/api/update \
         -H "Content-Type: application/json" \
         -d "[$item]"
done
```

Or use a script:

```rust
use rust_scraper_pro::{
    core::models::ScrapedData,
    output::database::{DatabaseOutput, PostgresOutput},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Read JSON file
    let json_data = std::fs::read_to_string("output/data.json")?;
    let data: Vec<ScrapedData> = serde_json::from_str(&json_data)?;

    // Connect to database
    let db = PostgresOutput::new(
        "postgres://postgres:postgres@localhost/rust_scraper_db",
        None
    ).await?;

    db.init().await?;

    // Import all data
    let count = db.save(&data).await?;
    println!("Imported {} items", count);

    Ok(())
}
```

---

## Security Best Practices

1. **Never commit .env:**
   - Already in `.gitignore`
   - Use `.env.example` for templates

2. **Use strong passwords:**
```env
DATABASE_URL=postgres://user:STRONG_PASSWORD_HERE@localhost/rust_scraper_db
```

3. **Enable SSL in production:**
```env
DATABASE_URL=postgres://user:pass@localhost/dbname?sslmode=require
```

4. **Limit database permissions:**
```sql
-- Create dedicated user
CREATE USER scraper_app WITH PASSWORD 'password';
GRANT CONNECT ON DATABASE rust_scraper_db TO scraper_app;
GRANT SELECT, INSERT, UPDATE, DELETE ON scraped_data TO scraper_app;
```

5. **Use connection pooling limits:**
```rust
.max_connections(5)  // Prevent connection exhaustion
```

---

## Future Enhancements

Potential improvements for the database integration:

- [ ] Add database migrations using `sqlx migrate`
- [ ] Implement soft deletes with `deleted_at` column
- [ ] Add full-text search with PostgreSQL `tsvector`
- [ ] Implement database backup/restore utilities
- [ ] Add database health metrics to `/api/health`
- [ ] Support for multiple databases (sharding)
- [ ] Add database query caching layer
- [ ] Implement database replication support

---

## Summary

The PostgreSQL integration provides:

✅ Persistent storage of scraped data
✅ Automatic schema migration
✅ Graceful degradation (fallback to in-memory)
✅ High-performance querying
✅ Production-ready error handling
✅ Full API compatibility
✅ Easy configuration via `.env`

The integration is designed to be **optional** - the application works perfectly fine without a database connection, using in-memory storage as a fallback.

---

## Support

For issues or questions:
- Check the [Troubleshooting](#troubleshooting) section
- Review PostgreSQL logs: `/var/log/postgresql/`
- Check application logs: `RUST_LOG=debug cargo run`
- Review `INTEGRATION.md` for overall architecture

**Database Integration Version:** 1.0
**Last Updated:** November 2025
