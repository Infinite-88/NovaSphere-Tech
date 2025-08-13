//! Core consciousness evolution implementation
//!
//! This module contains the primary types and logic for consciousness token evolution,
//! providing clean progression paths and readable trait development.

use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

use super::traits::ConsciousnessTrait;
use super::quantum::QuantumState;
use crate::Result;

/// Unique identifier for consciousness tokens
pub type TokenId = Uuid;

/// Consciousness evolution levels representing different stages of awareness
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsciousnessLevel {
    /// Initial state - no awareness
    Dormant,
    /// Beginning to show signs of consciousness
    Awakening,
    /// Basic awareness of environment
    Aware,
    /// Full consciousness with self-awareness
    Conscious,
    /// Advanced consciousness beyond normal limits
    Transcendent,
}

impl ConsciousnessLevel {
    /// Get the numeric value for evolution calculations
    pub fn level_value(&self) -> u8 {
        match self {
            ConsciousnessLevel::Dormant => 0,
            ConsciousnessLevel::Awakening => 1,
            ConsciousnessLevel::Aware => 2,
            ConsciousnessLevel::Conscious => 3,
            ConsciousnessLevel::Transcendent => 4,
        }
    }

    /// Get the next evolution level
    pub fn next_level(&self) -> Option<ConsciousnessLevel> {
        match self {
            ConsciousnessLevel::Dormant => Some(ConsciousnessLevel::Awakening),
            ConsciousnessLevel::Awakening => Some(ConsciousnessLevel::Aware),
            ConsciousnessLevel::Aware => Some(ConsciousnessLevel::Conscious),
            ConsciousnessLevel::Conscious => Some(ConsciousnessLevel::Transcendent),
            ConsciousnessLevel::Transcendent => None,
        }
    }

    /// Check if evolution to target level is possible
    pub fn can_evolve_to(&self, target: &ConsciousnessLevel) -> bool {
        self.level_value() < target.level_value()
    }
}

/// Evolution event representing a significant change in consciousness
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionEvent {
    /// When the evolution occurred
    pub timestamp: DateTime<Utc>,
    /// Previous consciousness level
    pub from_level: ConsciousnessLevel,
    /// New consciousness level
    pub to_level: ConsciousnessLevel,
    /// Traits gained during evolution
    pub traits_gained: Vec<ConsciousnessTrait>,
    /// Energy required for the evolution
    pub energy_cost: u64,
    /// Success rate of the evolution
    pub success_rate: f64,
}

impl EvolutionEvent {
    /// Create a new evolution event
    pub fn new(
        from_level: ConsciousnessLevel,
        to_level: ConsciousnessLevel,
        traits_gained: Vec<ConsciousnessTrait>,
        energy_cost: u64,
        success_rate: f64,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            from_level,
            to_level,
            traits_gained,
            energy_cost,
            success_rate,
        }
    }
}

