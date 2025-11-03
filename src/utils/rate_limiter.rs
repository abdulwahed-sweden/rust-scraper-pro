use std::time::{Duration, Instant};
use tokio::time::sleep;

pub struct RateLimiter {
    interval: Duration,
    last_request: Instant,
}

impl RateLimiter {
    pub fn new(interval_ms: u64) -> Self {
        Self {
            interval: Duration::from_millis(interval_ms),
            last_request: Instant::now() - Duration::from_millis(interval_ms),
        }
    }

    pub async fn wait(&mut self) {
        let elapsed = self.last_request.elapsed();
        if elapsed < self.interval {
            let wait_time = self.interval - elapsed;
            log::debug!("Rate limiting: waiting {}ms", wait_time.as_millis());
            sleep(wait_time).await;
        }
        self.last_request = Instant::now();
    }

    pub fn set_interval(&mut self, interval_ms: u64) {
        self.interval = Duration::from_millis(interval_ms);
    }

    pub fn get_interval(&self) -> Duration {
        self.interval
    }
}