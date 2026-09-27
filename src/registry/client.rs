//! Async npm registry client

use crate::error::{DepsimError, Result};
use crate::resolver::types::Packument;
use reqwest::Client;
use std::time::Duration;
use tokio::time::timeout;

/// Default npm registry URL
pub const DEFAULT_REGISTRY: &str = "https://registry.npmjs.org";

/// Registry client for fetching package metadata
pub struct RegistryClient {
    /// HTTP client
    client: Client,
    /// Registry base URL
    registry_url: String,
    /// Request timeout
    timeout: Duration,
}

impl RegistryClient {
    /// Create a new registry client with default settings
    pub fn new() -> Result<Self> {
        Self::with_registry(DEFAULT_REGISTRY.to_string())
    }

    /// Create a new registry client with custom registry URL
    pub fn with_registry(registry_url: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .gzip(true)
            .user_agent("depsim/0.1.0")
            .build()?;

        Ok(Self {
            client,
            registry_url,
            timeout: Duration::from_secs(5),
        })
    }

    /// Set custom timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Fetch packument for a package
    pub async fn fetch_packument(&self, package_name: &str) -> Result<Packument> {
        let url = format!("{}/{}", self.registry_url, package_name);

        let response = timeout(self.timeout, self.client.get(&url).send())
            .await
            .map_err(|_| DepsimError::TimeoutError(format!("Fetching {}", package_name)))?
            .map_err(|e| DepsimError::NetworkError(e))?;

        if response.status().is_success() {
            let packument = response.json::<Packument>().await?;
            Ok(packument)
        } else if response.status().as_u16() == 404 {
            Err(DepsimError::PackageNotFound(package_name.to_string()))
        } else {
            Err(DepsimError::RegistryError(format!(
                "HTTP {} for package {}",
                response.status(),
                package_name
            )))
        }
    }

    /// Fetch multiple packuments in parallel
    pub async fn fetch_packuments_parallel(
        &self,
        package_names: Vec<String>,
    ) -> Result<Vec<(String, Result<Packument>)>> {
        let tasks: Vec<_> = package_names
            .into_iter()
            .map(|name| {
                let client = self.clone();
                let package_name = name.clone();
                tokio::spawn(async move {
                    let result = client.fetch_packument(&package_name).await;
                    (package_name, result)
                })
            })
            .collect();

        let mut results = Vec::new();
        for task in tasks {
            match task.await {
                Ok(result) => results.push(result),
                Err(e) => {
                    return Err(DepsimError::RegistryError(format!(
                        "Task join error: {}",
                        e
                    )))
                }
            }
        }

        Ok(results)
    }

    /// Fetch packuments with a maximum concurrency limit
    pub async fn fetch_packuments_with_limit(
        &self,
        package_names: Vec<String>,
        max_concurrent: usize,
    ) -> Result<Vec<(String, Result<Packument>)>> {
        use futures::stream::{self, StreamExt};

        let client = self.clone();
        let results: Vec<_> = stream::iter(package_names)
            .map(|name| {
                let client = client.clone();
                async move {
                    let result = client.fetch_packument(&name).await;
                    (name, result)
                }
            })
            .buffer_unordered(max_concurrent)
            .collect()
            .await;

        Ok(results)
    }
}

impl Clone for RegistryClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            registry_url: self.registry_url.clone(),
            timeout: self.timeout,
        }
    }
}

impl Default for RegistryClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default registry client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_client() {
        let client = RegistryClient::new();
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_custom_registry() {
        let client = RegistryClient::with_registry("https://custom.registry.com".to_string());
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.registry_url, "https://custom.registry.com");
    }

    #[tokio::test]
    async fn test_custom_timeout() {
        let client = RegistryClient::new()
            .unwrap()
            .with_timeout(Duration::from_secs(10));
        assert_eq!(client.timeout, Duration::from_secs(10));
    }

    // Integration test - requires network
    #[tokio::test]
    #[ignore]
    async fn test_fetch_express() {
        let client = RegistryClient::new().unwrap();
        let result = client.fetch_packument("express").await;
        assert!(result.is_ok());
        let packument = result.unwrap();
        assert_eq!(packument.name, "express");
        assert!(!packument.versions.is_empty());
    }

    // Integration test - requires network
    #[tokio::test]
    #[ignore]
    async fn test_fetch_nonexistent_package() {
        let client = RegistryClient::new().unwrap();
        let result = client
            .fetch_packument("this-package-definitely-does-not-exist-12345")
            .await;
        assert!(result.is_err());
        match result {
            Err(DepsimError::PackageNotFound(_)) => (),
            _ => panic!("Expected PackageNotFound error"),
        }
    }

    // Integration test - requires network
    #[tokio::test]
    #[ignore]
    async fn test_fetch_multiple_packages() {
        let client = RegistryClient::new().unwrap();
        let packages = vec!["express".to_string(), "lodash".to_string()];
        let results = client.fetch_packuments_parallel(packages).await;
        assert!(results.is_ok());
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
    }
}

// Made with Bob
