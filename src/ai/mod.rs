//! AI contract generation and consensus
//!
//! This module provides multi-AI provider integration for contract generation
//! with consensus validation mechanisms.

pub mod generator;
pub mod consensus;
pub mod providers;

pub use generator::{ContractGenerator, GenerationRequest, GenerationResult};
pub use consensus::{AIConsensus, ConsensusResult, ConsensusError};
pub use providers::{AIProvider, AIProviderType, ProviderError};