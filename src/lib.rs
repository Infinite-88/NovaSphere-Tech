//! # NovaForge AI Empire - Clean Consciousness Evolution System
//!
//! This library provides a clean, well-structured implementation of consciousness token
//! evolution with quantum features for cross-chain operations.
//!
//! ## Features
//!
//! - **Consciousness Evolution**: Clean consciousness state management with simple evolution algorithms
//! - **Quantum Operations**: Basic quantum entanglement for cross-chain token synchronization
//! - **Multi-Chain Integration**: Unified chain manager for seamless operations
//! - **AI Contract Generation**: Multi-AI provider integration with consensus validation
//!
//! ## Quick Start
//!
//! ```rust
//! use novaforge_ai_empire::prelude::*;
//!
//! # fn main() -> Result<()> {
//! // Create a new consciousness token
//! let mut token = ConsciousnessToken::new();
//! 
//! // Add energy and evolve consciousness level
//! token.add_energy(50);
//! let evolved = token.evolve()?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Architecture
//!
//! The system is organized into several core modules:
//! - `consciousness` - Core consciousness evolution and quantum features
//! - `chains` - Multi-chain management and bridge operations
//! - `ai` - AI contract generation and consensus
//! - `utils` - Configuration and helper utilities

pub mod consciousness;
pub mod chains;
pub mod ai;
pub mod utils;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::consciousness::{
        ConsciousnessToken, ConsciousnessLevel, ConsciousnessTrait, EvolutionEvent,
        QuantumState, QuantumBridge, QuantumOperations,
    };
    pub use crate::chains::{
        ChainManager, ChainInterface, ChainId, ConsciousnessSynchronizer,
    };
    pub use crate::ai::{
        ContractGenerator, AIConsensus, AIProvider,
    };
    pub use crate::utils::{
        NovaForgeConfig, ConfigError, Result,
    };
}

/// Common result type used throughout the library
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;