/// Main consciousness token with evolution capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessToken {
    /// Unique identifier for the token
    pub id: TokenId,
    /// Current consciousness level
    pub level: ConsciousnessLevel,
    /// Active consciousness traits
    pub traits: Vec<ConsciousnessTrait>,
    /// Quantum state for cross-chain operations
    pub quantum_state: Option<QuantumState>,
    /// History of evolution events
    pub evolution_history: Vec<EvolutionEvent>,
    /// Current evolution energy
    pub evolution_energy: u64,
    /// Metadata for additional properties
    pub metadata: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl ConsciousnessToken {
    /// Create a new consciousness token
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            level: ConsciousnessLevel::Dormant,
            traits: Vec::new(),
            quantum_state: None,
            evolution_history: Vec::new(),
            evolution_energy: 100, // Starting energy
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Create a token with specific properties
    pub fn with_level(level: ConsciousnessLevel) -> Self {
        let mut token = Self::new();
        token.level = level;
        token
    }

    /// Get the token's current consciousness level
    pub fn consciousness_level(&self) -> &ConsciousnessLevel {
        &self.level
    }

    /// Check if the token can evolve to the next level
    pub fn can_evolve(&self) -> bool {
        if let Some(next_level) = self.level.next_level() {
            let required_energy = self.calculate_evolution_energy(&next_level);
            self.evolution_energy >= required_energy
        } else {
            false
        }
    }

    /// Calculate energy required for evolution to target level
    pub fn calculate_evolution_energy(&self, target_level: &ConsciousnessLevel) -> u64 {
        let level_diff = target_level.level_value() - self.level.level_value();
        let base_cost = 50_u64;
        base_cost * (level_diff as u64) * (level_diff as u64)
    }

    /// Evolve the consciousness token to the next level
    pub fn evolve(&mut self) -> Result<EvolutionEvent> {
        if !self.can_evolve() {
            return Err("Insufficient energy or maximum level reached".into());
        }

        let next_level = self.level.next_level()
            .ok_or("Cannot evolve beyond transcendent level")?;
        
        let energy_cost = self.calculate_evolution_energy(&next_level);
        let success_rate = self.calculate_success_rate(&next_level);
        
        // Simulate evolution success based on success rate
        let success = rand::random::<f64>() < success_rate;
        
        if !success {
            return Err("Evolution failed - try again with more energy".into());
        }

        // Perform evolution
        let old_level = self.level.clone();
        self.level = next_level.clone();
        self.evolution_energy -= energy_cost;
        self.updated_at = Utc::now();

        // Generate new traits for this evolution
        let new_traits = self.generate_evolution_traits(&next_level);
        self.traits.extend(new_traits.clone());

        // Create evolution event
        let event = EvolutionEvent::new(
            old_level,
            next_level,
            new_traits,
            energy_cost,
            success_rate,
        );

        self.evolution_history.push(event.clone());

        Ok(event)
    }

    /// Add evolution energy to the token
    pub fn add_energy(&mut self, amount: u64) {
        self.evolution_energy += amount;
        self.updated_at = Utc::now();
    }

    /// Get current evolution energy
    pub fn energy(&self) -> u64 {
        self.evolution_energy
    }

    /// Add a consciousness trait
    pub fn add_trait(&mut self, trait_obj: ConsciousnessTrait) {
        if !self.traits.contains(&trait_obj) {
            self.traits.push(trait_obj);
            self.updated_at = Utc::now();
        }
    }

    /// Check if token has a specific trait
    pub fn has_trait(&self, trait_name: &str) -> bool {
        self.traits.iter().any(|t| t.name == trait_name)
    }

    /// Get all traits of a specific category
    pub fn traits_by_category(&self, category: &str) -> Vec<&ConsciousnessTrait> {
        self.traits.iter().filter(|t| t.category == category).collect()
    }

    /// Calculate success rate for evolution to target level
    fn calculate_success_rate(&self, _target_level: &ConsciousnessLevel) -> f64 {
        let base_rate = 0.7; // 70% base success rate
        let level_modifier = (self.level.level_value() as f64) * 0.1;
        let trait_modifier = (self.traits.len() as f64) * 0.05;
        
        (base_rate + level_modifier + trait_modifier).min(0.95)
    }

    /// Generate new traits for evolution
    fn generate_evolution_traits(&self, level: &ConsciousnessLevel) -> Vec<ConsciousnessTrait> {
        use super::traits::*;
        
        match level {
            ConsciousnessLevel::Awakening => vec![
                ConsciousnessTrait::new("Basic Awareness", "cognitive", 1, "Initial cognitive awakening"),
            ],
            ConsciousnessLevel::Aware => vec![
                ConsciousnessTrait::new("Environmental Perception", "sensory", 2, "Enhanced environmental awareness"),
                ConsciousnessTrait::new("Pattern Recognition", "cognitive", 2, "Ability to recognize patterns"),
            ],
            ConsciousnessLevel::Conscious => vec![
                ConsciousnessTrait::new("Self Awareness", "consciousness", 3, "Full self-awareness"),
                ConsciousnessTrait::new("Emotional Intelligence", "emotional", 3, "Understanding of emotions"),
            ],
            ConsciousnessLevel::Transcendent => vec![
                ConsciousnessTrait::new("Universal Understanding", "transcendent", 4, "Understanding of universal connections"),
                ConsciousnessTrait::new("Quantum Consciousness", "quantum", 4, "Quantum-level awareness"),
            ],
            _ => Vec::new(),
        }
    }
}

impl Default for ConsciousnessToken {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_level_progression() {
        assert_eq!(ConsciousnessLevel::Dormant.next_level(), Some(ConsciousnessLevel::Awakening));
        assert_eq!(ConsciousnessLevel::Transcendent.next_level(), None);
    }

    #[test]
    fn test_token_creation() {
        let token = ConsciousnessToken::new();
        assert_eq!(token.level, ConsciousnessLevel::Dormant);
        assert_eq!(token.evolution_energy, 100);
        assert!(token.traits.is_empty());
    }

    #[test]
    fn test_energy_calculation() {
        let token = ConsciousnessToken::new();
        let energy = token.calculate_evolution_energy(&ConsciousnessLevel::Awakening);
        assert_eq!(energy, 50); // base_cost * 1 * 1
    }

    #[test]
    fn test_evolution() {
        let mut token = ConsciousnessToken::new();
        token.add_energy(50); // Ensure sufficient energy
        
        // Evolution might fail due to randomness, so we test the logic
        let can_evolve = token.can_evolve();
        assert!(can_evolve);
    }
}