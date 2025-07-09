//! # Universal Mesh Node Management
//!
//! Handles individual nodes in the WeaveMesh network with universal communication
//! primitives that work across all contexts and scales.

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::WeaveMeshError;
use crate::sacred_alliance::SacredAllianceLevel;
use super::health::HealthStatus;

/// A universal node in the WeaveMesh network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshNode {
    /// Unique identifier for this node
    pub id: Uuid,
    
    /// Node information and capabilities
    pub info: NodeInfo,
    
    /// Current health status
    pub health_status: HealthStatus,
    
    /// Last time this node was seen
    pub last_seen: DateTime<Utc>,
    
    /// Resources hosted by this node
    pub resources: Vec<String>,
}

/// Universal information about a mesh node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeInfo {
    /// Human-readable name for the node
    pub name: String,
    
    /// Node type and role
    pub node_type: NodeType,
    
    /// Universal capabilities this node provides
    pub capabilities: Vec<NodeCapability>,
    
    /// Network endpoints for this node
    pub endpoints: Vec<NodeEndpoint>,
    
    /// Node metadata
    pub metadata: HashMap<String, String>,
    
    /// Sacred Alliance participation level
    pub sacred_alliance_level: SacredAllianceLevel,
    
    /// Node version information
    pub version: NodeVersion,
}

/// Universal types of nodes in the mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    /// Development workstation
    Workstation {
        user: String,
        machine: String,
    },
    
    /// Continuous integration server
    CIServer {
        provider: String,
        environment: String,
    },
    
    /// Shared development server
    DevServer {
        team: String,
        purpose: String,
    },
    
    /// Mobile device
    Mobile {
        platform: String,
        device_type: String,
    },
    
    /// Cloud instance
    Cloud {
        provider: String,
        region: String,
        instance_type: String,
    },
    
    /// Edge device
    Edge {
        location: String,
        purpose: String,
    },
    
    /// Family communication node
    Family {
        household: String,
        role: String,
    },
    
    /// Universal node type
    Universal {
        context: String,
        scale: String,
    },
}

/// Universal capabilities a node can provide
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeCapability {
    /// Universal communication
    Communication {
        protocols: Vec<String>,
        max_connections: usize,
    },
    
    /// Pattern recognition
    PatternRecognition {
        analysis_types: Vec<String>,
        max_complexity: usize,
    },
    
    /// Sacred Alliance facilitation
    SacredAllianceFacilitation {
        max_participants: usize,
        ceremony_types: Vec<String>,
    },
    
    /// Resource hosting
    ResourceHosting {
        max_resources: Option<usize>,
        storage_capacity: Option<f64>,
    },
    
    /// Collaborative individuation
    CollaborativeIndividuation {
        max_sessions: usize,
        supported_methods: Vec<String>,
    },
    
    /// Context adaptation
    ContextAdaptation {
        supported_contexts: Vec<String>,
        adaptation_speed: f64,
    },
    
    /// Custom capability
    Custom {
        name: String,
        properties: HashMap<String, String>,
    },
}

/// Universal network endpoints for a node
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeEndpoint {
    /// Endpoint type
    pub endpoint_type: EndpointType,
    
    /// Network address
    pub address: String,
    
    /// Port number
    pub port: u16,
    
    /// Whether this endpoint uses TLS
    pub secure: bool,
    
    /// Endpoint priority (lower = higher priority)
    pub priority: u8,
}

/// Universal types of network endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndpointType {
    /// Zenoh router endpoint
    ZenohRouter,
    
    /// HTTP API endpoint
    HttpApi,
    
    /// WebSocket endpoint
    WebSocket,
    
    /// Universal communication endpoint
    Universal(String),
    
    /// Custom protocol endpoint
    Custom(String),
}

/// Universal node version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeVersion {
    /// WeaveMesh Core version
    pub weavemesh_core_version: String,
    
    /// Protocol version
    pub protocol_version: String,
    
    /// Build timestamp
    pub build_timestamp: DateTime<Utc>,
    
    /// Git commit hash
    pub git_commit: Option<String>,
}

