use crate::core::models::ScrapedData;
use anyhow::Result;
use csv::Writer;
use std::fs::File;
use std::path::Path;

pub struct CsvOutput;

impl CsvOutput {
    pub fn new() -> Self {
        Self
    }

    pub async fn export<P: AsRef<Path>>(&self, data: &[ScrapedData], path: P) -> Result<()> {
        // Create directory if it doesn't exist
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(path)?;
        let mut wtr = Writer::from_writer(file);

        // Write header
        wtr.write_record(&[
            "id", "source", "url", "title", "content", "price", "image_url", 
            "author", "timestamp", "category"
        ])?;

        for item in data {
            wtr.write_record(&[
                &item.id,
                &item.source,
                &item.url,
                item.title.as_deref().unwrap_or(""),
                item.content.as_deref().unwrap_or(""),
                &item.price.map(|p| p.to_string()).unwrap_or_default(),
                item.image_url.as_deref().unwrap_or(""),
                item.author.as_deref().unwrap_or(""),
                &item.timestamp.to_rfc3339(),
                item.category.as_deref().unwrap_or(""),
            ])?;
        }

        wtr.flush()?;
        log::info!("Exported {} items to CSV", data.len());
        Ok(())
    }

    pub async fn export_with_metadata<P: AsRef<Path>>(&self, data: &[ScrapedData], path: P) -> Result<()> {
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }

        let file = File::create(path)?;
        let mut wtr = Writer::from_writer(file);

        // Extended header with metadata
        wtr.write_record(&[
            "id", "source", "url", "title", "content", "price", "image_url", 
            "author", "timestamp", "category", "metadata"
        ])?;

        for item in data {
            let metadata_json = serde_json::to_string(&item.metadata).unwrap_or_default();
            
            wtr.write_record(&[
                &item.id,
                &item.source,
                &item.url,
                item.title.as_deref().unwrap_or(""),
                item.content.as_deref().unwrap_or(""),
                &item.price.map(|p| p.to_string()).unwrap_or_default(),
                item.image_url.as_deref().unwrap_or(""),
                item.author.as_deref().unwrap_or(""),
                &item.timestamp.to_rfc3339(),
                item.category.as_deref().unwrap_or(""),
                &metadata_json,
            ])?;
        }

        wtr.flush()?;
        log::info!("Exported {} items to CSV with metadata", data.len());
        Ok(())
    }
}