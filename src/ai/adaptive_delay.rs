//! Adaptive delay controller for intelligent rate limiting
//!
//! This module implements dynamic delay adjustment based on response times
//! to optimize scraping speed while respecting server load.

use parking_lot::RwLock;
use std::collections::VecDeque;
use std::sync::Arc;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelayMode {
    Fixed,
    Adaptive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdaptiveDelayConfig {
    pub mode: DelayMode,
    pub min_delay_ms: u64,
    pub max_delay_ms: u64,
    pub sample_size: usize,
    pub multiplier: f64,
}

impl Default for AdaptiveDelayConfig {
    fn default() -> Self {
        Self {
            mode: DelayMode::Adaptive,
            min_delay_ms: 200,
            max_delay_ms: 2500,
            sample_size: 10,
            multiplier: 1.2, // 20% slower than average
        }
    }
}

pub struct AdaptiveDelayController {
    config: AdaptiveDelayConfig,
    response_times: Arc<RwLock<VecDeque<Duration>>>,
}

impl AdaptiveDelayController {
    pub fn new(config: AdaptiveDelayConfig) -> Self {
        let sample_size = config.sample_size;
        Self {
            config,
            response_times: Arc::new(RwLock::new(VecDeque::with_capacity(sample_size))),
        }
    }

    /// Record a response time for adaptive calculation
    pub fn record_response_time(&self, duration: Duration) {
        let mut times = self.response_times.write();
        
        if times.len() >= self.config.sample_size {
            times.pop_front();
        }
        
        times.push_back(duration);
        
        log::debug!(
            "Recorded response time: {:?} (samples: {})",
            duration,
            times.len()
        );
    }

    /// Calculate the adaptive delay based on recent response times
    pub fn calculate_delay(&self) -> Duration {
        match self.config.mode {
            DelayMode::Fixed => Duration::from_millis(self.config.min_delay_ms),
            DelayMode::Adaptive => {
                let times = self.response_times.read();
                
                if times.is_empty() {
                    return Duration::from_millis(self.config.min_delay_ms);
                }

                // Calculate average response time
                let sum: Duration = times.iter().sum();
                let avg_ms = sum.as_millis() as f64 / times.len() as f64;
                
                // Apply multiplier (e.g., 1.2 = 20% slower)
                let adaptive_delay_ms = (avg_ms * self.config.multiplier) as u64;
                
                // Clamp to min/max bounds
                let clamped_delay = adaptive_delay_ms
                    .max(self.config.min_delay_ms)
                    .min(self.config.max_delay_ms);
                
                log::debug!(
                    "Adaptive delay calculated: {} ms (avg response: {:.0} ms, multiplier: {})",
                    clamped_delay,
                    avg_ms,
                    self.config.multiplier
                );
                
                Duration::from_millis(clamped_delay)
            }
        }
    }

    /// Wait for the calculated adaptive delay
    pub async fn wait(&self) -> Duration {
        let delay = self.calculate_delay();
        tokio::time::sleep(delay).await;
        delay
    }

    /// Execute a request and automatically record its timing
    pub async fn execute_with_timing<F, Fut, T>(&self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = anyhow::Result<T>>,
    {
        let start = Instant::now();
        let result = f().await;
        let duration = start.elapsed();
        
        // Only record successful requests
        if result.is_ok() {
            self.record_response_time(duration);
        }
        
        result
    }

    /// Get current statistics
    pub fn get_stats(&self) -> AdaptiveDelayStats {
        let times = self.response_times.read();
        
        if times.is_empty() {
            return AdaptiveDelayStats::default();
        }

        let sum: Duration = times.iter().sum();
        let avg = sum / times.len() as u32;
        
        let min = times.iter().min().copied().unwrap_or_default();
        let max = times.iter().max().copied().unwrap_or_default();
        
        AdaptiveDelayStats {
            samples: times.len(),
            avg_response_time: avg,
            min_response_time: min,
            max_response_time: max,
            current_delay: self.calculate_delay(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AdaptiveDelayStats {
    pub samples: usize,
    pub avg_response_time: Duration,
    pub min_response_time: Duration,
    pub max_response_time: Duration,
    pub current_delay: Duration,
}
