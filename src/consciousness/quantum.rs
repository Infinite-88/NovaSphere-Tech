//! Quantum features for consciousness tokens
//!
//! This module implements quantum entanglement and cross-chain operations,
//! providing the quantum bridge functionality for multi-chain consciousness sync.

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use thiserror::Error;

use super::evolution::TokenId;
use crate::chains::ChainId;

/// Quantum state for consciousness tokens
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumState {
    /// Unique quantum state identifier
    pub id: Uuid,
    /// Quantum entanglement pairs
    pub entangled_tokens: Vec<TokenId>,
    /// Current quantum phase
    pub phase: QuantumPhase,
    /// Quantum coherence level (0.0 to 1.0)
    pub coherence: f64,
    /// Quantum energy level
    pub energy: u64,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last quantum operation timestamp
    pub last_operation: DateTime<Utc>,
}

/// Quantum phase states
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuantumPhase {
    /// Stable quantum state
    Stable,
    /// Superposition state
    Superposition,
    /// Entangled with other tokens
    Entangled,
    /// Tunneling between chains
    Tunneling,
    /// Collapsed quantum state
    Collapsed,
}

/// Quantum channel for cross-chain communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumChannel {
    /// Channel identifier
    pub id: Uuid,
    /// Source chain
    pub source_chain: ChainId,
    /// Target chain
    pub target_chain: ChainId,
    /// Channel capacity
    pub capacity: u64,
    /// Current usage
    pub usage: u64,
    /// Channel status
    pub status: ChannelStatus,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

/// Quantum channel status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChannelStatus {
    Active,
    Inactive,
    Congested,
    Maintenance,
}

/// Quantum bridge for cross-chain operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumBridge {
    /// Bridge identifier
    pub id: Uuid,
    /// Entangled token pairs
    pub entangled_pairs: HashMap<TokenId, TokenId>,
    /// Available quantum channels
    pub quantum_channels: Vec<QuantumChannel>,
    /// Bridge statistics
    pub stats: BridgeStatistics,
    /// Bridge configuration
    pub config: BridgeConfig,
}

/// Bridge operation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeStatistics {
    /// Total operations performed
    pub total_operations: u64,
    /// Successful operations
    pub successful_operations: u64,
    /// Failed operations
    pub failed_operations: u64,
    /// Average operation time in milliseconds
    pub avg_operation_time: f64,
    /// Total quantum energy consumed
    pub total_energy_consumed: u64,
}

/// Bridge configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    /// Maximum entanglement pairs
    pub max_entanglements: usize,
    /// Maximum channels
    pub max_channels: usize,
    /// Minimum coherence threshold
    pub min_coherence: f64,
    /// Energy cost per operation
    pub energy_per_operation: u64,
}

/// Quantum operation errors
#[derive(Error, Debug)]
pub enum QuantumError {
    #[error("Insufficient quantum energy: required {required}, available {available}")]
    InsufficientEnergy { required: u64, available: u64 },
    
    #[error("Token not found: {token_id}")]
    TokenNotFound { token_id: TokenId },
    
    #[error("Entanglement failed: {reason}")]
    EntanglementFailed { reason: String },
    
    #[error("Quantum tunnel failed: {reason}")]
    TunnelFailed { reason: String },
    
    #[error("Coherence too low: {current}, minimum required {required}")]
    LowCoherence { current: f64, required: f64 },
    
    #[error("Channel not available: {channel_id}")]
    ChannelUnavailable { channel_id: Uuid },
    
    #[error("Invalid quantum state: {state:?}")]
    InvalidState { state: QuantumPhase },
    
    #[error("Bridge capacity exceeded")]
    CapacityExceeded,
    
    #[error("Quantum decoherence detected")]
    Decoherence,
}

/// Trait for quantum operations
pub trait QuantumOperations {
    /// Create quantum entanglement between two tokens
    fn create_entanglement(&mut self, token1: TokenId, token2: TokenId) -> Result<(), QuantumError>;
    
    /// Break quantum entanglement
    fn break_entanglement(&mut self, token1: TokenId, token2: TokenId) -> Result<(), QuantumError>;
    
    /// Perform quantum tunnel operation
    fn quantum_tunnel(&self, token: TokenId, target_chain: ChainId) -> Result<(), QuantumError>;
    
    /// Measure quantum state
    fn measure_state(&self, token: TokenId) -> Result<QuantumState, QuantumError>;
    
    /// Synchronize quantum states
    fn synchronize_states(&mut self, tokens: Vec<TokenId>) -> Result<(), QuantumError>;
}

