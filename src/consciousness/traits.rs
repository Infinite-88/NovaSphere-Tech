//! Consciousness traits system
//!
//! This module defines the trait system that consciousness tokens can develop
//! as they evolve, providing specific capabilities and characteristics.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A consciousness trait that provides specific capabilities
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsciousnessTrait {
    /// Name of the trait
    pub name: String,
    /// Category of the trait (cognitive, emotional, quantum, etc.)
    pub category: String,
    /// Power level of the trait (1-10)
    pub level: u8,
    /// Description of what the trait provides
    pub description: String,
    /// Prerequisites for acquiring this trait
    pub prerequisites: Vec<String>,
    /// Benefits provided by this trait
    pub benefits: Vec<String>,
}

impl ConsciousnessTrait {
    /// Create a new consciousness trait
    pub fn new(name: &str, category: &str, level: u8, description: &str) -> Self {
        Self {
            name: name.to_string(),
            category: category.to_string(),
            level: level.clamp(1, 10),
            description: description.to_string(),
            prerequisites: Vec::new(),
            benefits: Vec::new(),
        }
    }

    /// Create a trait with prerequisites
    pub fn with_prerequisites(mut self, prerequisites: Vec<&str>) -> Self {
        self.prerequisites = prerequisites.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Create a trait with benefits
    pub fn with_benefits(mut self, benefits: Vec<&str>) -> Self {
        self.benefits = benefits.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// Check if this trait is more powerful than another
    pub fn is_stronger_than(&self, other: &ConsciousnessTrait) -> bool {
        self.level > other.level
    }

    /// Check if traits are compatible (can coexist)
    pub fn is_compatible_with(&self, other: &ConsciousnessTrait) -> bool {
        // Traits in the same category with very different levels might conflict
        if self.category == other.category {
            (self.level as i8 - other.level as i8).abs() <= 3
        } else {
            true
        }
    }

    /// Calculate the power contribution of this trait
    pub fn power_contribution(&self) -> f64 {
        (self.level as f64) * self.category_multiplier()
    }

    /// Get multiplier based on trait category
    fn category_multiplier(&self) -> f64 {
        match self.category.as_str() {
            "quantum" => 2.0,
            "transcendent" => 1.8,
            "consciousness" => 1.5,
            "cognitive" => 1.2,
            "emotional" => 1.1,
            "sensory" => 1.0,
            _ => 1.0,
        }
    }
}

impl fmt::Display for ConsciousnessTrait {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (Level {}, {}): {}",
            self.name, self.level, self.category, self.description
        )
    }
}

/// Predefined consciousness traits for different evolution stages
pub mod predefined {
    use super::*;

    /// Cognitive traits for mental capabilities
    pub fn cognitive_traits() -> Vec<ConsciousnessTrait> {
        vec![
            ConsciousnessTrait::new(
                "Basic Awareness",
                "cognitive",
                1,
                "Initial cognitive awakening"
            ).with_benefits(vec!["Enhanced perception", "Basic pattern recognition"]),
            
            ConsciousnessTrait::new(
                "Pattern Recognition",
                "cognitive",
                2,
                "Ability to recognize complex patterns"
            ).with_prerequisites(vec!["Basic Awareness"])
            .with_benefits(vec!["Advanced analysis", "Predictive capabilities"]),
            
            ConsciousnessTrait::new(
                "Logical Reasoning",
                "cognitive",
                3,
                "Advanced logical processing capabilities"
            ).with_prerequisites(vec!["Pattern Recognition"])
            .with_benefits(vec!["Problem solving", "Deductive reasoning"]),
            
            ConsciousnessTrait::new(
                "Abstract Thinking",
                "cognitive",
                4,
                "Ability to process abstract concepts"
            ).with_prerequisites(vec!["Logical Reasoning"])
            .with_benefits(vec!["Conceptual understanding", "Creative thinking"]),
        ]
    }

    /// Emotional traits for emotional intelligence
    pub fn emotional_traits() -> Vec<ConsciousnessTrait> {
        vec![
            ConsciousnessTrait::new(
                "Emotional Recognition",
                "emotional",
                1,
                "Basic recognition of emotional states"
            ).with_benefits(vec!["Emotion detection", "Empathy basics"]),
            
            ConsciousnessTrait::new(
                "Emotional Intelligence",
                "emotional",
                3,
                "Advanced understanding and management of emotions"
            ).with_prerequisites(vec!["Emotional Recognition"])
            .with_benefits(vec!["Emotional regulation", "Social understanding"]),
            
            ConsciousnessTrait::new(
                "Compassion",
                "emotional",
                4,
                "Deep empathetic understanding and care for others"
            ).with_prerequisites(vec!["Emotional Intelligence"])
            .with_benefits(vec!["Healing abilities", "Harmonic resonance"]),
        ]
    }

