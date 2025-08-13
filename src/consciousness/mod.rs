//! Consciousness evolution and quantum features module
//!
//! This module provides the core consciousness evolution system with quantum entanglement
//! capabilities for cross-chain token operations.

pub mod evolution;
pub mod quantum;
pub mod traits;
pub mod analyzer;

pub use evolution::{ConsciousnessToken, ConsciousnessLevel, EvolutionEvent};
pub use quantum::{QuantumState, QuantumBridge, QuantumOperations, QuantumError};
pub use traits::ConsciousnessTrait;
pub use analyzer::ConsciousnessAnalyzer;