/// Universal node announcement for mesh discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeAnnouncement {
    /// Node ID
    pub node_id: Uuid,
    
    /// Node information
    pub node_info: NodeInfo,
    
    /// Announcement timestamp
    pub timestamp: DateTime<Utc>,
}

/// Universal node performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetrics {
    /// CPU usage percentage (0.0 to 100.0)
    pub cpu_usage: f64,
    
    /// Memory usage percentage (0.0 to 100.0)
    pub memory_usage: f64,
    
    /// Network usage in relative units
    pub network_usage: f64,
    
    /// Number of active connections
    pub active_connections: usize,
    
    /// Number of hosted resources
    pub hosted_resources: usize,
    
    /// Average response time in milliseconds
    pub avg_response_time: f64,
    
    /// Uptime in seconds
    pub uptime_seconds: u64,
    
    /// Collaborative individuation score
    pub collaboration_score: f64,
    
    /// Last update timestamp
    pub last_update: DateTime<Utc>,
}

impl MeshNode {
    /// Create a new universal mesh node
    pub async fn new_universal() -> Result<Self> {
        let id = Uuid::new_v4();
        
        // Detect local system information
        let hostname = hostname::get()
            .map_err(|e| WeaveMeshError::SystemError(format!("Failed to get hostname: {}", e)))?
            .to_string_lossy()
            .to_string();
        
        let username = std::env::var("USER")
            .or_else(|_| std::env::var("USERNAME"))
            .unwrap_or_else(|_| "unknown".to_string());
        
        // Determine universal node type
        let node_type = if std::env::var("CI").is_ok() {
            NodeType::CIServer {
                provider: std::env::var("CI_PROVIDER").unwrap_or_else(|_| "unknown".to_string()),
                environment: std::env::var("CI_ENVIRONMENT").unwrap_or_else(|_| "unknown".to_string()),
            }
        } else if std::env::var("WEAVEMESH_FAMILY_MODE").is_ok() {
            NodeType::Family {
                household: std::env::var("WEAVEMESH_HOUSEHOLD").unwrap_or_else(|_| "default".to_string()),
                role: std::env::var("WEAVEMESH_FAMILY_ROLE").unwrap_or_else(|_| "member".to_string()),
            }
        } else {
            NodeType::Universal {
                context: std::env::var("WEAVEMESH_CONTEXT").unwrap_or_else(|_| "development".to_string()),
                scale: std::env::var("WEAVEMESH_SCALE").unwrap_or_else(|_| "individual".to_string()),
            }
        };
        
        // Detect universal capabilities
        let mut capabilities = Vec::new();
        
        // Universal communication capability
        capabilities.push(NodeCapability::Communication {
            protocols: vec!["zenoh".to_string(), "http".to_string()],
            max_connections: 100,
        });
        
        // Pattern recognition capability
        capabilities.push(NodeCapability::PatternRecognition {
            analysis_types: vec!["collaborative".to_string(), "individual".to_string()],
            max_complexity: 1000,
        });
        
        // Sacred Alliance facilitation
        capabilities.push(NodeCapability::SacredAllianceFacilitation {
            max_participants: 10,
            ceremony_types: vec![
                "validation".to_string(),
                "integration".to_string(),
                "conflict_resolution".to_string(),
            ],
        });
        
        // Resource hosting
        capabilities.push(NodeCapability::ResourceHosting {
            max_resources: Some(1000),
            storage_capacity: Some(10.0),
        });
        
        // Collaborative individuation
        capabilities.push(NodeCapability::CollaborativeIndividuation {
            max_sessions: 5,
            supported_methods: vec![
                "pattern_recognition".to_string(),
                "reality_anchoring".to_string(),
                "dimensional_navigation".to_string(),
            ],
        });
        
        // Context adaptation
        capabilities.push(NodeCapability::ContextAdaptation {
            supported_contexts: vec![
                "family".to_string(),
                "development".to_string(),
                "enterprise".to_string(),
            ],
            adaptation_speed: 1.0,
        });
        
        // Create universal endpoints
        let endpoints = vec![
            NodeEndpoint {
                endpoint_type: EndpointType::ZenohRouter,
                address: "0.0.0.0".to_string(),
                port: 7447,
                secure: false,
                priority: 1,
            },
            NodeEndpoint {
                endpoint_type: EndpointType::HttpApi,
                address: "0.0.0.0".to_string(),
                port: 8080,
                secure: false,
                priority: 2,
            },
            NodeEndpoint {
                endpoint_type: EndpointType::Universal("weavemesh".to_string()),
                address: "0.0.0.0".to_string(),
                port: 9090,
                secure: true,
                priority: 0,
            },
        ];
        
        let info = NodeInfo {
            name: format!("{}@{}", username, hostname),
            node_type,
            capabilities,
            endpoints,
            metadata: HashMap::new(),
            sacred_alliance_level: SacredAllianceLevel::Basic,
            version: NodeVersion {
                weavemesh_core_version: env!("CARGO_PKG_VERSION").to_string(),
                protocol_version: "1.0".to_string(),
                build_timestamp: Utc::now(),
                git_commit: option_env!("GIT_HASH").map(|s| s.to_string()),
            },
        };
        
        Ok(Self {
            id,
            info,
            health_status: HealthStatus::Healthy,
            last_seen: Utc::now(),
            resources: Vec::new(),
        })
    }
    
