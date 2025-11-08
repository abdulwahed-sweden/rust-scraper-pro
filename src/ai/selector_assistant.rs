//! AI-powered CSS selector detection using DeepSeek
//!
//! Automatically analyzes HTML and recommends optimal CSS selectors
//! for extracting product data, articles, and other structured content.

use super::deepseek_client::{DeepSeekClient, DeepSeekMessage};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedSelectors {
    pub domain: String,
    pub title: Option<String>,
    pub price: Option<String>,
    pub image: Option<String>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub link: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, String>,
    pub confidence: f32,
    pub generated_at: String,
}

pub struct SelectorAssistant {
    client: DeepSeekClient,
    selectors_dir: PathBuf,
}

impl SelectorAssistant {
    pub fn new(client: DeepSeekClient) -> Self {
        Self {
            client,
            selectors_dir: PathBuf::from("selectors"),
        }
    }

    pub fn with_selectors_dir(client: DeepSeekClient, dir: PathBuf) -> Self {
        Self {
            client,
            selectors_dir: dir,
        }
    }

    /// Analyze HTML and detect optimal selectors using DeepSeek
    pub async fn detect_selectors(&self, domain: &str, html_sample: &str) -> Result<DetectedSelectors> {
        log::info!("Analyzing HTML from {} with DeepSeek AI...", domain);

        // Truncate HTML if too large (to avoid token limits)
        let truncated_html = if html_sample.len() > 8000 {
            &html_sample[..8000]
        } else {
            html_sample
        };

        let system_prompt = r#"You are an expert web scraping assistant. Your task is to analyze HTML and suggest optimal CSS selectors for extracting structured data.

Rules:
1. Provide selectors in standard CSS format
2. Prefer class names and data attributes over complex paths
3. Focus on selectors that are stable and unlikely to change
4. Return ONLY valid JSON, no additional text
5. If a field is not found, use null

Return JSON in this exact format:
{
  "title": "CSS selector for title",
  "price": "CSS selector for price",
  "image": "CSS selector for image (src attribute)",
  "category": "CSS selector for category",
  "description": "CSS selector for description/content",
  "author": "CSS selector for author",
  "date": "CSS selector for date",
  "link": "CSS selector for link",
  "confidence": 0.85
}"#;

        let user_prompt = format!(
            "Domain: {}\n\nAnalyze this HTML and extract optimal selectors for e-commerce products or articles:\n\n{}",
            domain, truncated_html
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
            .context("Failed to get selector recommendations from DeepSeek")?;

        let content = response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from DeepSeek"))?;

        // Parse JSON response
        let selector_data: SelectorData = serde_json::from_str(&content)
            .context("Failed to parse DeepSeek selector response as JSON")?;

        let detected = DetectedSelectors {
            domain: domain.to_string(),
            title: selector_data.title,
            price: selector_data.price,
            image: selector_data.image,
            category: selector_data.category,
            description: selector_data.description,
            author: selector_data.author,
            date: selector_data.date,
            link: selector_data.link,
            metadata: HashMap::new(),
            confidence: selector_data.confidence,
            generated_at: chrono::Utc::now().to_rfc3339(),
        };

        log::info!(
            "Detected selectors for {} (confidence: {:.0}%)",
            domain,
            detected.confidence * 100.0
        );

        Ok(detected)
    }

    /// Save detected selectors to file
    pub async fn save_selectors(&self, selectors: &DetectedSelectors) -> Result<PathBuf> {
        // Create selectors directory if it doesn't exist
        tokio::fs::create_dir_all(&self.selectors_dir)
            .await
            .context("Failed to create selectors directory")?;

        let filename = format!("{}.selectors.json", sanitize_filename(&selectors.domain));
        let file_path = self.selectors_dir.join(&filename);

        let json = serde_json::to_string_pretty(selectors)
            .context("Failed to serialize selectors")?;

        tokio::fs::write(&file_path, json)
            .await
            .context("Failed to write selectors file")?;

        log::info!("Saved selectors to: {}", file_path.display());
        Ok(file_path)
    }

    /// Load selectors from file
    pub async fn load_selectors(&self, domain: &str) -> Result<DetectedSelectors> {
        let filename = format!("{}.selectors.json", sanitize_filename(domain));
        let file_path = self.selectors_dir.join(&filename);

        let content = tokio::fs::read_to_string(&file_path)
            .await
            .context("Failed to read selectors file")?;

        let selectors: DetectedSelectors = serde_json::from_str(&content)
            .context("Failed to parse selectors file")?;

        log::info!("Loaded selectors for {} from file", domain);
        Ok(selectors)
    }

    /// Check if selectors exist for a domain
    pub async fn has_selectors(&self, domain: &str) -> bool {
        let filename = format!("{}.selectors.json", sanitize_filename(domain));
        let file_path = self.selectors_dir.join(&filename);
        file_path.exists()
    }

    /// Get or detect selectors (load from cache or generate new)
    pub async fn get_or_detect_selectors(
        &self,
        domain: &str,
        html_sample: &str,
    ) -> Result<DetectedSelectors> {
        if self.has_selectors(domain).await {
            log::info!("Using cached selectors for {}", domain);
            self.load_selectors(domain).await
        } else {
            log::info!("No cached selectors found, detecting new ones for {}", domain);
            let selectors = self.detect_selectors(domain, html_sample).await?;
            self.save_selectors(&selectors).await?;
            Ok(selectors)
        }
    }
}

#[derive(Debug, Deserialize)]
struct SelectorData {
    title: Option<String>,
    price: Option<String>,
    image: Option<String>,
    category: Option<String>,
    description: Option<String>,
    author: Option<String>,
    date: Option<String>,
    link: Option<String>,
    confidence: f32,
}

/// Sanitize filename for cross-platform compatibility
fn sanitize_filename(name: &str) -> String {
    name.replace("://", "_")
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "_")
        .replace("*", "_")
        .replace("?", "_")
        .replace("\"", "_")
        .replace("<", "_")
        .replace(">", "_")
        .replace("|", "_")
}
