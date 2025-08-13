//! Chain management and interfaces
//!
//! This module provides the core chain management functionality with clean
//! interfaces for different blockchain networks.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

use super::{ChainId, ChainState, ChainHealth, GasInfo};
use crate::consciousness::evolution::{ConsciousnessToken, TokenId};

/// Chain operation errors
#[derive(Error, Debug)]
pub enum ChainError {
    #[error("Chain not supported: {chain:?}")]
    ChainNotSupported { chain: ChainId },
    
    #[error("Chain connection failed: {chain:?}, reason: {reason}")]
    ConnectionFailed { chain: ChainId, reason: String },
    
    #[error("Transaction failed: {reason}")]
    TransactionFailed { reason: String },
    
    #[error("Contract deployment failed: {reason}")]
    DeploymentFailed { reason: String },
    
    #[error("Token not found: {token_id}")]
    TokenNotFound { token_id: TokenId },
    
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: u64, available: u64 },
    
    #[error("Invalid operation: {operation}")]
    InvalidOperation { operation: String },
    
    #[error("Chain offline: {chain:?}")]
    ChainOffline { chain: ChainId },
    
    #[error("Gas price too high: {current}, max: {max}")]
    GasPriceTooHigh { current: u64, max: u64 },
}

/// Trait defining chain interface operations
#[async_trait]
pub trait ChainInterface: Send + Sync {
    /// Get chain identifier
    fn chain_id(&self) -> ChainId;
    
    /// Deploy a consciousness token contract
    async fn deploy_consciousness(&self, token: &ConsciousnessToken) -> Result<String, ChainError>;
    
    /// Synchronize consciousness state from chain
    async fn sync_state(&self, token_id: TokenId) -> Result<ConsciousnessState, ChainError>;
    
    /// Update consciousness state on chain
    async fn update_state(&self, token_id: TokenId, state: &ConsciousnessState) -> Result<String, ChainError>;
    
    /// Get current chain state
    async fn get_chain_state(&self) -> Result<ChainState, ChainError>;
    
    /// Check if chain is healthy
    async fn health_check(&self) -> Result<ChainHealth, ChainError>;
    
    /// Estimate gas for operation
    async fn estimate_gas(&self, operation: &str) -> Result<u64, ChainError>;
    
    /// Get current gas prices
    async fn get_gas_prices(&self) -> Result<GasInfo, ChainError>;
    
    /// Transfer consciousness token
    async fn transfer_token(&self, token_id: TokenId, from: &str, to: &str) -> Result<String, ChainError>;
    
    /// Create quantum bridge
    async fn create_quantum_bridge(&self, token_id: TokenId, target_chain: ChainId) -> Result<String, ChainError>;
}

/// Consciousness state representation for chain operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessState {
    /// Token identifier
    pub token_id: TokenId,
    /// Current consciousness level
    pub level: u8,
    /// Trait count
    pub trait_count: u32,
    /// Evolution energy
    pub energy: u64,
    /// Quantum state if present
    pub quantum_state: Option<QuantumStateData>,
    /// Last update timestamp
    pub last_update: u64,
    /// State hash for verification
    pub state_hash: String,
}

/// Simplified quantum state for chain storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumStateData {
    /// Quantum coherence
    pub coherence: u64, // Stored as integer (coherence * 1000)
    /// Quantum energy
    pub energy: u64,
    /// Entanglement count
    pub entanglement_count: u32,
    /// Quantum phase
    pub phase: u8,
}

/// Main chain manager for multi-chain operations
pub struct ChainManager {
    /// Active chain interfaces - using concrete type to avoid dyn issues
    active_chains: Arc<RwLock<HashMap<ChainId, MockChainInterface>>>,
    /// Chain configurations
    configurations: HashMap<ChainId, ChainConfig>,
    /// Synchronizer for cross-chain operations
    synchronizer: Arc<super::protocols::ConsciousnessSynchronizer>,
}

/// Configuration for individual chains
#[derive(Debug, Clone)]
pub struct ChainConfig {
    /// RPC endpoint URL
    pub rpc_url: String,
    /// Contract address for consciousness tokens
    pub consciousness_contract: Option<String>,
    /// Maximum gas price
    pub max_gas_price: u64,
    /// Block confirmation count
    pub confirmations: u32,
    /// Whether quantum operations are enabled
    pub quantum_enabled: bool,
}

