# Adaptive Scraping Mode

**Status:** ✅ Production Ready  
**Feature:** Dynamic delay adjustment based on server response times  
**Purpose:** Optimize scraping speed while respecting server load

---

## Overview

Traditional web scrapers use **fixed delays** between requests (e.g., always wait 2 seconds). This approach is either:
- **Too slow** (wastes time when server responds quickly)
- **Too aggressive** (risks being blocked when server is under load)

**Adaptive Scraping** solves this by automatically adjusting delays based on actual server response times.

---

## How It Works

### Traditional Fixed Delay
```
Request 1 → [2s delay] → Request 2 → [2s delay] → Request 3
  (50ms)                    (50ms)                    (50ms)
```
**Total time:** 2050ms per request (unnecessarily slow!)

### Adaptive Delay
```
Request 1 → [60ms adaptive delay] → Request 2 → [72ms] → Request 3
  (50ms)                              (60ms)               (70ms)
```
**Total time:** ~110-142ms per request (much faster!)

### The Algorithm

```rust
// 1. Track response times
let response_times = [45ms, 50ms, 48ms, 52ms, 49ms];

// 2. Calculate average
let avg_response_time = 48.8ms;

// 3. Apply safety multiplier (20% slower than average)
let adaptive_delay = avg_response_time * 1.2;  // = 58.56ms

// 4. Clamp to min/max bounds
let final_delay = adaptive_delay.clamp(200ms, 2500ms);  // = 200ms (hit minimum)

// 5. Wait before next request
tokio::time::sleep(final_delay).await;
```

---

## Configuration

**File:** `config/settings.toml`

```toml
[scraper]
mode = "adaptive"        # Options: "fixed" or "adaptive"
min_delay_ms = 200       # Minimum delay (safety floor)
max_delay_ms = 2500      # Maximum delay (politeness ceiling)
sample_size = 10         # Number of response times to track
multiplier = 1.2         # Delay multiplier (1.2 = 20% slower)
```

### Parameters Explained

| Parameter | Description | Recommended Value |
|-----------|-------------|-------------------|
| `mode` | Scraping mode | `"adaptive"` for intelligent delay, `"fixed"` for constant rate |
| `min_delay_ms` | Minimum delay in milliseconds | `200` (never faster than 200ms) |
| `max_delay_ms` | Maximum delay in milliseconds | `2500` (never slower than 2.5s) |
| `sample_size` | How many response times to track | `10` (good balance of responsiveness and stability) |
| `multiplier` | Safety margin multiplier | `1.2` (20% slower than server responses) |

### Speed Presets

#### Slow (Conservative)
```toml
[scraper]
mode = "adaptive"
min_delay_ms = 1000
max_delay_ms = 5000
multiplier = 1.5  # 50% slower than average
```

#### Medium (Balanced)
```toml
[scraper]
mode = "adaptive"
min_delay_ms = 500
max_delay_ms = 3000
multiplier = 1.2  # 20% slower (default)
```

#### Fast (Aggressive)
```toml
[scraper]
mode = "adaptive"
min_delay_ms = 200
max_delay_ms = 1500
multiplier = 1.1  # 10% slower
```

#### Fixed Rate (Traditional)
```toml
[scraper]
mode = "fixed"
min_delay_ms = 2000  # Always 2 seconds
max_delay_ms = 2000
```

---

## Usage

### Basic Usage

```rust
use rust_scraper_pro::ai::{
    AdaptiveDelayController,
    AdaptiveDelayConfig,
    DelayMode,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Create adaptive delay controller
    let config = AdaptiveDelayConfig {
        mode: DelayMode::Adaptive,
        min_delay_ms: 200,
        max_delay_ms: 2500,
        sample_size: 10,
        multiplier: 1.2,
    };
    
    let delay_controller = AdaptiveDelayController::new(config);

    // In your scraping loop
    for url in urls {
        // Wait with adaptive delay
        delay_controller.wait().await;
        
        // Fetch data
        let start = std::time::Instant::now();
        let data = fetch_url(url).await?;
        let elapsed = start.elapsed();
        
        // Record response time for future calculations
        delay_controller.record_response_time(elapsed);
    }

    Ok(())
}
```

### With Automatic Timing

```rust
use rust_scraper_pro::ai::AdaptiveDelayController;

let delay_controller = AdaptiveDelayController::new(config);

for url in urls {
    // Wait before request
    delay_controller.wait().await;
    
    // Execute request with automatic timing
    let data = delay_controller.execute_with_timing(|| async {
        fetch_url(url).await
    }).await?;
    
    // Response time is automatically recorded!
}
```

### Get Statistics

```rust
let stats = delay_controller.get_stats();

println!("Adaptive Delay Statistics:");
println!("  Samples collected: {}", stats.samples);
println!("  Average response:  {:?}", stats.avg_response_time);
println!("  Min response:      {:?}", stats.min_response_time);
println!("  Max response:      {:?}", stats.max_response_time);
println!("  Current delay:     {:?}", stats.current_delay);
```

**Example Output:**
```
Adaptive Delay Statistics:
  Samples collected: 10
  Average response:  167ms
  Min response:      142ms
  Max response:      203ms
  Current delay:     200ms  (clamped to minimum)
```

---

## Integration with Scraper Engine

The adaptive delay is integrated into the main `ScraperEngine`. Here's how it works:

