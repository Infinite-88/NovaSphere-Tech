//! Configuration management for NovaForge system
//!
//! This module provides configuration structures and management
//! for all system components.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

use crate::chains::ChainId;

/// Configuration errors
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration file not found: {path}")]
    FileNotFound { path: String },
    
    #[error("Invalid configuration format: {reason}")]
    InvalidFormat { reason: String },
    
    #[error("Missing required field: {field}")]
    MissingField { field: String },
    
    #[error("Invalid value for field {field}: {value}")]
    InvalidValue { field: String, value: String },
}

/// Main NovaForge system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovaForgeConfig {
    /// Chain-specific configurations
    pub chains: ChainConfig,
    /// Consciousness system configuration
    pub consciousness: ConsciousnessConfig,
    /// Quantum system configuration
    pub quantum: QuantumConfig,
    /// AI system configuration
    pub ai: AIConfig,
    /// Logging configuration
    pub logging: LoggingConfig,
}

/// Chain configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainConfig {
    /// Default chain for operations
    pub default_chain: ChainId,
    /// Chain-specific settings
    pub chain_settings: HashMap<ChainId, ChainSettings>,
    /// Global chain settings
    pub global: GlobalChainSettings,
}

/// Settings for individual chains
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainSettings {
    /// RPC endpoint URL
    pub rpc_url: String,
    /// Maximum gas price (in chain-native units)
    pub max_gas_price: u64,
    /// Block confirmation count
    pub confirmations: u32,
    /// Whether the chain is enabled
    pub enabled: bool,
    /// Consciousness contract address
    pub consciousness_contract: Option<String>,
    /// Quantum bridge contract address
    pub quantum_bridge_contract: Option<String>,
}

/// Global chain settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalChainSettings {
    /// Maximum number of simultaneous chain connections
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connection_timeout: u64,
    /// Retry attempts for failed operations
    pub retry_attempts: u32,
    /// Delay between retries in milliseconds
    pub retry_delay: u64,
}

/// Consciousness system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessConfig {
    /// Default starting energy for new tokens
    pub default_energy: u64,
    /// Maximum evolution energy
    pub max_energy: u64,
    /// Energy regeneration rate per hour
    pub energy_regen_rate: u64,
    /// Maximum traits per token
    pub max_traits: u32,
    /// Evolution success rate modifiers
    pub evolution_modifiers: EvolutionModifiers,
}

/// Evolution configuration modifiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionModifiers {
    /// Base success rate (0.0 to 1.0)
    pub base_success_rate: f64,
    /// Energy efficiency multiplier
    pub energy_efficiency: f64,
    /// Trait power multiplier
    pub trait_power_multiplier: f64,
}

/// Quantum system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumConfig {
    /// Maximum entanglements per token
    pub max_entanglements: u32,
    /// Maximum quantum channels
    pub max_channels: u32,
    /// Minimum coherence threshold
    pub min_coherence: f64,
    /// Default quantum energy
    pub default_quantum_energy: u64,
    /// Quantum operation costs
    pub operation_costs: QuantumOperationCosts,
}

/// Quantum operation cost configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumOperationCosts {
    /// Cost to create entanglement
    pub entanglement_cost: u64,
    /// Cost per quantum tunnel operation
    pub tunnel_cost: u64,
    /// Cost per synchronization
    pub sync_cost: u64,
    /// Cost to create quantum channel
    pub channel_cost: u64,
}

/// AI system configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    /// Enabled AI providers
    pub providers: Vec<AIProviderConfig>,
    /// Consensus configuration
    pub consensus: AIConsensusConfig,
    /// Contract generation settings
    pub contract_generation: ContractGenerationConfig,
}

/// AI provider configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProviderConfig {
    /// Provider name (claude, gpt, cohere)
    pub name: String,
    /// API endpoint
    pub endpoint: String,
    /// API key (should be loaded from environment)
    pub api_key_env: String,
    /// Provider weight in consensus
    pub weight: f64,
    /// Whether provider is enabled
    pub enabled: bool,
    /// Request timeout in seconds
    pub timeout: u64,
}

/// AI consensus configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConsensusConfig {
    /// Minimum providers required for consensus
    pub min_providers: u32,
    /// Consensus threshold (0.0 to 1.0)
    pub threshold: f64,
    /// Maximum time to wait for consensus in seconds
    pub timeout: u64,
}

/// Contract generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractGenerationConfig {
    /// Default contract template
    pub default_template: String,
    /// Maximum contract size in bytes
    pub max_contract_size: usize,
    /// Gas limit for contract deployment
    pub deployment_gas_limit: u64,
    /// Validation timeout in seconds
    pub validation_timeout: u64,
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    /// Log level (trace, debug, info, warn, error)
    pub level: String,
    /// Log format (json, pretty)
    pub format: String,
    /// Log file path (optional)
    pub file: Option<String>,
    /// Whether to log to console
    pub console: bool,
}

impl Default for NovaForgeConfig {
    fn default() -> Self {
        Self {
            chains: ChainConfig::default(),
            consciousness: ConsciousnessConfig::default(),
            quantum: QuantumConfig::default(),
            ai: AIConfig::default(),
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            default_chain: ChainId::Ethereum,
            chain_settings: HashMap::new(),
            global: GlobalChainSettings::default(),
        }
    }
}

impl Default for GlobalChainSettings {
    fn default() -> Self {
        Self {
            max_connections: 10,
            connection_timeout: 30,
            retry_attempts: 3,
            retry_delay: 1000,
        }
    }
}