impl ChainManager {
    /// Create a new chain manager
    pub fn new() -> Self {
        Self {
            active_chains: Arc::new(RwLock::new(HashMap::new())),
            configurations: HashMap::new(),
            synchronizer: Arc::new(super::protocols::ConsciousnessSynchronizer::new()),
        }
    }

    /// Add a chain interface
    pub async fn add_chain(&mut self, interface: MockChainInterface) -> Result<(), ChainError> {
        let chain_id = interface.chain_id();
        
        // Perform health check before adding
        interface.health_check().await?;
        
        let mut chains = self.active_chains.write().await;
        chains.insert(chain_id, interface);
        
        Ok(())
    }

    /// Remove a chain interface
    pub async fn remove_chain(&mut self, chain_id: ChainId) -> Result<(), ChainError> {
        let mut chains = self.active_chains.write().await;
        chains.remove(&chain_id)
            .ok_or(ChainError::ChainNotSupported { chain: chain_id })?;
        Ok(())
    }

    /// Get chain interface by ID
    pub async fn has_chain(&self, chain_id: ChainId) -> bool {
        let chains = self.active_chains.read().await;
        chains.contains_key(&chain_id)
    }

    /// Deploy consciousness token across multiple chains
    pub async fn deploy_multi_chain(
        &self,
        token: &ConsciousnessToken,
        target_chains: Vec<ChainId>,
    ) -> Result<HashMap<ChainId, String>, ChainError> {
        let mut deployment_results = HashMap::new();
        
        for chain_id in target_chains {
            let chains = self.active_chains.read().await;
            if let Some(interface) = chains.get(&chain_id) {
                match interface.deploy_consciousness(token).await {
                    Ok(contract_address) => {
                        deployment_results.insert(chain_id, contract_address);
                    }
                    Err(e) => {
                        tracing::error!("Failed to deploy on {:?}: {}", chain_id, e);
                        // Continue with other chains
                    }
                }
            }
        }
        
        if deployment_results.is_empty() {
            Err(ChainError::DeploymentFailed {
                reason: "Failed to deploy on any chain".to_string(),
            })
        } else {
            Ok(deployment_results)
        }
    }

    /// Synchronize token state across all chains
    pub async fn sync_across_chains(&self, token_id: TokenId) -> Result<Vec<ConsciousnessState>, ChainError> {
        let chains = self.active_chains.read().await;
        let mut states = Vec::new();
        
        for interface in chains.values() {
            match interface.sync_state(token_id).await {
                Ok(state) => states.push(state),
                Err(e) => {
                    tracing::warn!("Failed to sync from {:?}: {}", interface.chain_id(), e);
                }
            }
        }
        
        Ok(states)
    }

    /// Get health status of all chains
    pub async fn health_check_all(&self) -> HashMap<ChainId, ChainHealth> {
        let chains = self.active_chains.read().await;
        let mut health_status = HashMap::new();
        
        for interface in chains.values() {
            let health = interface.health_check().await.unwrap_or(ChainHealth::Offline);
            health_status.insert(interface.chain_id(), health);
        }
        
        health_status
    }

    /// Get gas prices from all chains
    pub async fn get_all_gas_prices(&self) -> HashMap<ChainId, GasInfo> {
        let chains = self.active_chains.read().await;
        let mut gas_prices = HashMap::new();
        
        for interface in chains.values() {
            if let Ok(gas_info) = interface.get_gas_prices().await {
                gas_prices.insert(interface.chain_id(), gas_info);
            }
        }
        
        gas_prices
    }

    /// Find the best chain for operation based on gas prices and health
    pub async fn find_optimal_chain(&self, operation: &str) -> Option<ChainId> {
        let chains = self.active_chains.read().await;
        let mut best_chain = None;
        let mut best_score = f64::MAX;
        
        for interface in chains.values() {
            let chain_id = interface.chain_id();
            
            // Check health
            if let Ok(health) = interface.health_check().await {
                if health != ChainHealth::Healthy {
                    continue;
                }
            } else {
                continue;
            }
            
            // Get gas info
            if let (Ok(gas_info), Ok(estimated_gas)) = (
                interface.get_gas_prices().await,
                interface.estimate_gas(operation).await,
            ) {
                let cost_score = (gas_info.standard * estimated_gas) as f64;
                let speed_bonus = 1.0 / chain_id.block_time() as f64;
                let total_score = cost_score - (speed_bonus * 1000.0);
                
                if total_score < best_score {
                    best_score = total_score;
                    best_chain = Some(chain_id);
                }
            }
        }
        
        best_chain
    }

