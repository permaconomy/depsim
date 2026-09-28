//! npm registry client and caching

pub mod cache;
pub mod client;

pub use cache::{CacheStats, PackumentCache};
pub use client::{RegistryClient, DEFAULT_REGISTRY};

use crate::error::Result;
use crate::resolver::types::Packument;
use std::sync::Arc;

/// Registry service combining client and cache
pub struct RegistryService {
    client: RegistryClient,
    cache: Arc<PackumentCache>,
}

impl RegistryService {
    /// Create a new registry service with default settings
    pub fn new() -> Result<Self> {
        Ok(Self {
            client: RegistryClient::new()?,
            cache: Arc::new(PackumentCache::new()),
        })
    }

    /// Create a new registry service with custom registry URL
    pub fn with_registry(registry_url: String) -> Result<Self> {
        Ok(Self {
            client: RegistryClient::with_registry(registry_url)?,
            cache: Arc::new(PackumentCache::new()),
        })
    }

    /// Fetch a packument, using cache if available
    pub async fn fetch_packument(&self, package_name: &str) -> Result<Arc<Packument>> {
        // Check cache first
        if let Some(packument) = self.cache.get(package_name) {
            return Ok(packument);
        }

        // Fetch from registry
        let packument = self.client.fetch_packument(package_name).await?;

        // Store in cache
        self.cache.insert(package_name.to_string(), packument.clone());

        // Return from cache to get Arc
        Ok(self.cache.get(package_name).expect("Just inserted"))
    }

    /// Fetch multiple packuments in parallel with caching
    pub async fn fetch_packuments(
        &self,
        package_names: Vec<String>,
    ) -> Result<Vec<(String, Result<Arc<Packument>>)>> {
        let mut to_fetch = Vec::new();
        let mut results = Vec::new();

        // Check cache for each package
        for name in package_names {
            if let Some(packument) = self.cache.get(&name) {
                results.push((name, Ok(packument)));
            } else {
                to_fetch.push(name);
            }
        }

        // Fetch missing packages
        if !to_fetch.is_empty() {
            let fetch_results = self
                .client
                .fetch_packuments_with_limit(to_fetch, 50)
                .await?;

            for (name, result) in fetch_results {
                match result {
                    Ok(packument) => {
                        self.cache.insert(name.clone(), packument);
                        let cached = self.cache.get(&name).expect("Just inserted");
                        results.push((name, Ok(cached)));
                    }
                    Err(e) => {
                        results.push((name, Err(e)));
                    }
                }
            }
        }

        Ok(results)
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> CacheStats {
        self.cache.stats()
    }

    /// Clear the cache
    pub fn clear_cache(&self) {
        self.cache.clear();
    }
}

impl Default for RegistryService {
    fn default() -> Self {
        Self::new().expect("Failed to create default registry service")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_service() {
        let service = RegistryService::new();
        assert!(service.is_ok());
    }

    #[tokio::test]
    async fn test_custom_registry() {
        let service = RegistryService::with_registry("https://custom.registry.com".to_string());
        assert!(service.is_ok());
    }

    // Integration test - requires network
    #[tokio::test]
    #[ignore]
    async fn test_fetch_with_cache() {
        let service = RegistryService::new().unwrap();

        // First fetch - should hit registry
        let result1 = service.fetch_packument("express").await;
        assert!(result1.is_ok());

        // Second fetch - should hit cache
        let result2 = service.fetch_packument("express").await;
        assert!(result2.is_ok());

        // Both should be the same
        assert_eq!(result1.unwrap().name, result2.unwrap().name);
    }
}

// Made with Bob
