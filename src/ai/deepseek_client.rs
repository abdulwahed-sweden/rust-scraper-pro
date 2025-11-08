//! DeepSeek API client for AI-powered features
//!
//! This module provides integration with DeepSeek's API for:
//! - CSS selector detection
//! - Data normalization
//! - Intelligent text processing

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEEPSEEK_API_BASE: &str = "https://api.deepseek.com/v1";
const DEFAULT_MODEL: &str = "deepseek-chat";
const DEFAULT_TIMEOUT: u64 = 60;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepSeekRequest {
    pub model: String,
    pub messages: Vec<DeepSeekMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeepSeekChoice {
    pub message: DeepSeekMessage,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeepSeekUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeepSeekResponse {
    pub id: String,
    pub choices: Vec<DeepSeekChoice>,
    pub usage: DeepSeekUsage,
}

pub struct DeepSeekClient {
    client: reqwest::Client,
    api_key: String,
    base_url: String,
    model: String,
}

impl DeepSeekClient {
    /// Create a new DeepSeek client with API key from environment
    pub fn new() -> Result<Self> {
        let api_key = std::env::var("DEEPSEEK_API_KEY")
            .context("DEEPSEEK_API_KEY environment variable not set")?;

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            api_key,
            base_url: DEEPSEEK_API_BASE.to_string(),
            model: DEFAULT_MODEL.to_string(),
        })
    }

    /// Create a client with custom configuration
    pub fn with_config(api_key: String, model: Option<String>) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            api_key,
            base_url: DEEPSEEK_API_BASE.to_string(),
            model: model.unwrap_or_else(|| DEFAULT_MODEL.to_string()),
        })
    }

    /// Send a completion request to DeepSeek API
    pub async fn completion(&self, messages: Vec<DeepSeekMessage>) -> Result<DeepSeekResponse> {
        let request = DeepSeekRequest {
            model: self.model.clone(),
            messages,
            temperature: Some(0.7),
            max_tokens: Some(4000),
        };

        log::debug!("Sending request to DeepSeek API...");

        let response = self
            .client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .context("Failed to send request to DeepSeek API")?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            anyhow::bail!("DeepSeek API error ({}): {}", status, error_text);
        }

        let deepseek_response: DeepSeekResponse = response
            .json()
            .await
            .context("Failed to parse DeepSeek API response")?;

        log::info!(
            "DeepSeek API response received (tokens: prompt={}, completion={}, total={})",
            deepseek_response.usage.prompt_tokens,
            deepseek_response.usage.completion_tokens,
            deepseek_response.usage.total_tokens
        );

        Ok(deepseek_response)
    }

    /// Send a simple prompt and get text response
    pub async fn ask(&self, prompt: &str) -> Result<String> {
        let messages = vec![DeepSeekMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }];

        let response = self.completion(messages).await?;

        response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from DeepSeek API"))
    }

    /// Ask DeepSeek with system instructions
    pub async fn ask_with_system(&self, system: &str, user: &str) -> Result<String> {
        let messages = vec![
            DeepSeekMessage {
                role: "system".to_string(),
                content: system.to_string(),
            },
            DeepSeekMessage {
                role: "user".to_string(),
                content: user.to_string(),
            },
        ];

        let response = self.completion(messages).await?;

        response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| anyhow::anyhow!("No response from DeepSeek API"))
    }

    /// Test API connection
    pub async fn test_connection(&self) -> Result<()> {
        log::info!("Testing DeepSeek API connection...");
        
        let response = self.ask("Respond with 'OK' if you receive this message.").await?;
        
        log::info!("DeepSeek API connection test successful: {}", response.trim());
        Ok(())
    }
}

impl Default for DeepSeekClient {
    fn default() -> Self {
        Self::new().expect("Failed to create DeepSeek client")
    }
}