impl QuantumState {
    /// Create a new quantum state
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            entangled_tokens: Vec::new(),
            phase: QuantumPhase::Stable,
            coherence: 1.0,
            energy: 1000,
            created_at: now,
            last_operation: now,
        }
    }

    /// Create quantum state with specific energy
    pub fn with_energy(energy: u64) -> Self {
        let mut state = Self::new();
        state.energy = energy;
        state
    }

    /// Add entanglement with another token
    pub fn add_entanglement(&mut self, token_id: TokenId) -> Result<(), QuantumError> {
        if self.entangled_tokens.contains(&token_id) {
            return Err(QuantumError::EntanglementFailed {
                reason: "Token already entangled".to_string(),
            });
        }

        self.entangled_tokens.push(token_id);
        self.phase = QuantumPhase::Entangled;
        self.last_operation = Utc::now();
        Ok(())
    }

    /// Remove entanglement
    pub fn remove_entanglement(&mut self, token_id: TokenId) -> Result<(), QuantumError> {
        if let Some(pos) = self.entangled_tokens.iter().position(|x| *x == token_id) {
            self.entangled_tokens.remove(pos);
            if self.entangled_tokens.is_empty() {
                self.phase = QuantumPhase::Stable;
            }
            self.last_operation = Utc::now();
            Ok(())
        } else {
            Err(QuantumError::EntanglementFailed {
                reason: "Token not entangled".to_string(),
            })
        }
    }

    /// Check if entangled with specific token
    pub fn is_entangled_with(&self, token_id: TokenId) -> bool {
        self.entangled_tokens.contains(&token_id)
    }

    /// Update coherence level
    pub fn update_coherence(&mut self, new_coherence: f64) {
        self.coherence = new_coherence.clamp(0.0, 1.0);
        self.last_operation = Utc::now();
        
        // Auto-collapse if coherence too low
        if self.coherence < 0.1 {
            self.phase = QuantumPhase::Collapsed;
        }
    }

    /// Consume quantum energy
    pub fn consume_energy(&mut self, amount: u64) -> Result<(), QuantumError> {
        if self.energy < amount {
            return Err(QuantumError::InsufficientEnergy {
                required: amount,
                available: self.energy,
            });
        }
        
        self.energy -= amount;
        self.last_operation = Utc::now();
        Ok(())
    }

    /// Add quantum energy
    pub fn add_energy(&mut self, amount: u64) {
        self.energy += amount;
        self.last_operation = Utc::now();
    }

    /// Check if state is stable
    pub fn is_stable(&self) -> bool {
        matches!(self.phase, QuantumPhase::Stable) && self.coherence > 0.8
    }
}

impl Default for QuantumState {
    fn default() -> Self {
        Self::new()
    }
}

impl BridgeConfig {
    /// Create default bridge configuration
    pub fn default() -> Self {
        Self {
            max_entanglements: 1000,
            max_channels: 10,
            min_coherence: 0.5,
            energy_per_operation: 10,
        }
    }

    /// Create high-capacity configuration
    pub fn high_capacity() -> Self {
        Self {
            max_entanglements: 10000,
            max_channels: 50,
            min_coherence: 0.3,
            energy_per_operation: 20,
        }
    }
}

impl QuantumBridge {
    /// Create a new quantum bridge
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            entangled_pairs: HashMap::new(),
            quantum_channels: Vec::new(),
            stats: BridgeStatistics {
                total_operations: 0,
                successful_operations: 0,
                failed_operations: 0,
                avg_operation_time: 0.0,
                total_energy_consumed: 0,
            },
            config: BridgeConfig::default(),
        }
    }

    /// Create bridge with custom configuration
    pub fn with_config(config: BridgeConfig) -> Self {
        let mut bridge = Self::new();
        bridge.config = config;
        bridge
    }

    /// Add a quantum channel
    pub fn add_channel(&mut self, source: ChainId, target: ChainId, capacity: u64) -> Result<Uuid, QuantumError> {
        if self.quantum_channels.len() >= self.config.max_channels {
            return Err(QuantumError::CapacityExceeded);
        }

        let channel = QuantumChannel {
            id: Uuid::new_v4(),
            source_chain: source,
            target_chain: target,
            capacity,
            usage: 0,
            status: ChannelStatus::Active,
            created_at: Utc::now(),
        };

        let channel_id = channel.id;
        self.quantum_channels.push(channel);
        Ok(channel_id)
    }

    /// Find available channel for chain pair
    pub fn find_channel(&self, source: ChainId, target: ChainId) -> Option<&QuantumChannel> {
        self.quantum_channels
            .iter()
            .find(|c| {
                c.source_chain == source 
                && c.target_chain == target 
                && c.status == ChannelStatus::Active
                && c.usage < c.capacity
            })
    }

    /// Calculate bridge efficiency
    pub fn efficiency(&self) -> f64 {
        if self.stats.total_operations == 0 {
            1.0
        } else {
            self.stats.successful_operations as f64 / self.stats.total_operations as f64
        }
    }

    /// Get bridge status summary
    pub fn status_summary(&self) -> String {
        format!(
            "Bridge {}: {} entanglements, {} channels, {:.2}% efficiency",
            self.id,
            self.entangled_pairs.len(),
            self.quantum_channels.len(),
            self.efficiency() * 100.0
        )
    }
}

