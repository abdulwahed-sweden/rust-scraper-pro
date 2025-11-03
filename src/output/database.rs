use crate::core::models::ScrapedData;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Sqlite, SqlitePool};
use std::path::Path;

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
            .await?;

        Ok(Self {
            pool,
            table_name: table_name.unwrap_or("scraped_data").to_string(),
        })
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

        sqlx::query(&query).execute(&self.pool).await?;
        Ok(())
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
                .bind(item.timestamp.to_rfc3339())
                .bind(&item.category.as_deref())
                .bind(&metadata_json)
                .execute(&self.pool)
                .await?;

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
        sqlx::query(&query).execute(&self.pool).await?;
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
        let pool = SqlitePool::connect(db_path).await?;

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

        sqlx::query(&query).execute(&self.pool).await?;
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
        let mut transaction = self.pool.begin().await?;

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
                .await?;

            count += result.rows_affected() as usize;
        }

        transaction.commit().await?;
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
        sqlx::query(&query).execute(&self.pool).await?;
        log::info!("Cleared SQLite table '{}'", self.table_name);
        Ok(())
    }
}