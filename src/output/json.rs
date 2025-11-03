use crate::core::models::ScrapedData;
use anyhow::Result;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct JsonOutput;

impl JsonOutput {
    pub fn new() -> Self {
        Self
    }

    pub async fn export<P: AsRef<Path>>(&self, data: &[ScrapedData], path: P) -> Result<()> {
        let json = serde_json::to_string_pretty(data)?;
        
        // Create directory if it doesn't exist
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        
        log::info!("Exported {} items to JSON", data.len());
        Ok(())
    }

    pub async fn export_minified<P: AsRef<Path>>(&self, data: &[ScrapedData], path: P) -> Result<()> {
        let json = serde_json::to_string(data)?;
        
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        
        log::info!("Exported {} items to minified JSON", data.len());
        Ok(())
    }
}