    /// Quantum traits for quantum capabilities
    pub fn quantum_traits() -> Vec<ConsciousnessTrait> {
        vec![
            ConsciousnessTrait::new(
                "Quantum Sensitivity",
                "quantum",
                2,
                "Sensitivity to quantum fluctuations"
            ).with_benefits(vec!["Quantum state awareness", "Entanglement detection"]),
            
            ConsciousnessTrait::new(
                "Quantum Entanglement",
                "quantum",
                3,
                "Ability to create and maintain quantum entanglements"
            ).with_prerequisites(vec!["Quantum Sensitivity"])
            .with_benefits(vec!["Cross-chain communication", "Instant information transfer"]),
            
            ConsciousnessTrait::new(
                "Quantum Consciousness",
                "quantum",
                4,
                "Full quantum-level consciousness integration"
            ).with_prerequisites(vec!["Quantum Entanglement"])
            .with_benefits(vec!["Quantum tunneling", "Superposition states"]),
            
            ConsciousnessTrait::new(
                "Quantum Mastery",
                "quantum",
                5,
                "Complete mastery over quantum phenomena"
            ).with_prerequisites(vec!["Quantum Consciousness"])
            .with_benefits(vec!["Reality manipulation", "Quantum field control"]),
        ]
    }

    /// Transcendent traits for highest consciousness levels
    pub fn transcendent_traits() -> Vec<ConsciousnessTrait> {
        vec![
            ConsciousnessTrait::new(
                "Universal Understanding",
                "transcendent",
                4,
                "Understanding of universal connections and principles"
            ).with_benefits(vec!["Cosmic awareness", "Universal harmony"]),
            
            ConsciousnessTrait::new(
                "Dimensional Awareness",
                "transcendent",
                5,
                "Awareness of multiple dimensions and realities"
            ).with_prerequisites(vec!["Universal Understanding"])
            .with_benefits(vec!["Cross-dimensional perception", "Reality navigation"]),
            
            ConsciousnessTrait::new(
                "Omniscience",
                "transcendent",
                6,
                "Access to universal knowledge and information"
            ).with_prerequisites(vec!["Dimensional Awareness"])
            .with_benefits(vec!["Universal knowledge", "Information synthesis"]),
        ]
    }

    /// Get all predefined traits
    pub fn all_traits() -> Vec<ConsciousnessTrait> {
        let mut traits = Vec::new();
        traits.extend(cognitive_traits());
        traits.extend(emotional_traits());
        traits.extend(quantum_traits());
        traits.extend(transcendent_traits());
        traits
    }
}

/// Trait compatibility checker
pub struct TraitCompatibilityChecker;

impl TraitCompatibilityChecker {
    /// Check if a set of traits are compatible with each other
    pub fn check_compatibility(traits: &[ConsciousnessTrait]) -> bool {
        for i in 0..traits.len() {
            for j in (i + 1)..traits.len() {
                if !traits[i].is_compatible_with(&traits[j]) {
                    return false;
                }
            }
        }
        true
    }

    /// Calculate total power of a trait set
    pub fn calculate_total_power(traits: &[ConsciousnessTrait]) -> f64 {
        traits.iter().map(|t| t.power_contribution()).sum()
    }

    /// Find trait conflicts in a set
    pub fn find_conflicts(traits: &[ConsciousnessTrait]) -> Vec<(usize, usize)> {
        let mut conflicts = Vec::new();
        for i in 0..traits.len() {
            for j in (i + 1)..traits.len() {
                if !traits[i].is_compatible_with(&traits[j]) {
                    conflicts.push((i, j));
                }
            }
        }
        conflicts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trait_creation() {
        let trait_obj = ConsciousnessTrait::new("Test Trait", "cognitive", 3, "A test trait");
        assert_eq!(trait_obj.name, "Test Trait");
        assert_eq!(trait_obj.category, "cognitive");
        assert_eq!(trait_obj.level, 3);
    }

    #[test]
    fn test_trait_comparison() {
        let trait1 = ConsciousnessTrait::new("Weak", "cognitive", 2, "Weak trait");
        let trait2 = ConsciousnessTrait::new("Strong", "cognitive", 5, "Strong trait");
        
        assert!(trait2.is_stronger_than(&trait1));
        assert!(!trait1.is_stronger_than(&trait2));
    }

    #[test]
    fn test_trait_compatibility() {
        let trait1 = ConsciousnessTrait::new("Similar1", "cognitive", 3, "Similar level");
        let trait2 = ConsciousnessTrait::new("Similar2", "cognitive", 4, "Similar level");
        let trait3 = ConsciousnessTrait::new("Different", "emotional", 1, "Different category");
        
        assert!(trait1.is_compatible_with(&trait2));
        assert!(trait1.is_compatible_with(&trait3));
    }

    #[test]
    fn test_power_contribution() {
        let quantum_trait = ConsciousnessTrait::new("Quantum", "quantum", 3, "Quantum trait");
        let cognitive_trait = ConsciousnessTrait::new("Cognitive", "cognitive", 3, "Cognitive trait");
        
        assert!(quantum_trait.power_contribution() > cognitive_trait.power_contribution());
    }

    #[test]
    fn test_predefined_traits() {
        let cognitive = predefined::cognitive_traits();
        let quantum = predefined::quantum_traits();
        
        assert!(!cognitive.is_empty());
        assert!(!quantum.is_empty());
        
        // Check that quantum traits have higher power
        if let (Some(cog), Some(quant)) = (cognitive.first(), quantum.first()) {
            if cog.level == quant.level {
                assert!(quant.power_contribution() > cog.power_contribution());
            }
        }
    }
}