impl QuantumOperations for QuantumBridge {
    fn create_entanglement(&mut self, token1: TokenId, token2: TokenId) -> Result<(), QuantumError> {
        if self.entangled_pairs.len() >= self.config.max_entanglements {
            return Err(QuantumError::CapacityExceeded);
        }

        if self.entangled_pairs.contains_key(&token1) || self.entangled_pairs.contains_key(&token2) {
            return Err(QuantumError::EntanglementFailed {
                reason: "One or both tokens already entangled".to_string(),
            });
        }

        self.entangled_pairs.insert(token1, token2);
        self.entangled_pairs.insert(token2, token1);
        
        self.stats.total_operations += 1;
        self.stats.successful_operations += 1;
        self.stats.total_energy_consumed += self.config.energy_per_operation;

        Ok(())
    }

    fn break_entanglement(&mut self, token1: TokenId, token2: TokenId) -> Result<(), QuantumError> {
        if !self.entangled_pairs.contains_key(&token1) {
            return Err(QuantumError::TokenNotFound { token_id: token1 });
        }

        self.entangled_pairs.remove(&token1);
        self.entangled_pairs.remove(&token2);
        
        self.stats.total_operations += 1;
        self.stats.successful_operations += 1;

        Ok(())
    }

    fn quantum_tunnel(&self, token: TokenId, _target_chain: ChainId) -> Result<(), QuantumError> {
        // Implementation would interact with actual chain
        // For now, we simulate the operation
        if !self.entangled_pairs.contains_key(&token) {
            return Err(QuantumError::TokenNotFound { token_id: token });
        }

        // Simulate tunnel success based on coherence and energy
        let success_rate = 0.9; // 90% success rate for simulation
        let success = rand::random::<f64>() < success_rate;

        if !success {
            return Err(QuantumError::TunnelFailed {
                reason: "Quantum interference detected".to_string(),
            });
        }

        Ok(())
    }

    fn measure_state(&self, token: TokenId) -> Result<QuantumState, QuantumError> {
        // For simulation, return a default quantum state
        // In real implementation, this would query the actual token state
        if !self.entangled_pairs.contains_key(&token) {
            return Err(QuantumError::TokenNotFound { token_id: token });
        }

        Ok(QuantumState::new())
    }

    fn synchronize_states(&mut self, tokens: Vec<TokenId>) -> Result<(), QuantumError> {
        // Verify all tokens are entangled
        for token in &tokens {
            if !self.entangled_pairs.contains_key(token) {
                return Err(QuantumError::TokenNotFound { token_id: *token });
            }
        }

        // Simulate synchronization
        self.stats.total_operations += 1;
        self.stats.successful_operations += 1;
        self.stats.total_energy_consumed += self.config.energy_per_operation * tokens.len() as u64;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_state_creation() {
        let state = QuantumState::new();
        assert_eq!(state.phase, QuantumPhase::Stable);
        assert_eq!(state.coherence, 1.0);
        assert!(state.entangled_tokens.is_empty());
    }

    #[test]
    fn test_entanglement() {
        let mut state = QuantumState::new();
        let token_id = Uuid::new_v4();
        
        assert!(state.add_entanglement(token_id).is_ok());
        assert_eq!(state.phase, QuantumPhase::Entangled);
        assert!(state.is_entangled_with(token_id));
    }

    #[test]
    fn test_energy_consumption() {
        let mut state = QuantumState::with_energy(100);
        
        assert!(state.consume_energy(50).is_ok());
        assert_eq!(state.energy, 50);
        
        assert!(state.consume_energy(100).is_err());
    }

    #[test]
    fn test_bridge_creation() {
        let bridge = QuantumBridge::new();
        assert!(bridge.entangled_pairs.is_empty());
        assert!(bridge.quantum_channels.is_empty());
        assert_eq!(bridge.efficiency(), 1.0);
    }

    #[test]
    fn test_bridge_entanglement() {
        let mut bridge = QuantumBridge::new();
        let token1 = Uuid::new_v4();
        let token2 = Uuid::new_v4();
        
        assert!(bridge.create_entanglement(token1, token2).is_ok());
        assert_eq!(bridge.entangled_pairs.len(), 2);
        assert!(bridge.break_entanglement(token1, token2).is_ok());
        assert!(bridge.entangled_pairs.is_empty());
    }

    #[test]
    fn test_channel_management() {
        let mut bridge = QuantumBridge::new();
        let source = ChainId::Ethereum;
        let target = ChainId::Polygon;
        
        let channel_id = bridge.add_channel(source, target, 1000).unwrap();
        assert_eq!(bridge.quantum_channels.len(), 1);
        
        let channel = bridge.find_channel(source, target);
        assert!(channel.is_some());
        assert_eq!(channel.unwrap().id, channel_id);
    }
}