```rust
use rust_scraper_pro::{
    core::{config::Config, scraper::ScraperEngine},
    processors::pipeline::ProcessingPipeline,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load config (includes [scraper] section)
    let config = Config::load("config/settings.toml").await?;
    
    // ScraperEngine automatically uses adaptive delay from config
    let pipeline = ProcessingPipeline::new();
    let mut engine = ScraperEngine::new(config, pipeline, None);

    // Scrape with adaptive delays
    let data = engine.scrape_source(my_source).await?;
    
    Ok(())
}
```

The engine:
1. Reads adaptive delay config from `settings.toml`
2. Creates an `AdaptiveDelayController` automatically
3. Records all response times during scraping
4. Adjusts delays dynamically based on server performance

---

## Real-World Example

### Scenario: Scraping Books to Scrape

```bash
# Initial scrape (cold start)
Request 1: 350ms → Delay: 200ms (min_delay)
Request 2: 280ms → Delay: 200ms (min_delay)
Request 3: 190ms → Delay: 200ms (min_delay)
Request 4: 165ms → Delay: 200ms (min_delay)
Request 5: 158ms → Delay: 200ms (min_delay)

# After 10 samples (adaptive kicks in)
Avg response: 165ms
Adaptive delay: 165ms * 1.2 = 198ms → Clamped to 200ms (min)

# Server slows down
Request 15: 450ms
Request 16: 520ms
Request 17: 610ms

# New adaptive delay
Avg response: 480ms
Adaptive delay: 480ms * 1.2 = 576ms ✅ (within bounds)

# Scraper automatically slows down to match server load!
```

---

## Performance Comparison

### Fixed Delay (2 seconds)
```
100 requests * 2000ms = 200,000ms = 3.33 minutes
```

### Adaptive Delay
```
100 requests * 200ms  = 20,000ms  = 0.33 minutes
```

**Speed improvement:** 10x faster on responsive servers!

---

## Best Practices

### 1. Start Conservative
Begin with higher `min_delay_ms` and `multiplier`, then optimize:

```toml
# Initial conservative settings
min_delay_ms = 500
max_delay_ms = 3000
multiplier = 1.5
```

Monitor logs for any rate-limiting errors, then decrease if safe.

### 2. Monitor Response Times
Enable debug logging to see adaptive calculations:

```bash
export RUST_LOG=debug
cargo run
```

**Look for:**
```
[DEBUG] Recorded response time: 165ms (samples: 8)
[DEBUG] Adaptive delay calculated: 200ms (avg response: 167ms, multiplier: 1.2)
```

### 3. Respect Robots.txt
Always set `follow_robots_txt = true` in config!

### 4. Use Larger Sample Size for Stable Sites
For predictable servers:
```toml
sample_size = 20  # More stable, less reactive
```

For variable servers (news sites, APIs):
```toml
sample_size = 5   # More reactive, faster adaptation
```

### 5. Set Reasonable Bounds
- **Min delay:** Never below 100ms (too aggressive)
- **Max delay:** Match site's typical timeout (usually 2-5 seconds)

---

## Logging

Enable adaptive delay logging:

```bash
# Full debug output
RUST_LOG=rust_scraper_pro::ai::adaptive_delay=debug cargo run

# Info level
RUST_LOG=info cargo run
```

**Sample Logs:**
```
[INFO ] Adaptive delay calculated: 234ms (avg response: 195ms, multiplier: 1.2)
[INFO ] Recorded response time: 187ms (samples: 10)
[DEBUG] Response times: [165ms, 178ms, 192ms, 203ms, 187ms, 169ms, 174ms, 195ms, 182ms, 187ms]
```

---

## Troubleshooting

### Issue: Delay stuck at minimum
**Cause:** Server responds very quickly  
**Solution:** This is good! The server can handle faster requests.

### Issue: Delay stuck at maximum
**Cause:** Server is slow or under heavy load  
**Solution:** 
- Check if site is down
- Increase `max_delay_ms`
- Scrape during off-peak hours

### Issue: Erratic delay changes
**Cause:** Server response times are highly variable  
**Solution:** Increase `sample_size` to 15-20 for smoother averaging

### Issue: "Too many requests" errors
**Cause:** Multiplier is too aggressive  
**Solution:** Increase `multiplier` to 1.5 or 2.0

---

## Advantages Over Fixed Delay

| Feature | Fixed Delay | Adaptive Delay |
|---------|-------------|----------------|
| **Speed** | Always same (slow) | Optimized for server |
| **Adaptability** | None | Responds to load |
| **Efficiency** | Wastes time | Maximizes throughput |
| **Safety** | May be too fast or slow | Self-regulating |
| **Server Respect** | One-size-fits-all | Matches server capacity |

---

## Future Enhancements

- [ ] Per-domain adaptive profiles
- [ ] Automatic fallback to fixed mode on errors
- [ ] Adaptive burst mode (faster initial requests)
- [ ] Integration with server `Retry-After` headers
- [ ] Machine learning for long-term optimization

---

## Conclusion

Adaptive Scraping Mode makes rust-scraper-pro:

- ✅ **Faster** - Up to 10x speed improvement on responsive servers
- ✅ **Smarter** - Automatically adjusts to server conditions
- ✅ **Safer** - Self-regulating to avoid rate limits
- ✅ **Respectful** - Matches server capacity dynamically

**Ready to scrape intelligently? Configure adaptive mode in `config/settings.toml` today!**

---

**Generated:** November 8, 2025  
**Version:** 1.0.0  
**Algorithm:** Response Time Moving Average with Configurable Multiplier
