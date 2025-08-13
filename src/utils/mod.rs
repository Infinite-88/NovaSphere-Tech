//! Utility modules for configuration and helpers
//!
//! This module provides configuration management and helper utilities
//! for the NovaForge AI Empire system.

pub mod config;
pub mod helpers;

pub use config::{NovaForgeConfig, ConfigError};
pub use helpers::*;

/// Common result type for utility operations
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;