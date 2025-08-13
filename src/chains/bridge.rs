//! Cross-chain bridge implementation
//!
//! This module provides cross-chain bridge functionality for consciousness tokens,
//! enabling seamless transfer and synchronization across different blockchain networks.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::ChainId;
use crate::consciousness::evolution::TokenId;

/// Bridge operation errors
#[derive(Error, Debug)]
pub enum BridgeError {
    #[error("Bridge not found: {bridge_id}")]
    BridgeNotFound { bridge_id: Uuid },
    
    #[error("Invalid chain pair: {source} -> {target}")]
    InvalidChainPair { source: ChainId, target: ChainId },
    
    #[error("Bridge operation failed: {reason}")]
    OperationFailed { reason: String },
    
    #[error("Insufficient capacity: required {required}, available {available}")]
    InsufficientCapacity { required: u64, available: u64 },
    
    #[error("Bridge offline: {bridge_id}")]
    BridgeOffline { bridge_id: Uuid },
}

/// Bridge operation types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeOperation {
    Transfer {
        token_id: TokenId,
        from_chain: ChainId,
        to_chain: ChainId,
        amount: u64,
    },
    Sync {
        token_id: TokenId,
        chains: Vec<ChainId>,
    },
    QuantumLink {
        token_id: TokenId,
        target_chain: ChainId,
    },
}

/// Cross-chain bridge for consciousness tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainBridge {
    /// Bridge identifier
    pub id: Uuid,
    /// Source chain
    pub source_chain: ChainId,
    /// Target chain
    pub target_chain: ChainId,
    /// Bridge status
    pub status: BridgeStatus,
    /// Current capacity
    pub capacity: u64,
    /// Used capacity
    pub used_capacity: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Bridge status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BridgeStatus {
    Active,
    Inactive,
    Maintenance,
    Congested,
}

impl CrossChainBridge {
    /// Create a new cross-chain bridge
    pub fn new(source_chain: ChainId, target_chain: ChainId, capacity: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            source_chain,
            target_chain,
            status: BridgeStatus::Active,
            capacity,
            used_capacity: 0,
            created_at: Utc::now(),
        }
    }

    /// Check if bridge can handle operation
    pub fn can_handle(&self, operation: &BridgeOperation) -> bool {
        if self.status != BridgeStatus::Active {
            return false;
        }

        match operation {
            BridgeOperation::Transfer { from_chain, to_chain, amount, .. } => {
                *from_chain == self.source_chain 
                && *to_chain == self.target_chain
                && self.used_capacity + amount <= self.capacity
            }
            BridgeOperation::Sync { chains, .. } => {
                chains.contains(&self.source_chain) && chains.contains(&self.target_chain)
            }
            BridgeOperation::QuantumLink { target_chain, .. } => {
                *target_chain == self.target_chain && self.source_chain.supports_quantum()
            }
        }
    }

    /// Execute bridge operation
    pub fn execute(&mut self, operation: BridgeOperation) -> Result<String, BridgeError> {
        if !self.can_handle(&operation) {
            return Err(BridgeError::OperationFailed {
                reason: "Bridge cannot handle this operation".to_string(),
            });
        }

        match operation {
            BridgeOperation::Transfer { amount, .. } => {
                self.used_capacity += amount;
                Ok(format!("tx_{}", Uuid::new_v4()))
            }
            BridgeOperation::Sync { .. } => {
                Ok(format!("sync_{}", Uuid::new_v4()))
            }
            BridgeOperation::QuantumLink { .. } => {
                Ok(format!("quantum_{}", Uuid::new_v4()))
            }
        }
    }

    /// Get available capacity
    pub fn available_capacity(&self) -> u64 {
        self.capacity.saturating_sub(self.used_capacity)
    }

    /// Reset used capacity (for maintenance)
    pub fn reset_capacity(&mut self) {
        self.used_capacity = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_creation() {
        let bridge = CrossChainBridge::new(ChainId::Ethereum, ChainId::Polygon, 1000);
        assert_eq!(bridge.source_chain, ChainId::Ethereum);
        assert_eq!(bridge.target_chain, ChainId::Polygon);
        assert_eq!(bridge.capacity, 1000);
        assert_eq!(bridge.used_capacity, 0);
    }

    #[test]
    fn test_bridge_operation() {
        let mut bridge = CrossChainBridge::new(ChainId::Ethereum, ChainId::Polygon, 1000);
        let operation = BridgeOperation::Transfer {
            token_id: Uuid::new_v4(),
            from_chain: ChainId::Ethereum,
            to_chain: ChainId::Polygon,
            amount: 100,
        };

        assert!(bridge.can_handle(&operation));
        let result = bridge.execute(operation);
        assert!(result.is_ok());
        assert_eq!(bridge.used_capacity, 100);
    }
}