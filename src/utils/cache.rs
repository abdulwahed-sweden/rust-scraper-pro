use anyhow::Result;
use moka::sync::Cache;
use std::hash::Hash;
use std::time::Duration;
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;
use tokio::fs;

pub struct CacheSystem<K, V> {
    memory_cache: Option<Cache<K, V>>,
    cache_dir: Option<String>,
}

impl<K, V> CacheSystem<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub fn new_memory_cache(max_capacity: u64, ttl_seconds: u64) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(ttl_seconds))
            .build();

        Self {
            memory_cache: Some(cache),
            cache_dir: None,
        }
    }

    pub fn new_file_cache(cache_dir: &str) -> Self {
        Self {
            memory_cache: None,
            cache_dir: Some(cache_dir.to_string()),
        }
    }

    pub fn new_hybrid_cache(max_capacity: u64, ttl_seconds: u64, cache_dir: &str) -> Self {
        let cache = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(ttl_seconds))
            .build();

        Self {
            memory_cache: Some(cache),
            cache_dir: Some(cache_dir.to_string()),
        }
    }

    pub async fn get(&self, key: &K) -> Option<V>
    where
        K: ToString,
        V: DeserializeOwned,
    {
        // Try memory cache first
        if let Some(cache) = &self.memory_cache {
            if let Some(value) = cache.get(key) {
                log::debug!("Cache hit (memory) for key: {}", key.to_string());
                return Some(value);
            }
        }

        // Try file cache
        if let Some(_cache_dir) = &self.cache_dir {
            let file_path = self.get_file_path(key);
            if let Ok(content) = fs::read_to_string(&file_path).await {
                if let Ok(value) = serde_json::from_str::<V>(&content) {
                    log::debug!("Cache hit (file) for key: {}", key.to_string());
                    
                    // Note: Not repopulating memory cache to avoid type issues
                    
                    return Some(value);
                }
            }
        }

        None
    }

    pub async fn set(&self, key: K, value: V) -> Result<()>
    where
        K: ToString + Clone,
        V: Serialize + Clone,
    {
        // Set in memory cache
        if let Some(cache) = &self.memory_cache {
            cache.insert(key.clone(), value.clone());
        }

        // Set in file cache
        if let Some(_cache_dir) = &self.cache_dir {
            let file_path = self.get_file_path(&key);

            // Create directory if it doesn't exist
            if let Some(parent) = Path::new(&file_path).parent() {
                fs::create_dir_all(parent).await?;
            }

            let serialized = serde_json::to_string(&value)?;
            fs::write(&file_path, serialized).await?;
        }

        log::debug!("Cache set for key: {}", key.to_string());
        Ok(())
    }

    pub async fn remove(&self, key: &K) -> Result<()>
    where
        K: ToString,
    {
        // Remove from memory cache
        if let Some(cache) = &self.memory_cache {
            cache.invalidate(key);
        }

        // Remove from file cache
        if let Some(_cache_dir) = &self.cache_dir {
            let file_path = self.get_file_path(key);
            if Path::new(&file_path).exists() {
                fs::remove_file(&file_path).await?;
            }
        }

        log::debug!("Cache removed for key: {}", key.to_string());
        Ok(())
    }

    pub async fn clear(&self) -> Result<()> {
        // Clear memory cache
        if let Some(cache) = &self.memory_cache {
            cache.invalidate_all();
        }

        // Clear file cache
        if let Some(cache_dir) = &self.cache_dir {
            let path = Path::new(cache_dir);
            if path.exists() {
                fs::remove_dir_all(path).await?;
                fs::create_dir_all(path).await?;
            }
        }

        log::info!("Cache cleared");
        Ok(())
    }

    fn get_file_path(&self, key: &K) -> String 
    where
        K: ToString,
    {
        let key_str = key.to_string();
        let hashed_key = format!("{:x}", md5::compute(&key_str));
        
        if let Some(cache_dir) = &self.cache_dir {
            format!("{}/{}.json", cache_dir, hashed_key)
        } else {
            format!("cache/{}.json", hashed_key)
        }
    }

    pub fn stats(&self) -> CacheStats {
        if let Some(cache) = &self.memory_cache {
            CacheStats {
                entry_count: cache.entry_count(),
                hit_rate: 0.0, // Stats not available in sync cache
                miss_rate: 0.0,
            }
        } else {
            CacheStats::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub entry_count: u64,
    pub hit_rate: f64,
    pub miss_rate: f64,
}

impl Default for CacheStats {
    fn default() -> Self {
        Self {
            entry_count: 0,
            hit_rate: 0.0,
            miss_rate: 0.0,
        }
    }
}

// Specialized cache for HTML content
pub type HtmlCache = CacheSystem<String, String>;

impl HtmlCache {
    pub fn new_html_cache(max_capacity: u64, ttl_seconds: u64) -> Self {
        Self::new_memory_cache(max_capacity, ttl_seconds)
    }

    pub async fn get_html(&self, url: &str) -> Option<String> {
        self.get(&url.to_string()).await
    }

    pub async fn set_html(&self, url: &str, html: &str) -> Result<()> {
        self.set(url.to_string(), html.to_string()).await
    }

    pub async fn remove_html(&self, url: &str) -> Result<()> {
        self.remove(&url.to_string()).await
    }
}
