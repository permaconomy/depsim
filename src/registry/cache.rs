//! In-memory cache for packument metadata

use crate::resolver::types::Packument;
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Cache entry with TTL
#[derive(Clone)]
struct CacheEntry {
    packument: Arc<Packument>,
    inserted_at: Instant,
}

/// Thread-safe in-memory cache for packuments
pub struct PackumentCache {
    /// Cache storage
    cache: DashMap<String, CacheEntry>,
    /// Time-to-live for cache entries
    ttl: Duration,
    /// Maximum cache size (number of entries)
    max_size: usize,
}

impl PackumentCache {
    /// Create a new cache with default settings (5 minute TTL, 1000 max entries)
    pub fn new() -> Self {
        Self::with_config(Duration::from_secs(300), 1000)
    }

    /// Create a new cache with custom TTL and max size
    pub fn with_config(ttl: Duration, max_size: usize) -> Self {
        Self {
            cache: DashMap::new(),
            ttl,
            max_size,
        }
    }

    /// Get a packument from cache if it exists and is not expired
    pub fn get(&self, package_name: &str) -> Option<Arc<Packument>> {
        if let Some(entry) = self.cache.get(package_name) {
            if entry.inserted_at.elapsed() < self.ttl {
                return Some(Arc::clone(&entry.packument));
            } else {
                // Entry expired, remove it
                drop(entry);
                self.cache.remove(package_name);
            }
        }
        None
    }

    /// Insert a packument into the cache
    pub fn insert(&self, package_name: String, packument: Packument) {
        // Check if we need to evict entries
        if self.cache.len() >= self.max_size {
            self.evict_oldest();
        }

        let entry = CacheEntry {
            packument: Arc::new(packument),
            inserted_at: Instant::now(),
        };

        self.cache.insert(package_name, entry);
    }

    /// Evict the oldest entry (simple FIFO eviction)
    fn evict_oldest(&self) {
        // Find the oldest entry
        let mut oldest_key: Option<String> = None;
        let mut oldest_time = Instant::now();

        for entry in self.cache.iter() {
            if entry.value().inserted_at < oldest_time {
                oldest_time = entry.value().inserted_at;
                oldest_key = Some(entry.key().clone());
            }
        }

        // Remove the oldest entry
        if let Some(key) = oldest_key {
            self.cache.remove(&key);
        }
    }

    /// Clear all entries from the cache
    pub fn clear(&self) {
        self.cache.clear();
    }

    /// Get the number of entries in the cache
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Remove expired entries
    pub fn cleanup_expired(&self) {
        let now = Instant::now();
        self.cache.retain(|_, entry| now.duration_since(entry.inserted_at) < self.ttl);
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        CacheStats {
            size: self.cache.len(),
            max_size: self.max_size,
            ttl_seconds: self.ttl.as_secs(),
        }
    }
}

impl Default for PackumentCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Current number of entries
    pub size: usize,
    /// Maximum number of entries
    pub max_size: usize,
    /// TTL in seconds
    pub ttl_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::types::PackageMetadata;
    use std::collections::HashMap;
    use std::thread;

    fn create_test_packument(name: &str) -> Packument {
        let mut versions = HashMap::new();
        versions.insert(
            "1.0.0".to_string(),
            PackageMetadata {
                name: name.to_string(),
                version: "1.0.0".to_string(),
                dependencies: HashMap::new(),
                peer_dependencies: HashMap::new(),
                optional_dependencies: HashMap::new(),
                dev_dependencies: HashMap::new(),
            },
        );

        let mut dist_tags = HashMap::new();
        dist_tags.insert("latest".to_string(), "1.0.0".to_string());

        Packument {
            name: name.to_string(),
            versions,
            dist_tags,
        }
    }

    #[test]
    fn test_cache_insert_and_get() {
        let cache = PackumentCache::new();
        let packument = create_test_packument("express");

        cache.insert("express".to_string(), packument);
        let result = cache.get("express");

        assert!(result.is_some());
        assert_eq!(result.unwrap().name, "express");
    }

    #[test]
    fn test_cache_miss() {
        let cache = PackumentCache::new();
        let result = cache.get("nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_cache_expiration() {
        let cache = PackumentCache::with_config(Duration::from_millis(100), 1000);
        let packument = create_test_packument("express");

        cache.insert("express".to_string(), packument);
        assert!(cache.get("express").is_some());

        // Wait for expiration
        thread::sleep(Duration::from_millis(150));
        assert!(cache.get("express").is_none());
    }

    #[test]
    fn test_cache_eviction() {
        let cache = PackumentCache::with_config(Duration::from_secs(300), 2);

        cache.insert("pkg1".to_string(), create_test_packument("pkg1"));
        cache.insert("pkg2".to_string(), create_test_packument("pkg2"));
        assert_eq!(cache.len(), 2);

        // This should trigger eviction
        cache.insert("pkg3".to_string(), create_test_packument("pkg3"));
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_cache_clear() {
        let cache = PackumentCache::new();
        cache.insert("pkg1".to_string(), create_test_packument("pkg1"));
        cache.insert("pkg2".to_string(), create_test_packument("pkg2"));

        assert_eq!(cache.len(), 2);
        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_stats() {
        let cache = PackumentCache::with_config(Duration::from_secs(600), 500);
        cache.insert("pkg1".to_string(), create_test_packument("pkg1"));

        let stats = cache.stats();
        assert_eq!(stats.size, 1);
        assert_eq!(stats.max_size, 500);
        assert_eq!(stats.ttl_seconds, 600);
    }
}

// Made with Bob