    /// Create a node from announcement
    pub fn from_announcement(announcement: NodeAnnouncement) -> Self {
        Self {
            id: announcement.node_id,
            info: announcement.node_info,
            health_status: HealthStatus::Healthy,
            last_seen: announcement.timestamp,
            resources: Vec::new(),
        }
    }
    
    /// Check if this node has a specific universal capability
    pub fn has_capability(&self, capability_type: &str) -> bool {
        self.info.capabilities.iter().any(|cap| {
            match cap {
                NodeCapability::Communication { .. } => capability_type == "communication",
                NodeCapability::PatternRecognition { .. } => capability_type == "pattern_recognition",
                NodeCapability::SacredAllianceFacilitation { .. } => capability_type == "sacred_alliance",
                NodeCapability::ResourceHosting { .. } => capability_type == "resource_hosting",
                NodeCapability::CollaborativeIndividuation { .. } => capability_type == "collaborative_individuation",
                NodeCapability::ContextAdaptation { .. } => capability_type == "context_adaptation",
                NodeCapability::Custom { name, .. } => name == capability_type,
            }
        })
    }
    
    /// Get the best endpoint for a specific type
    pub fn get_endpoint(&self, endpoint_type: EndpointType) -> Option<&NodeEndpoint> {
        self.info.endpoints
            .iter()
            .filter(|ep| std::mem::discriminant(&ep.endpoint_type) == std::mem::discriminant(&endpoint_type))
            .min_by_key(|ep| ep.priority)
    }
    
    /// Update the last seen timestamp
    pub fn update_last_seen(&mut self) {
        self.last_seen = Utc::now();
    }
    
    /// Add a resource to this node
    pub fn add_resource(&mut self, resource_id: String) {
        if !self.resources.contains(&resource_id) {
            self.resources.push(resource_id);
        }
    }
    
    /// Remove a resource from this node
    pub fn remove_resource(&mut self, resource_id: &str) {
        self.resources.retain(|id| id != resource_id);
    }
    
    /// Check if this node is considered stale
    pub fn is_stale(&self, stale_threshold_seconds: u64) -> bool {
        let now = Utc::now();
        let threshold = chrono::Duration::seconds(stale_threshold_seconds as i64);
        now.signed_duration_since(self.last_seen) > threshold
    }
    
    /// Get universal node metrics
    pub async fn get_metrics(&self) -> Result<NodeMetrics> {
        // In a real implementation, this would collect actual system metrics
        // For now, return universal mock data
        Ok(NodeMetrics {
            cpu_usage: 25.0,
            memory_usage: 60.0,
            network_usage: 10.0,
            active_connections: 5,
            hosted_resources: self.resources.len(),
            avg_response_time: 50.0,
            uptime_seconds: 3600,
            collaboration_score: 0.8,
            last_update: Utc::now(),
        })
    }
    
    /// Check if this node can participate in Sacred Alliance
    pub fn can_participate_in_sacred_alliance(&self) -> bool {
        self.info.can_participate_in_ceremonies()
    }
    
