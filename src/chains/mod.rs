//! Multi-chain integration and management
//!
//! This module provides unified chain management for seamless consciousness token
//! operations across multiple blockchain networks.

pub mod manager;
pub mod bridge;
pub mod protocols;

pub use manager::{ChainManager, ChainInterface, ChainError};
pub use bridge::{CrossChainBridge, BridgeOperation, BridgeError};
pub use protocols::{SynchronizationProtocol, ConsciousnessSynchronizer};

use serde::{Deserialize, Serialize};
use std::fmt;
use std::error::Error as StdError;

/// Supported blockchain identifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChainId {
    Ethereum,
    Polygon,
    Binance,
    Avalanche,
    Arbitrum,
    Optimism,
    Cosmos,
    Solana,
}

impl fmt::Display for ChainId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChainId::Ethereum => write!(f, "Ethereum"),
            ChainId::Polygon => write!(f, "Polygon"),
            ChainId::Binance => write!(f, "Binance Smart Chain"),
            ChainId::Avalanche => write!(f, "Avalanche"),
            ChainId::Arbitrum => write!(f, "Arbitrum"),
            ChainId::Optimism => write!(f, "Optimism"),
            ChainId::Cosmos => write!(f, "Cosmos"),
            ChainId::Solana => write!(f, "Solana"),
        }
    }
}

impl StdError for ChainId {}

impl ChainId {
    /// Get the native token symbol for the chain
    pub fn native_token(&self) -> &'static str {
        match self {
            ChainId::Ethereum => "ETH",
            ChainId::Polygon => "MATIC",
            ChainId::Binance => "BNB",
            ChainId::Avalanche => "AVAX",
            ChainId::Arbitrum => "ETH",
            ChainId::Optimism => "ETH",
            ChainId::Cosmos => "ATOM",
            ChainId::Solana => "SOL",
        }
    }

    /// Get the typical block time in seconds
    pub fn block_time(&self) -> u64 {
        match self {
            ChainId::Ethereum => 12,
            ChainId::Polygon => 2,
            ChainId::Binance => 3,
            ChainId::Avalanche => 2,
            ChainId::Arbitrum => 1,
            ChainId::Optimism => 2,
            ChainId::Cosmos => 6,
            ChainId::Solana => 1,
        }
    }

    /// Check if the chain supports quantum operations
    pub fn supports_quantum(&self) -> bool {
        match self {
            ChainId::Ethereum | ChainId::Polygon | ChainId::Avalanche => true,
            _ => false,
        }
    }

    /// Get all supported chains
    pub fn all() -> Vec<ChainId> {
        vec![
            ChainId::Ethereum,
            ChainId::Polygon,
            ChainId::Binance,
            ChainId::Avalanche,
            ChainId::Arbitrum,
            ChainId::Optimism,
            ChainId::Cosmos,
            ChainId::Solana,
        ]
    }
}

/// Chain state information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainState {
    /// Chain identifier
    pub chain_id: ChainId,
    /// Current block height
    pub block_height: u64,
    /// Chain health status
    pub health: ChainHealth,
    /// Number of deployed consciousness contracts
    pub consciousness_contracts: u32,
    /// Total consciousness tokens on chain
    pub total_tokens: u64,
    /// Active quantum bridges
    pub quantum_bridges: u32,
    /// Gas price information
    pub gas_info: GasInfo,
}

/// Chain health status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChainHealth {
    Healthy,
    Degraded,
    Unstable,
    Offline,
}

/// Gas price information for transactions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GasInfo {
    /// Standard gas price
    pub standard: u64,
    /// Fast gas price
    pub fast: u64,
    /// Instant gas price
    pub instant: u64,
    /// Gas price unit (gwei, lamports, etc.)
    pub unit: String,
}