//! Serialization utilities for WeaveMesh Core
//!
//! This module provides efficient MessagePack serialization for all
//! WeaveMesh data structures, optimized for Zenoh transport.

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Serialize data to MessagePack format
pub fn serialize<T>(data: &T) -> Result<Vec<u8>>
where
    T: Serialize,
{
    rmp_serde::to_vec(data).map_err(Into::into)
}

/// Deserialize data from MessagePack format
pub fn deserialize<T>(bytes: &[u8]) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    rmp_serde::from_slice(bytes).map_err(Into::into)
}

/// Serialize data to JSON format (for debugging/human readability)
pub fn serialize_json<T>(data: &T) -> Result<String>
where
    T: Serialize,
{
    serde_json::to_string_pretty(data).map_err(Into::into)
}

/// Deserialize data from JSON format
pub fn deserialize_json<T>(json: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    serde_json::from_str(json).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Attribution, CollaborationType};

    #[test]
    fn test_messagepack_roundtrip() {
        let attribution = Attribution::new(
            Some("human".to_string()),
            Some("ai".to_string()),
            CollaborationType::CoCreated,
            0.9,
        );

        let serialized = serialize(&attribution).unwrap();
        let deserialized: Attribution = deserialize(&serialized).unwrap();

        assert_eq!(attribution.confidence, deserialized.confidence);
        assert_eq!(attribution.collaboration_type, deserialized.collaboration_type);
    }

    #[test]
    fn test_json_roundtrip() {
        let attribution = Attribution::new(
            Some("human".to_string()),
            Some("ai".to_string()),
            CollaborationType::PairProgramming,
            0.8,
        );

        let json = serialize_json(&attribution).unwrap();
        let deserialized: Attribution = deserialize_json(&json).unwrap();

        assert_eq!(attribution.confidence, deserialized.confidence);
        assert_eq!(attribution.collaboration_type, deserialized.collaboration_type);
    }
}
