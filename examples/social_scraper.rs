//! Realistic Social Media / JSON API Scraper Example
//!
//! Demonstrates scraping from Reddit's public JSON API.
//! Reddit provides free access to public posts via `.json` endpoints.
//!
//! This example shows:
//! - Fetching data from JSON APIs (no HTML parsing needed)
//! - Processing social media content (posts, comments, authors)
//! - Handling API-specific data structures
//! - Rate limiting for API endpoints
//!
//! Usage: cargo run --example social_scraper

use anyhow::Result;
use rust_scraper_pro::{
    core::models::ScrapedData,
    output::{
        json::JsonOutput,
        csv::CsvOutput,
    },
    utils::logger::setup_logger,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
struct RedditResponse {
    data: RedditData,
}

#[derive(Debug, Deserialize, Serialize)]
struct RedditData {
    children: Vec<RedditChild>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RedditChild {
    data: RedditPost,
}

#[derive(Debug, Deserialize, Serialize)]
struct RedditPost {
    title: String,
    author: String,
    selftext: Option<String>,
    url: String,
    score: i64,
    num_comments: i64,
    created_utc: f64,
    subreddit: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    setup_logger()?;

    println!("\n📱 Rust Scraper Pro - Reddit JSON API Example");
    println!("==============================================\n");

    // Configure Reddit JSON endpoint
    let subreddit = "worldnews";
    let url = format!("https://www.reddit.com/r/{}.json", subreddit);

    println!("✓ Target: Reddit (Public JSON API)");
    println!("  Subreddit: r/{}", subreddit);
    println!("  Endpoint: {}\n", url);

    // Create HTTP client with appropriate user agent
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (compatible; RustScraperPro/1.0; Educational)")
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    println!("⏳ Fetching posts from Reddit...");

    // Fetch data from Reddit API
    match client.get(&url).send().await {
        Ok(response) => {
            if !response.status().is_success() {
                eprintln!("❌ HTTP Error: {}", response.status());
                eprintln!("   Reddit may be rate limiting or the endpoint changed.");
                return Ok(());
            }

            let reddit_data: RedditResponse = response.json().await?;
            let posts = reddit_data.data.children;

            println!("✓ Fetched {} posts\n", posts.len());

            // Convert Reddit posts to ScrapedData format
            let mut scraped_items = Vec::new();

            for post in posts.iter() {
                let p = &post.data;

                let mut metadata = HashMap::new();
                metadata.insert("score".to_string(), p.score.to_string());
                metadata.insert("comments".to_string(), p.num_comments.to_string());
                metadata.insert("subreddit".to_string(), p.subreddit.clone());
                metadata.insert("created_utc".to_string(), p.created_utc.to_string());

                let item = ScrapedData {
                    id: uuid::Uuid::new_v4().to_string(),
                    source: format!("Reddit r/{}", subreddit),
                    url: p.url.clone(),
                    title: Some(p.title.clone()),
                    content: p.selftext.clone(),
                    price: None,
                    image_url: None,
                    author: Some(p.author.clone()),
                    timestamp: chrono::Utc::now(),
                    metadata,
                    category: Some(subreddit.to_string()),
                };

                scraped_items.push(item);
            }

            // Display sample posts
            println!("🔥 Top Posts from r/{}:", subreddit);
            println!("═══════════════════════════════════════════════════════════");
            for (idx, item) in scraped_items.iter().take(10).enumerate() {
                println!("\n[{}] {}", idx + 1, item.title.as_ref().unwrap_or(&"(no title)".to_string()));
                println!("    👤 u/{}", item.author.as_ref().unwrap_or(&"deleted".to_string()));

                if let Some(score) = item.metadata.get("score") {
                    println!("    ⬆️  Score: {}", score);
                }

                if let Some(comments) = item.metadata.get("comments") {
                    println!("    💬 Comments: {}", comments);
                }

                // Trim content for display
                if let Some(content) = &item.content {
                    if !content.is_empty() {
                        let preview = if content.len() > 100 {
                            format!("{}...", &content[..100])
                        } else {
                            content.clone()
                        };
                        println!("    📝 {}", preview);
                    }
                }
            }
            println!("\n═══════════════════════════════════════════════════════════\n");

            // Calculate statistics
            let total_score: i64 = scraped_items.iter()
                .filter_map(|item| item.metadata.get("score"))
                .filter_map(|s| s.parse::<i64>().ok())
                .sum();

            let total_comments: i64 = scraped_items.iter()
                .filter_map(|item| item.metadata.get("comments"))
                .filter_map(|s| s.parse::<i64>().ok())
                .sum();

            println!("📊 Statistics:");
            println!("   Total posts: {}", scraped_items.len());
            println!("   Total score: {} upvotes", total_score);
            println!("   Total comments: {}", total_comments);
            println!("   Avg score: {:.1}", total_score as f64 / scraped_items.len() as f64);
            println!("   Avg comments: {:.1}\n", total_comments as f64 / scraped_items.len() as f64);

            // Export to JSON
            println!("💾 Exporting data...");
            let json_output = JsonOutput::new();
            json_output.export(&scraped_items, "output/reddit_worldnews.json").await?;
            println!("✓ JSON: output/reddit_worldnews.json");

            // Export to CSV
            let csv_output = CsvOutput::new();
            csv_output.export(&scraped_items, "output/reddit_worldnews.csv").await?;
            println!("✓ CSV: output/reddit_worldnews.csv");

            println!("\n✅ Reddit scraping completed successfully!");
            println!("   {} posts from r/{} exported\n", scraped_items.len(), subreddit);
        }
        Err(e) => {
            eprintln!("❌ Failed to fetch data from Reddit: {}", e);
            eprintln!("   This could be due to:");
            eprintln!("   - Network connectivity issues");
            eprintln!("   - Reddit API rate limiting");
            eprintln!("   - Temporary Reddit outage");
            eprintln!("\n   Try again in a few minutes.");
        }
    }

    Ok(())
}