impl Default for ConsciousnessConfig {
    fn default() -> Self {
        Self {
            default_energy: 100,
            max_energy: 10000,
            energy_regen_rate: 10,
            max_traits: 50,
            evolution_modifiers: EvolutionModifiers::default(),
        }
    }
}

impl Default for EvolutionModifiers {
    fn default() -> Self {
        Self {
            base_success_rate: 0.7,
            energy_efficiency: 1.0,
            trait_power_multiplier: 1.0,
        }
    }
}

impl Default for QuantumConfig {
    fn default() -> Self {
        Self {
            max_entanglements: 1000,
            max_channels: 10,
            min_coherence: 0.5,
            default_quantum_energy: 1000,
            operation_costs: QuantumOperationCosts::default(),
        }
    }
}

impl Default for QuantumOperationCosts {
    fn default() -> Self {
        Self {
            entanglement_cost: 50,
            tunnel_cost: 100,
            sync_cost: 25,
            channel_cost: 200,
        }
    }
}

impl Default for AIConfig {
    fn default() -> Self {
        Self {
            providers: vec![
                AIProviderConfig {
                    name: "claude".to_string(),
                    endpoint: "https://api.anthropic.com/v1".to_string(),
                    api_key_env: "CLAUDE_API_KEY".to_string(),
                    weight: 1.0,
                    enabled: true,
                    timeout: 30,
                },
                AIProviderConfig {
                    name: "gpt".to_string(),
                    endpoint: "https://api.openai.com/v1".to_string(),
                    api_key_env: "OPENAI_API_KEY".to_string(),
                    weight: 1.0,
                    enabled: true,
                    timeout: 30,
                },
            ],
            consensus: AIConsensusConfig::default(),
            contract_generation: ContractGenerationConfig::default(),
        }
    }
}

impl Default for AIConsensusConfig {
    fn default() -> Self {
        Self {
            min_providers: 2,
            threshold: 0.6,
            timeout: 60,
        }
    }
}

impl Default for ContractGenerationConfig {
    fn default() -> Self {
        Self {
            default_template: "consciousness_token".to_string(),
            max_contract_size: 1_000_000,
            deployment_gas_limit: 5_000_000,
            validation_timeout: 30,
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            format: "pretty".to_string(),
            file: None,
            console: true,
        }
    }
}

impl NovaForgeConfig {
    /// Load configuration from file
    pub fn from_file(path: &str) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)
            .map_err(|_| ConfigError::FileNotFound { path: path.to_string() })?;
        
        serde_json::from_str(&content)
            .map_err(|e| ConfigError::InvalidFormat { reason: e.to_string() })
    }

    /// Save configuration to file
    pub fn to_file(&self, path: &str) -> Result<(), ConfigError> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::InvalidFormat { reason: e.to_string() })?;
        
        std::fs::write(path, content)
            .map_err(|_| ConfigError::FileNotFound { path: path.to_string() })
    }

    /// Validate configuration
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validate chain configuration
        if self.chains.global.max_connections == 0 {
            return Err(ConfigError::InvalidValue {
                field: "chains.global.max_connections".to_string(),
                value: "0".to_string(),
            });
        }

        // Validate consciousness configuration
        if self.consciousness.default_energy > self.consciousness.max_energy {
            return Err(ConfigError::InvalidValue {
                field: "consciousness.default_energy".to_string(),
                value: self.consciousness.default_energy.to_string(),
            });
        }

        // Validate quantum configuration
        if self.quantum.min_coherence < 0.0 || self.quantum.min_coherence > 1.0 {
            return Err(ConfigError::InvalidValue {
                field: "quantum.min_coherence".to_string(),
                value: self.quantum.min_coherence.to_string(),
            });
        }

        // Validate AI configuration
        if self.ai.consensus.threshold < 0.0 || self.ai.consensus.threshold > 1.0 {
            return Err(ConfigError::InvalidValue {
                field: "ai.consensus.threshold".to_string(),
                value: self.ai.consensus.threshold.to_string(),
            });
        }

        Ok(())
    }

    /// Get chain settings for a specific chain
    pub fn get_chain_settings(&self, chain_id: ChainId) -> Option<&ChainSettings> {
        self.chains.chain_settings.get(&chain_id)
    }

    /// Add or update chain settings
    pub fn set_chain_settings(&mut self, chain_id: ChainId, settings: ChainSettings) {
        self.chains.chain_settings.insert(chain_id, settings);
    }

    /// Get enabled AI providers
    pub fn enabled_ai_providers(&self) -> Vec<&AIProviderConfig> {
        self.ai.providers.iter().filter(|p| p.enabled).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = NovaForgeConfig::default();
        assert_eq!(config.chains.default_chain, ChainId::Ethereum);
        assert_eq!(config.consciousness.default_energy, 100);
        assert_eq!(config.quantum.max_entanglements, 1000);
    }

    #[test]
    fn test_config_validation() {
        let config = NovaForgeConfig::default();
        assert!(config.validate().is_ok());
        
        let mut invalid_config = config;
        invalid_config.quantum.min_coherence = 1.5; // Invalid: > 1.0
        assert!(invalid_config.validate().is_err());
    }

    #[test]
    fn test_chain_settings() {
        let mut config = NovaForgeConfig::default();
        let settings = ChainSettings {
            rpc_url: "https://eth.example.com".to_string(),
            max_gas_price: 50,
            confirmations: 12,
            enabled: true,
            consciousness_contract: None,
            quantum_bridge_contract: None,
        };
        
        config.set_chain_settings(ChainId::Ethereum, settings);
        let retrieved = config.get_chain_settings(ChainId::Ethereum);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().max_gas_price, 50);
    }
}