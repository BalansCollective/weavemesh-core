//! Storage interface for WeaveMesh Core
//! 
//! This module provides a basic storage interface that can be implemented
//! by different storage backends (encrypted, cloud, distributed, etc.)

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Universal storage interface for WeaveMesh resources
pub trait Storage: Send + Sync {
    /// Store a resource and return its unique identifier
    async fn store_resource(
        &mut self,
        name: String,
        content: Vec<u8>,
        content_type: String,
        access_control: AccessControl,
        tags: Vec<String>,
    ) -> Result<String>;
    
    /// Retrieve a resource by its identifier
    async fn get_resource(&self, resource_id: &str) -> Result<StoredResource>;
    
    /// Get the content of a resource
    async fn get_resource_content(&self, resource_id: &str) -> Result<Vec<u8>>;
    
    /// List resources with optional filtering
    fn list_resources(&self, filter: Option<ResourceFilter>) -> Vec<ResourceMetadata>;
    
    /// Delete a resource
    async fn delete_resource(&mut self, resource_id: &str) -> Result<()>;
    
    /// Get storage statistics
    fn get_stats(&self) -> StorageStats;
}

/// Metadata about a stored resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadata {
    /// Unique identifier for this resource
    pub resource_id: String,
    
    /// Human-readable name for the resource
    pub name: String,
    
    /// MIME type of the resource content
    pub content_type: String,
    
    /// Size of the data in bytes
    pub size: u64,
    
    /// When this resource was created
    pub created_at: chrono::DateTime<chrono::Utc>,
    
    /// When this resource was last modified
    pub modified_at: chrono::DateTime<chrono::Utc>,
    
    /// Access control settings for this resource
    pub access_control: AccessControl,
    
    /// Tags for organizing resources
    pub tags: Vec<String>,
}

/// Access control settings for a resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControl {
    /// Whether this resource is private to this node
    pub is_private: bool,
    
    /// Node IDs that can access this resource
    pub allowed_nodes: Vec<String>,
    
    /// Family/group IDs that can access this resource
    pub allowed_groups: Vec<String>,
    
    /// Whether this resource can be shared publicly
    pub is_public: bool,
}

impl Default for AccessControl {
    fn default() -> Self {
        Self {
            is_private: true,
            allowed_nodes: Vec::new(),
            allowed_groups: Vec::new(),
            is_public: false,
        }
    }
}

/// Stored resource with content
#[derive(Debug, Clone)]
pub struct StoredResource {
    /// Metadata about this resource
    pub metadata: ResourceMetadata,
    
    /// Content of the resource
    pub content: Vec<u8>,
}

/// Filter for listing resources
#[derive(Debug, Clone)]
pub struct ResourceFilter {
    pub content_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub is_private: Option<bool>,
    pub name_contains: Option<String>,
}

impl ResourceFilter {
    pub fn matches(&self, metadata: &ResourceMetadata) -> bool {
        if let Some(ref content_type) = self.content_type {
            if &metadata.content_type != content_type {
                return false;
            }
        }
        
        if let Some(ref tags) = self.tags {
            if !tags.iter().any(|tag| metadata.tags.contains(tag)) {
                return false;
            }
        }
        
        if let Some(is_private) = self.is_private {
            if metadata.access_control.is_private != is_private {
                return false;
            }
        }
        
        if let Some(ref name_contains) = self.name_contains {
            if !metadata.name.to_lowercase().contains(&name_contains.to_lowercase()) {
                return false;
            }
        }
        
        true
    }
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_resources: usize,
    pub total_size: u64,
}

/// Simple in-memory storage implementation for testing and basic use
#[derive(Debug)]
pub struct MemoryStorage {
    resources: HashMap<String, StoredResource>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage for MemoryStorage {
    async fn store_resource(
        &mut self,
        name: String,
        content: Vec<u8>,
        content_type: String,
        access_control: AccessControl,
        tags: Vec<String>,
    ) -> Result<String> {
        let resource_id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        let metadata = ResourceMetadata {
            resource_id: resource_id.clone(),
            name,
            content_type,
            size: content.len() as u64,
            created_at: now,
            modified_at: now,
            access_control,
            tags,
        };
        
        let resource = StoredResource {
            metadata,
            content,
        };
        
        self.resources.insert(resource_id.clone(), resource);
        Ok(resource_id)
    }
    
    async fn get_resource(&self, resource_id: &str) -> Result<StoredResource> {
        self.resources
            .get(resource_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Resource not found: {}", resource_id))
    }
    
    async fn get_resource_content(&self, resource_id: &str) -> Result<Vec<u8>> {
        let resource = self.get_resource(resource_id).await?;
        Ok(resource.content)
    }
    
    fn list_resources(&self, filter: Option<ResourceFilter>) -> Vec<ResourceMetadata> {
        let mut resources: Vec<ResourceMetadata> = self.resources
            .values()
            .map(|r| r.metadata.clone())
            .collect();
        
        if let Some(filter) = filter {
            resources.retain(|metadata| filter.matches(metadata));
        }
        
        // Sort by modification time (newest first)
        resources.sort_by(|a, b| b.modified_at.cmp(&a.modified_at));
        
        resources
    }
    
    async fn delete_resource(&mut self, resource_id: &str) -> Result<()> {
        self.resources
            .remove(resource_id)
            .ok_or_else(|| anyhow::anyhow!("Resource not found: {}", resource_id))?;
        Ok(())
    }
    
    fn get_stats(&self) -> StorageStats {
        let total_resources = self.resources.len();
        let total_size: u64 = self.resources
            .values()
            .map(|r| r.metadata.size)
            .sum();
        
        StorageStats {
            total_resources,
            total_size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_memory_storage() {
        let mut storage = MemoryStorage::new();
        
        // Store a resource
        let content = b"Hello, WeaveMesh!".to_vec();
        let resource_id = storage.store_resource(
            "test.txt".to_string(),
            content.clone(),
            "text/plain".to_string(),
            AccessControl::default(),
            vec!["test".to_string()],
        ).await.unwrap();
        
        // Retrieve the resource
        let retrieved_content = storage.get_resource_content(&resource_id).await.unwrap();
        assert_eq!(content, retrieved_content);
        
        // Check metadata
        let resources = storage.list_resources(None);
        assert_eq!(resources.len(), 1);
        assert_eq!(resources[0].name, "test.txt");
        
        // Check stats
        let stats = storage.get_stats();
        assert_eq!(stats.total_resources, 1);
        assert_eq!(stats.total_size, content.len() as u64);
        
        // Delete resource
        storage.delete_resource(&resource_id).await.unwrap();
        let resources = storage.list_resources(None);
        assert_eq!(resources.len(), 0);
    }
    
    #[tokio::test]
    async fn test_resource_filtering() {
        let mut storage = MemoryStorage::new();
        
        // Store multiple resources
        storage.store_resource(
            "doc1.txt".to_string(),
            b"Document 1".to_vec(),
            "text/plain".to_string(),
            AccessControl::default(),
            vec!["document".to_string()],
        ).await.unwrap();
        
        storage.store_resource(
            "image1.png".to_string(),
            b"Image data".to_vec(),
            "image/png".to_string(),
            AccessControl::default(),
            vec!["image".to_string()],
        ).await.unwrap();
        
        // Filter by content type
        let filter = ResourceFilter {
            content_type: Some("text/plain".to_string()),
            tags: None,
            is_private: None,
            name_contains: None,
        };
        
        let filtered = storage.list_resources(Some(filter));
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].content_type, "text/plain");
    }
}