    /// Get collaborative individuation capacity
    pub fn get_collaboration_capacity(&self) -> usize {
        self.info.capabilities
            .iter()
            .filter_map(|cap| {
                if let NodeCapability::CollaborativeIndividuation { max_sessions, .. } = cap {
                    Some(*max_sessions)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0)
    }
}

impl NodeInfo {
    /// Check if this node can participate in Sacred Alliance ceremonies
    pub fn can_participate_in_ceremonies(&self) -> bool {
        matches!(
            self.sacred_alliance_level,
            SacredAllianceLevel::Basic
                | SacredAllianceLevel::Active
                | SacredAllianceLevel::Facilitator
                | SacredAllianceLevel::Guardian
        )
    }
    
    /// Check if this node can facilitate ceremonies
    pub fn can_facilitate_ceremonies(&self) -> bool {
        matches!(
            self.sacred_alliance_level,
            SacredAllianceLevel::Facilitator | SacredAllianceLevel::Guardian
        )
    }
    
    /// Get the maximum number of ceremony participants this node can handle
    pub fn max_ceremony_participants(&self) -> usize {
        self.capabilities
            .iter()
            .filter_map(|cap| {
                if let NodeCapability::SacredAllianceFacilitation { max_participants, .. } = cap {
                    Some(*max_participants)
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0)
    }
    
    /// Check if this node supports a specific context
    pub fn supports_context(&self, context: &str) -> bool {
        self.capabilities.iter().any(|cap| {
            if let NodeCapability::ContextAdaptation { supported_contexts, .. } = cap {
                supported_contexts.contains(&context.to_string())
            } else {
                false
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_universal_node_creation() {
        let node = MeshNode::new_universal().await.unwrap();
        assert!(!node.info.name.is_empty());
        assert!(!node.info.capabilities.is_empty());
        assert!(!node.info.endpoints.is_empty());
        assert!(node.has_capability("communication"));
        assert!(node.has_capability("pattern_recognition"));
        assert!(node.has_capability("collaborative_individuation"));
    }

    #[test]
    fn test_universal_capability_checking() {
        let mut node = MeshNode {
            id: Uuid::new_v4(),
            info: NodeInfo {
                name: "test".to_string(),
                node_type: NodeType::Universal {
                    context: "test".to_string(),
                    scale: "individual".to_string(),
                },
                capabilities: vec![
                    NodeCapability::Communication {
                        protocols: vec!["zenoh".to_string()],
                        max_connections: 10,
                    }
                ],
                endpoints: Vec::new(),
                metadata: HashMap::new(),
                sacred_alliance_level: SacredAllianceLevel::Basic,
                version: NodeVersion {
                    weavemesh_core_version: "0.1.0".to_string(),
                    protocol_version: "1.0".to_string(),
                    build_timestamp: Utc::now(),
                    git_commit: None,
                },
            },
            health_status: HealthStatus::Healthy,
            last_seen: Utc::now(),
            resources: Vec::new(),
        };

        assert!(node.has_capability("communication"));
        assert!(!node.has_capability("pattern_recognition"));
        assert!(node.can_participate_in_sacred_alliance());
    }

    #[test]
    fn test_context_support() {
        let info = NodeInfo {
            name: "test".to_string(),
            node_type: NodeType::Universal {
                context: "test".to_string(),
                scale: "individual".to_string(),
            },
            capabilities: vec![
                NodeCapability::ContextAdaptation {
                    supported_contexts: vec!["family".to_string(), "development".to_string()],
                    adaptation_speed: 1.0,
                }
            ],
            endpoints: Vec::new(),
            metadata: HashMap::new(),
            sacred_alliance_level: SacredAllianceLevel::Basic,
            version: NodeVersion {
                weavemesh_core_version: "0.1.0".to_string(),
                protocol_version: "1.0".to_string(),
                build_timestamp: Utc::now(),
                git_commit: None,
            },
        };

        assert!(info.supports_context("family"));
        assert!(info.supports_context("development"));
        assert!(!info.supports_context("enterprise"));
    }
}
