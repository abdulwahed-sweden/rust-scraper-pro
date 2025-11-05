use crate::core::models::ScrapedData;
use anyhow::{Context, Result};
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, SqlitePool, Row};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[async_trait]
pub trait DatabaseOutput {
    async fn init(&self) -> Result<()>;
    async fn save(&self, data: &[ScrapedData]) -> Result<usize>;
    async fn query(&self, query: &str) -> Result<Vec<ScrapedData>>;
    async fn clear(&self) -> Result<()>;
}

pub struct PostgresOutput {
    pool: Pool<Postgres>,
    table_name: String,
}

impl PostgresOutput {
    pub async fn new(connection_string: &str, table_name: Option<&str>) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection_string)
            .await
            .context("Failed to connect to PostgreSQL database")?;

        Ok(Self {
            pool,
            table_name: table_name.unwrap_or("scraped_data").to_string(),
        })
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }

    async fn create_table(&self) -> Result<()> {
        let query = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
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
            )
            "#,
            self.table_name
        );

        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .context("Failed to create database table")?;
        Ok(())
    }

    /// Get all scraped data with optional limit and offset
    pub async fn get_all(&self, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<ScrapedData>> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);

        let query = format!(
            r#"
            SELECT id, source, url, title, content, price, image_url, author,
                   timestamp, category, metadata
            FROM {}
            ORDER BY timestamp DESC
            LIMIT $1 OFFSET $2
            "#,
            self.table_name
        );

        let rows = sqlx::query(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch data from database")?;

        let mut results = Vec::new();
        for row in rows {
            let metadata_json: serde_json::Value = row.try_get("metadata").unwrap_or(serde_json::json!({}));
            let metadata: HashMap<String, String> = serde_json::from_value(metadata_json).unwrap_or_default();

            // PostgreSQL returns DateTime<Utc> directly for TIMESTAMPTZ
            let timestamp: DateTime<Utc> = row.try_get("timestamp")?;

            results.push(ScrapedData {
                id: row.try_get("id")?,
                source: row.try_get("source")?,
                url: row.try_get("url")?,
                title: row.try_get("title")?,
                content: row.try_get("content")?,
                price: row.try_get("price")?,
                image_url: row.try_get("image_url")?,
                author: row.try_get("author")?,
                timestamp,
                category: row.try_get("category")?,
                metadata,
            });
        }

        Ok(results)
    }

    /// Search scraped data by query string
    pub async fn search(&self, query_str: &str, source_filter: Option<&str>, limit: Option<i64>) -> Result<Vec<ScrapedData>> {
        let limit = limit.unwrap_or(50);

        let query = if let Some(_source) = source_filter {
            format!(
                r#"
                SELECT id, source, url, title, content, price, image_url, author,
                       timestamp, category, metadata
                FROM {}
                WHERE (title ILIKE $1 OR content ILIKE $1) AND source = $2
                ORDER BY timestamp DESC
                LIMIT $3
                "#,
                self.table_name
            )
        } else {
            format!(
                r#"
                SELECT id, source, url, title, content, price, image_url, author,
                       timestamp, category, metadata
                FROM {}
                WHERE title ILIKE $1 OR content ILIKE $1
                ORDER BY timestamp DESC
                LIMIT $2
                "#,
                self.table_name
            )
        };

        let search_pattern = format!("%{}%", query_str);

        let rows = if let Some(source) = source_filter {
            sqlx::query(&query)
                .bind(&search_pattern)
                .bind(source)
                .bind(limit)
                .fetch_all(&self.pool)
                .await
                .context("Failed to search database with source filter")?
        } else {
            sqlx::query(&query)
                .bind(&search_pattern)
                .bind(limit)
                .fetch_all(&self.pool)
                .await
                .context("Failed to search database")?
        };

        let mut results = Vec::new();
        for row in rows {
            let metadata_json: serde_json::Value = row.try_get("metadata").unwrap_or(serde_json::json!({}));
            let metadata: HashMap<String, String> = serde_json::from_value(metadata_json).unwrap_or_default();

            // PostgreSQL returns DateTime<Utc> directly for TIMESTAMPTZ
            let timestamp: DateTime<Utc> = row.get("timestamp");

            results.push(ScrapedData {
                id: row.get("id"),
                source: row.get("source"),
                url: row.get("url"),
                title: row.get("title"),
                content: row.get("content"),
                price: row.get("price"),
                image_url: row.get("image_url"),
                author: row.get("author"),
                timestamp,
                category: row.get("category"),
                metadata,
            });
        }

        Ok(results)
    }

    /// Get count of all records
    pub async fn count(&self) -> Result<i64> {
        let query = format!("SELECT COUNT(*) as count FROM {}", self.table_name);
        let row = sqlx::query(&query)
            .fetch_one(&self.pool)
            .await
            .context("Failed to count records in database")?;
        let count: i64 = row.get("count");
        Ok(count)
    }

    /// Get unique sources
    pub async fn get_sources(&self) -> Result<Vec<String>> {
        let query = format!(
            "SELECT DISTINCT source FROM {} ORDER BY source",
            self.table_name
        );
        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .context("Failed to get unique sources from database")?;

        let mut sources = Vec::new();
        for row in rows {
            sources.push(row.get("source"));
        }

        Ok(sources)
    }
}

