//! AI-powered features for intelligent scraping
//! 
//! This module provides AI capabilities including:
//! - Adaptive delay management
//! - DeepSeek API integration
//! - Automatic selector detection
//! - Data normalization and refinement

pub mod adaptive_delay;
pub mod deepseek_client;
pub mod selector_assistant;
pub mod normalizer;

pub use adaptive_delay::{AdaptiveDelayController, AdaptiveDelayConfig, DelayMode, AdaptiveDelayStats};
pub use deepseek_client::{DeepSeekClient, DeepSeekMessage, DeepSeekRequest, DeepSeekResponse};
pub use selector_assistant::{SelectorAssistant, DetectedSelectors};
pub use normalizer::{DataNormalizer, NormalizedData, NormalizationStats};