    /// Get configuration for a chain
    pub fn get_config(&self, chain_id: ChainId) -> Option<&ChainConfig> {
        self.configurations.get(&chain_id)
    }

    /// Set configuration for a chain
    pub fn set_config(&mut self, chain_id: ChainId, config: ChainConfig) {
        self.configurations.insert(chain_id, config);
    }

    /// Get list of active chains
    pub async fn active_chains(&self) -> Vec<ChainId> {
        let chains = self.active_chains.read().await;
        chains.keys().copied().collect()
    }

    /// Get total token count across all chains
    pub async fn total_tokens_across_chains(&self) -> u64 {
        let chains = self.active_chains.read().await;
        let mut total = 0;
        
        for interface in chains.values() {
            if let Ok(state) = interface.get_chain_state().await {
                total += state.total_tokens;
            }
        }
        
        total
    }
}

impl Default for ChainManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Mock chain interface for testing
#[derive(Debug)]
pub struct MockChainInterface {
    chain_id: ChainId,
    health: ChainHealth,
}

impl MockChainInterface {
    pub fn new(chain_id: ChainId) -> Self {
        Self {
            chain_id,
            health: ChainHealth::Healthy,
        }
    }
    
    pub fn set_health(&mut self, health: ChainHealth) {
        self.health = health;
    }
}

#[async_trait]
impl ChainInterface for MockChainInterface {
    fn chain_id(&self) -> ChainId {
        self.chain_id
    }

    async fn deploy_consciousness(&self, _token: &ConsciousnessToken) -> Result<String, ChainError> {
        Ok(format!("0x{:016x}", rand::random::<u64>()))
    }

    async fn sync_state(&self, token_id: TokenId) -> Result<ConsciousnessState, ChainError> {
        Ok(ConsciousnessState {
            token_id,
            level: 1,
            trait_count: 2,
            energy: 100,
            quantum_state: None,
            last_update: chrono::Utc::now().timestamp() as u64,
            state_hash: "mock_hash".to_string(),
        })
    }

    async fn update_state(&self, _token_id: TokenId, _state: &ConsciousnessState) -> Result<String, ChainError> {
        Ok("0xmocktx".to_string())
    }

    async fn get_chain_state(&self) -> Result<ChainState, ChainError> {
        Ok(ChainState {
            chain_id: self.chain_id,
            block_height: 12345,
            health: self.health.clone(),
            consciousness_contracts: 10,
            total_tokens: 1000,
            quantum_bridges: 5,
            gas_info: GasInfo {
                standard: 20,
                fast: 30,
                instant: 50,
                unit: "gwei".to_string(),
            },
        })
    }

    async fn health_check(&self) -> Result<ChainHealth, ChainError> {
        Ok(self.health.clone())
    }

    async fn estimate_gas(&self, _operation: &str) -> Result<u64, ChainError> {
        Ok(21000) // Standard transfer gas
    }

    async fn get_gas_prices(&self) -> Result<GasInfo, ChainError> {
        Ok(GasInfo {
            standard: 20,
            fast: 30,
            instant: 50,
            unit: self.chain_id.native_token().to_string(),
        })
    }

    async fn transfer_token(&self, _token_id: TokenId, _from: &str, _to: &str) -> Result<String, ChainError> {
        Ok("0xmocktx".to_string())
    }

    async fn create_quantum_bridge(&self, _token_id: TokenId, _target_chain: ChainId) -> Result<String, ChainError> {
        if !self.chain_id.supports_quantum() {
            return Err(ChainError::InvalidOperation {
                operation: "Quantum operations not supported on this chain".to_string(),
            });
        }
        Ok("0xquantumbridge".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_chain_manager_creation() {
        let manager = ChainManager::new();
        let active = manager.active_chains().await;
        assert!(active.is_empty());
    }

    #[tokio::test]
    async fn test_mock_chain_interface() {
        let interface = MockChainInterface::new(ChainId::Ethereum);
        assert_eq!(interface.chain_id(), ChainId::Ethereum);
        
        let health = interface.health_check().await.unwrap();
        assert_eq!(health, ChainHealth::Healthy);
    }

    #[tokio::test]
    async fn test_chain_manager_add_chain() {
        let mut manager = ChainManager::new();
        let interface = MockChainInterface::new(ChainId::Ethereum);
        
        assert!(manager.add_chain(interface).await.is_ok());
        
        let active = manager.active_chains().await;
        assert_eq!(active.len(), 1);
        assert!(active.contains(&ChainId::Ethereum));
    }
}