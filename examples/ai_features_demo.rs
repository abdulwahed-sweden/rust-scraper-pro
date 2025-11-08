//! Demonstration of AI-powered features
//!
//! This example shows:
//! 1. DeepSeek API connection testing
//! 2. Adaptive delay controller
//! 3. Selector assistant (if enabled)
//! 4. Data normalizer (if enabled)
//!
//! Run with: cargo run --example ai_features_demo

use anyhow::Result;
use rust_scraper_pro::ai::{
    AdaptiveDelayConfig,
    AdaptiveDelayController,
    DeepSeekClient,
    DelayMode,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::init();

    println!("\n🚀 rust-scraper-pro AI Features Demo\n");
    println!("═══════════════════════════════════════\n");

    // Demo 1: Test DeepSeek API Connection
    demo_deepseek_connection().await?;

    // Demo 2: Adaptive Delay Controller
    demo_adaptive_delay().await?;

    // Demo 3: Speed Presets
    demo_speed_presets().await?;

    println!("\n✅ All demos completed successfully!");
    println!("═══════════════════════════════════════\n");

    Ok(())
}

async fn demo_deepseek_connection() -> Result<()> {
    println!("📡 Demo 1: DeepSeek API Connection Test");
    println!("───────────────────────────────────────");

    match DeepSeekClient::new() {
        Ok(client) => {
            println!("✅ DeepSeek client created successfully");
            
            match client.test_connection().await {
                Ok(_) => println!("✅ API connection test passed!"),
                Err(e) => println!("⚠️  API connection test failed: {}", e),
            }
        }
        Err(e) => {
            println!("⚠️  DeepSeek client creation failed: {}", e);
            println!("   Make sure DEEPSEEK_API_KEY is set in .env file");
        }
    }

    println!();
    Ok(())
}

async fn demo_adaptive_delay() -> Result<()> {
    println!("⏱️  Demo 2: Adaptive Delay Controller");
    println!("───────────────────────────────────────");

    // Create adaptive delay controller
    let config = AdaptiveDelayConfig {
        mode: DelayMode::Adaptive,
        min_delay_ms: 200,
        max_delay_ms: 2500,
        sample_size: 10,
        multiplier: 1.2,
    };

    let controller = AdaptiveDelayController::new(config);
    println!("✅ Adaptive delay controller created");
    println!("   Mode: Adaptive");
    println!("   Range: 200ms - 2500ms");
    println!("   Multiplier: 1.2 (20% slower than average)");

    // Simulate some response times
    println!("\n📊 Simulating 10 server responses...");
    let simulated_responses = vec![
        Duration::from_millis(150),
        Duration::from_millis(175),
        Duration::from_millis(160),
        Duration::from_millis(200),
        Duration::from_millis(145),
        Duration::from_millis(190),
        Duration::from_millis(165),
        Duration::from_millis(180),
        Duration::from_millis(155),
        Duration::from_millis(170),
    ];

    for (i, response_time) in simulated_responses.iter().enumerate() {
        controller.record_response_time(*response_time);
        
        let delay = controller.calculate_delay();
        println!("   Response {}: {:?} → Next delay: {:?}", 
            i + 1, response_time, delay);
    }

    // Get statistics
    let stats = controller.get_stats();
    println!("\n📈 Final Statistics:");
    println!("   Samples collected:  {}", stats.samples);
    println!("   Average response:   {:?}", stats.avg_response_time);
    println!("   Min response:       {:?}", stats.min_response_time);
    println!("   Max response:       {:?}", stats.max_response_time);
    println!("   Current delay:      {:?}", stats.current_delay);

    println!();
    Ok(())
}

async fn demo_speed_presets() -> Result<()> {
    println!("🎛️  Demo 3: Speed Presets");
    println!("───────────────────────────────────────");

    let presets = vec![
        ("Slow (Conservative)", AdaptiveDelayConfig {
            mode: DelayMode::Adaptive,
            min_delay_ms: 1000,
            max_delay_ms: 5000,
            sample_size: 10,
            multiplier: 1.5,
        }),
        ("Medium (Balanced)", AdaptiveDelayConfig {
            mode: DelayMode::Adaptive,
            min_delay_ms: 500,
            max_delay_ms: 3000,
            sample_size: 10,
            multiplier: 1.2,
        }),
        ("Fast (Aggressive)", AdaptiveDelayConfig {
            mode: DelayMode::Adaptive,
            min_delay_ms: 200,
            max_delay_ms: 1500,
            sample_size: 10,
            multiplier: 1.1,
        }),
        ("Fixed Rate", AdaptiveDelayConfig {
            mode: DelayMode::Fixed,
            min_delay_ms: 2000,
            max_delay_ms: 2000,
            sample_size: 1,
            multiplier: 1.0,
        }),
    ];

    for (name, config) in presets {
        let controller = AdaptiveDelayController::new(config.clone());
        
        // Record a sample response time
        controller.record_response_time(Duration::from_millis(150));
        
        let delay = controller.calculate_delay();
        
        println!("   {} → {:?}", name, delay);
    }

    println!("\n💡 Tip: Configure in config/settings.toml under [scraper]");
    println!();
    Ok(())
}