#[async_trait]
impl DatabaseOutput for PostgresOutput {
    async fn init(&self) -> Result<()> {
        self.create_table().await?;
        log::info!("PostgreSQL table '{}' initialized", self.table_name);
        Ok(())
    }

    async fn save(&self, data: &[ScrapedData]) -> Result<usize> {
        let mut count = 0;

        for item in data {
            let query = format!(
                r#"
                INSERT INTO {} (id, source, url, title, content, price, image_url, author, timestamp, category, metadata)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT (id) DO UPDATE SET
                    title = EXCLUDED.title,
                    content = EXCLUDED.content,
                    price = EXCLUDED.price,
                    metadata = EXCLUDED.metadata,
                    updated_at = NOW()
                "#,
                self.table_name
            );

            let metadata_json = serde_json::to_value(&item.metadata)?;

            let result = sqlx::query(&query)
                .bind(&item.id)
                .bind(&item.source)
                .bind(&item.url)
                .bind(&item.title.as_deref())
                .bind(&item.content.as_deref())
                .bind(item.price)
                .bind(&item.image_url.as_deref())
                .bind(&item.author.as_deref())
                .bind(&item.timestamp)
                .bind(&item.category.as_deref())
                .bind(&metadata_json)
                .execute(&self.pool)
                .await
                .context(format!("Failed to save item with id: {}", item.id))?;

            count += result.rows_affected() as usize;
        }

        log::info!("Saved {} items to PostgreSQL", count);
        Ok(count)
    }

    async fn query(&self, _query: &str) -> Result<Vec<ScrapedData>> {
        // TODO: Implement proper query with FromRow derive for ScrapedData
        log::warn!("Query method not yet implemented for PostgreSQL");
        Ok(Vec::new())
    }

    async fn clear(&self) -> Result<()> {
        let query = format!("DELETE FROM {}", self.table_name);
        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .context("Failed to clear database table")?;
        log::info!("Cleared PostgreSQL table '{}'", self.table_name);
        Ok(())
    }
}

pub struct SqliteOutput {
    pool: SqlitePool,
    table_name: String,
}

impl SqliteOutput {
    pub async fn new(db_path: &str, table_name: Option<&str>) -> Result<Self> {
        let pool = SqlitePool::connect(db_path)
            .await
            .context("Failed to connect to SQLite database")?;

        Ok(Self {
            pool,
            table_name: table_name.unwrap_or("scraped_data").to_string(),
        })
    }

    async fn create_table(&self) -> Result<()> {
        let query = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                id TEXT PRIMARY KEY,
                source TEXT NOT NULL,
                url TEXT NOT NULL,
                title TEXT,
                content TEXT,
                price REAL,
                image_url TEXT,
                author TEXT,
                timestamp DATETIME NOT NULL,
                category TEXT,
                metadata TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
            self.table_name
        );

        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .context("Failed to create SQLite table")?;
        Ok(())
    }
}

#[async_trait]
impl DatabaseOutput for SqliteOutput {
    async fn init(&self) -> Result<()> {
        self.create_table().await?;
        log::info!("SQLite table '{}' initialized", self.table_name);
        Ok(())
    }

    async fn save(&self, data: &[ScrapedData]) -> Result<usize> {
        let mut count = 0;
        let mut transaction = self.pool.begin()
            .await
            .context("Failed to begin SQLite transaction")?;

        for item in data {
            let query = format!(
                r#"
                INSERT INTO {} (id, source, url, title, content, price, image_url, author, timestamp, category, metadata)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(id) DO UPDATE SET
                    title = excluded.title,
                    content = excluded.content,
                    price = excluded.price,
                    metadata = excluded.metadata,
                    updated_at = CURRENT_TIMESTAMP
                "#,
                self.table_name
            );

            let metadata_json = serde_json::to_string(&item.metadata)?;

            let result = sqlx::query(&query)
                .bind(&item.id)
                .bind(&item.source)
                .bind(&item.url)
                .bind(&item.title.as_deref())
                .bind(&item.content.as_deref())
                .bind(item.price)
                .bind(&item.image_url.as_deref())
                .bind(&item.author.as_deref())
                .bind(item.timestamp.to_rfc3339())
                .bind(&item.category.as_deref())
                .bind(&metadata_json)
                .execute(&mut *transaction)
                .await
                .context(format!("Failed to save item to SQLite: {}", item.id))?;

            count += result.rows_affected() as usize;
        }

        transaction.commit()
            .await
            .context("Failed to commit SQLite transaction")?;
        log::info!("Saved {} items to SQLite", count);
        Ok(count)
    }

    async fn query(&self, _query: &str) -> Result<Vec<ScrapedData>> {
        // TODO: Implement proper query with FromRow derive for ScrapedData
        log::warn!("Query method not yet implemented for SQLite");
        Ok(Vec::new())
    }

    async fn clear(&self) -> Result<()> {
        let query = format!("DELETE FROM {}", self.table_name);
        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .context("Failed to clear SQLite table")?;
        log::info!("Cleared SQLite table '{}'", self.table_name);
        Ok(())
    }
}