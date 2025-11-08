//! AI-powered data normalization and refinement layer
//!
//! Uses DeepSeek to intelligently clean, standardize, and unify
//! data from multiple scraping sources before database insertion.

use super::deepseek_client::{DeepSeekClient, DeepSeekMessage};
use crate::core::models::ScrapedData;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedData {
    pub id: String,
    pub title: String,
    pub price_usd: Option<f64>,
    pub image: Option<String>,
    pub category: Option<String>,
    pub source: String,
    pub timestamp: String,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizationStats {
    pub total_input: usize,
    pub total_output: usize,
    pub duplicates_removed: usize,
    pub invalid_removed: usize,
    pub fields_standardized: usize,
}

pub struct DataNormalizer {
    client: DeepSeekClient,
    batch_size: usize,
}

impl DataNormalizer {
    pub fn new(client: DeepSeekClient) -> Self {
        Self {
            client,
            batch_size: 50, // Process in batches to avoid token limits
        }
    }

    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size;
        self
    }

    /// Normalize a batch of scraped data using AI
    pub async fn normalize_batch(&self, data: Vec<ScrapedData>) -> Result<Vec<NormalizedData>> {
        if data.is_empty() {
            return Ok(Vec::new());
        }

        log::info!("Normalizing {} items with DeepSeek AI...", data.len());

        // Convert to simple format for AI processing
        let simplified: Vec<SimplifiedItem> = data
            .iter()
            .map(|item| SimplifiedItem {
                id: item.id.clone(),
                source: item.source.clone(),
                title: item.title.clone(),
                price: item.price,
                image_url: item.image_url.clone(),
                category: item.category.clone(),
                timestamp: item.timestamp.to_rfc3339(),
            })
            .collect();

        let data_json = serde_json::to_string_pretty(&simplified)
            .context("Failed to serialize data for AI processing")?;

        let system_prompt = r#"You are a data normalization expert. Your task is to clean, standardize, and deduplicate e-commerce/scraping data.

Rules for normalization:
1. **Field names**: Use consistent schema: id, title, price_usd, image, category, source, timestamp
2. **Prices**: Convert all prices to numeric USD format (use approximate conversions if needed)
   - GBP to USD: multiply by 1.27
   - EUR to USD: multiply by 1.08
   - Remove currency symbols and text
3. **Deduplication**: Remove exact duplicates based on title + source
4. **Data quality**: Remove items with missing critical fields (no title)
5. **Standardization**: 
   - Trim whitespace
   - Normalize categories to title case
   - Ensure URLs are valid
6. **Output**: Return ONLY valid JSON array, no additional text

Expected output format:
[
  {
    "id": "uuid",
    "title": "Clean Title",
    "price_usd": 123.45,
    "image": "https://example.com/image.jpg",
    "category": "Category Name",
    "source": "Source Name",
    "timestamp": "ISO8601 timestamp",
    "metadata": {"key": "value"}
  }
]"#;

        let user_prompt = format!(
            "Normalize and refine this scraped data:\n\n{}",
            data_json
        );

        let messages = vec![
            DeepSeekMessage {
                role: "system".to_string(),
                content: system_prompt.to_string(),
            },
            DeepSeekMessage {
                role: "user".to_string(),
                content: user_prompt,
            },
        ];

        let response = self.client.completion(messages).await
            .context("Failed to get normalization response from DeepSeek")?;

        let content = response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from DeepSeek"))?;

        // Extract JSON from response (sometimes AI adds markdown code blocks)
        let json_content = extract_json(&content);

        let normalized: Vec<NormalizedData> = serde_json::from_str(&json_content)
            .context("Failed to parse DeepSeek normalization response")?;

        log::info!(
            "Normalization complete: {} → {} items",
            data.len(),
            normalized.len()
        );

        Ok(normalized)
    }

    /// Normalize all data with automatic batching
    pub async fn normalize_all(&self, data: Vec<ScrapedData>) -> Result<(Vec<NormalizedData>, NormalizationStats)> {
        let total_input = data.len();
        let mut all_normalized = Vec::new();

        log::info!("Starting normalization of {} items (batch size: {})", total_input, self.batch_size);

        // Process in batches
        for (i, chunk) in data.chunks(self.batch_size).enumerate() {
            log::info!("Processing batch {}/{}", i + 1, (total_input + self.batch_size - 1) / self.batch_size);
            
            let normalized = self.normalize_batch(chunk.to_vec()).await?;
            all_normalized.extend(normalized);

            // Small delay between batches to avoid rate limiting
            if i > 0 {
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            }
        }

        let stats = NormalizationStats {
            total_input,
            total_output: all_normalized.len(),
            duplicates_removed: total_input.saturating_sub(all_normalized.len()),
            invalid_removed: 0, // AI handles this internally
            fields_standardized: all_normalized.len() * 6, // rough estimate
        };

        log::info!("Normalization stats: {:?}", stats);

        Ok((all_normalized, stats))
    }

    /// Save normalized data to JSON file
    pub async fn save_to_json(&self, data: &[NormalizedData], path: &str) -> Result<()> {
        let json = serde_json::to_string_pretty(data)
            .context("Failed to serialize normalized data")?;

        tokio::fs::write(path, json)
            .await
            .context("Failed to write normalized data")?;

        log::info!("Saved {} normalized items to {}", data.len(), path);
        Ok(())
    }

    /// Quick normalization without AI (fallback mode)
    pub fn normalize_simple(data: Vec<ScrapedData>) -> Vec<NormalizedData> {
        log::info!("Using simple normalization (no AI)");

        data.into_iter()
            .filter_map(|item| {
                // Basic validation
                if item.title.is_none() {
                    return None;
                }

                Some(NormalizedData {
                    id: item.id,
                    title: item.title.unwrap_or_default(),
                    price_usd: item.price,
                    image: item.image_url,
                    category: item.category,
                    source: item.source,
                    timestamp: item.timestamp.to_rfc3339(),
                    metadata: item.metadata,
                })
            })
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SimplifiedItem {
    id: String,
    source: String,
    title: Option<String>,
    price: Option<f64>,
    image_url: Option<String>,
    category: Option<String>,
    timestamp: String,
}

/// Extract JSON from text that might contain markdown code blocks
fn extract_json(text: &str) -> String {
    let trimmed = text.trim();
    
    // Check for markdown code blocks
    if trimmed.starts_with("```json") {
        trimmed
            .strip_prefix("```json")
            .and_then(|s| s.strip_suffix("```"))
            .unwrap_or(trimmed)
            .trim()
            .to_string()
    } else if trimmed.starts_with("```") {
        trimmed
            .strip_prefix("```")
            .and_then(|s| s.strip_suffix("```"))
            .unwrap_or(trimmed)
            .trim()
            .to_string()
    } else {
        trimmed.to_string()
    }
}
