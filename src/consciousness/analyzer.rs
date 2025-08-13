//! Consciousness analysis and monitoring
//!
//! This module provides analysis tools for consciousness tokens, including
//! evolution tracking, trait analysis, and consciousness metrics.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

use super::evolution::{ConsciousnessToken, ConsciousnessLevel};
use super::traits::ConsciousnessTrait;
use super::quantum::QuantumState;

/// Consciousness analysis report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsciousnessAnalysis {
    /// Token being analyzed
    pub token_id: uuid::Uuid,
    /// Analysis timestamp
    pub timestamp: DateTime<Utc>,
    /// Overall consciousness score
    pub consciousness_score: f64,
    /// Evolution potential
    pub evolution_potential: f64,
    /// Trait analysis
    pub trait_analysis: TraitAnalysis,
    /// Quantum state analysis
    pub quantum_analysis: Option<QuantumAnalysis>,
    /// Evolution recommendations
    pub recommendations: Vec<String>,
    /// Risk assessment
    pub risks: Vec<String>,
}

/// Trait analysis breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraitAnalysis {
    /// Total trait power
    pub total_power: f64,
    /// Trait distribution by category
    pub category_distribution: HashMap<String, u32>,
    /// Most powerful trait
    pub strongest_trait: Option<String>,
    /// Trait compatibility score
    pub compatibility_score: f64,
    /// Missing key traits
    pub missing_traits: Vec<String>,
}

/// Quantum state analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumAnalysis {
    /// Quantum coherence level
    pub coherence: f64,
    /// Entanglement count
    pub entanglement_count: usize,
    /// Quantum energy level
    pub energy_level: u64,
    /// Quantum stability score
    pub stability_score: f64,
    /// Quantum recommendations
    pub quantum_recommendations: Vec<String>,
}

/// Evolution trend analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionTrends {
    /// Evolution frequency (evolutions per time period)
    pub evolution_frequency: f64,
    /// Average evolution success rate
    pub success_rate: f64,
    /// Energy efficiency (evolution per energy unit)
    pub energy_efficiency: f64,
    /// Trait acquisition rate
    pub trait_acquisition_rate: f64,
    /// Predicted next evolution time
    pub next_evolution_prediction: Option<DateTime<Utc>>,
}

/// Consciousness analyzer for detailed token analysis
pub struct ConsciousnessAnalyzer {
    /// Analysis configuration
    config: AnalyzerConfig,
    /// Historical analysis cache
    analysis_cache: HashMap<uuid::Uuid, Vec<ConsciousnessAnalysis>>,
}

/// Analyzer configuration
#[derive(Debug, Clone)]
pub struct AnalyzerConfig {
    /// Weight for trait power in consciousness score
    pub trait_weight: f64,
    /// Weight for evolution level in consciousness score
    pub level_weight: f64,
    /// Weight for quantum state in consciousness score
    pub quantum_weight: f64,
    /// Minimum coherence for stable quantum state
    pub min_coherence: f64,
    /// Maximum analysis history to keep
    pub max_history: usize,
}

impl Default for AnalyzerConfig {
    fn default() -> Self {
        Self {
            trait_weight: 0.4,
            level_weight: 0.4,
            quantum_weight: 0.2,
            min_coherence: 0.7,
            max_history: 100,
        }
    }
}

impl ConsciousnessAnalyzer {
    /// Create a new analyzer with default configuration
    pub fn new() -> Self {
        Self {
            config: AnalyzerConfig::default(),
            analysis_cache: HashMap::new(),
        }
    }

    /// Create analyzer with custom configuration
    pub fn with_config(config: AnalyzerConfig) -> Self {
        Self {
            config,
            analysis_cache: HashMap::new(),
        }
    }

    /// Perform comprehensive analysis of a consciousness token
    pub fn analyze(&mut self, token: &ConsciousnessToken) -> ConsciousnessAnalysis {
        let trait_analysis = self.analyze_traits(&token.traits);
        let quantum_analysis = token.quantum_state.as_ref().map(|qs| self.analyze_quantum_state(qs));
        
        let consciousness_score = self.calculate_consciousness_score(
            &token.level,
            &trait_analysis,
            quantum_analysis.as_ref(),
        );
        
        let evolution_potential = self.calculate_evolution_potential(token);
        let recommendations = self.generate_recommendations(token, &trait_analysis);
        let risks = self.assess_risks(token);

        let analysis = ConsciousnessAnalysis {
            token_id: token.id,
            timestamp: Utc::now(),
            consciousness_score,
            evolution_potential,
            trait_analysis,
            quantum_analysis,
            recommendations,
            risks,
        };

        // Cache the analysis
        self.cache_analysis(token.id, analysis.clone());

        analysis
    }

    /// Analyze trait composition and power
    fn analyze_traits(&self, traits: &[ConsciousnessTrait]) -> TraitAnalysis {
        let total_power = traits.iter().map(|t| t.power_contribution()).sum();
        
        let mut category_distribution = HashMap::new();
        for trait_obj in traits {
            *category_distribution.entry(trait_obj.category.clone()).or_insert(0) += 1;
        }

        let strongest_trait = traits
            .iter()
            .max_by(|a, b| a.power_contribution().partial_cmp(&b.power_contribution()).unwrap())
            .map(|t| t.name.clone());

        let compatibility_score = self.calculate_trait_compatibility(traits);
        let missing_traits = self.identify_missing_traits(traits);

        TraitAnalysis {
            total_power,
            category_distribution,
            strongest_trait,
            compatibility_score,
            missing_traits,
        }
    }

    /// Analyze quantum state if present
    fn analyze_quantum_state(&self, quantum_state: &QuantumState) -> QuantumAnalysis {
        let stability_score = self.calculate_quantum_stability(quantum_state);
        let quantum_recommendations = self.generate_quantum_recommendations(quantum_state);

        QuantumAnalysis {
            coherence: quantum_state.coherence,
            entanglement_count: quantum_state.entangled_tokens.len(),
            energy_level: quantum_state.energy,
            stability_score,
            quantum_recommendations,
        }
    }

    /// Calculate overall consciousness score
    fn calculate_consciousness_score(
        &self,
        level: &ConsciousnessLevel,
        trait_analysis: &TraitAnalysis,
        quantum_analysis: Option<&QuantumAnalysis>,
    ) -> f64 {
        let level_score = (level.level_value() as f64) * 20.0; // 0-80 scale
        let trait_score = (trait_analysis.total_power / 10.0).min(20.0); // 0-20 scale
        let quantum_score = quantum_analysis
            .map(|qa| qa.coherence * qa.stability_score * 20.0)
            .unwrap_or(0.0); // 0-20 scale

        (level_score * self.config.level_weight)
            + (trait_score * self.config.trait_weight)
            + (quantum_score * self.config.quantum_weight)
    }

    /// Calculate evolution potential
    fn calculate_evolution_potential(&self, token: &ConsciousnessToken) -> f64 {
        let energy_factor = (token.evolution_energy as f64) / 1000.0;
        let trait_factor = (token.traits.len() as f64) / 10.0;
        let level_factor = match token.level {
            ConsciousnessLevel::Dormant => 1.0,
            ConsciousnessLevel::Awakening => 0.8,
            ConsciousnessLevel::Aware => 0.6,
            ConsciousnessLevel::Conscious => 0.4,
            ConsciousnessLevel::Transcendent => 0.0,
        };

        (energy_factor + trait_factor + level_factor).min(1.0)
    }

    /// Calculate trait compatibility score
    fn calculate_trait_compatibility(&self, traits: &[ConsciousnessTrait]) -> f64 {
        if traits.len() <= 1 {
            return 1.0;
        }

        let mut compatible_pairs = 0;
        let mut total_pairs = 0;

        for i in 0..traits.len() {
            for j in (i + 1)..traits.len() {
                total_pairs += 1;
                if traits[i].is_compatible_with(&traits[j]) {
                    compatible_pairs += 1;
                }
            }
        }

        if total_pairs == 0 {
            1.0
        } else {
            compatible_pairs as f64 / total_pairs as f64
        }
    }

    /// Identify missing key traits for current level
    fn identify_missing_traits(&self, current_traits: &[ConsciousnessTrait]) -> Vec<String> {
        let mut missing = Vec::new();
        let _current_trait_names: Vec<&str> = current_traits.iter().map(|t| t.name.as_str()).collect();

        // Check for essential traits by category
        let has_cognitive = current_traits.iter().any(|t| t.category == "cognitive");
        let has_emotional = current_traits.iter().any(|t| t.category == "emotional");
        let has_quantum = current_traits.iter().any(|t| t.category == "quantum");

        if !has_cognitive {
            missing.push("Basic cognitive trait required".to_string());
        }
        if current_traits.len() > 2 && !has_emotional {
            missing.push("Emotional intelligence recommended".to_string());
        }
        if current_traits.len() > 5 && !has_quantum {
            missing.push("Quantum awareness for advanced evolution".to_string());
        }

        missing
    }

    /// Calculate quantum state stability
    fn calculate_quantum_stability(&self, quantum_state: &QuantumState) -> f64 {
        let coherence_factor = quantum_state.coherence;
        let energy_factor = (quantum_state.energy as f64 / 1000.0).min(1.0);
        let entanglement_factor = if quantum_state.entangled_tokens.is_empty() {
            0.5
        } else {
            1.0
        };

        (coherence_factor + energy_factor + entanglement_factor) / 3.0
    }

    /// Generate recommendations for consciousness improvement
    fn generate_recommendations(&self, token: &ConsciousnessToken, trait_analysis: &TraitAnalysis) -> Vec<String> {
        let mut recommendations = Vec::new();

        // Energy recommendations
        if token.evolution_energy < 100 {
            recommendations.push("Increase evolution energy through activities".to_string());
        }

        // Trait recommendations
        if trait_analysis.total_power < 10.0 {
            recommendations.push("Focus on acquiring more powerful traits".to_string());
        }

        if trait_analysis.compatibility_score < 0.8 {
            recommendations.push("Consider removing conflicting traits".to_string());
        }

        // Evolution recommendations
        if token.can_evolve() {
            recommendations.push("Ready for evolution to next consciousness level".to_string());
        }

        // Quantum recommendations
        if token.quantum_state.is_none() && token.level.level_value() >= 2 {
            recommendations.push("Consider quantum state activation for advanced features".to_string());
        }

        recommendations
    }

    /// Generate quantum-specific recommendations
    fn generate_quantum_recommendations(&self, quantum_state: &QuantumState) -> Vec<String> {
        let mut recommendations = Vec::new();

        if quantum_state.coherence < self.config.min_coherence {
            recommendations.push("Improve quantum coherence through stabilization".to_string());
        }

        if quantum_state.energy < 500 {
            recommendations.push("Increase quantum energy for better operations".to_string());
        }

        if quantum_state.entangled_tokens.is_empty() {
            recommendations.push("Create quantum entanglements for cross-chain capabilities".to_string());
        }

        recommendations
    }

    /// Assess potential risks
    fn assess_risks(&self, token: &ConsciousnessToken) -> Vec<String> {
        let mut risks = Vec::new();

        // Energy risks
        if token.evolution_energy < 50 {
            risks.push("Low energy - evolution failure risk".to_string());
        }

        // Quantum risks
        if let Some(quantum_state) = &token.quantum_state {
            if quantum_state.coherence < 0.3 {
                risks.push("Quantum decoherence risk - may lose quantum capabilities".to_string());
            }
            if quantum_state.entangled_tokens.len() > 10 {
                risks.push("High entanglement count may cause instability".to_string());
            }
        }

        // Trait risks
        if token.traits.len() > 20 {
            risks.push("Too many traits may cause conflicts".to_string());
        }

        risks
    }

    /// Analyze evolution trends over time
    pub fn analyze_evolution_trends(&self, token: &ConsciousnessToken) -> EvolutionTrends {
        let events = &token.evolution_history;
        
        let evolution_frequency = if events.len() > 1 {
            let time_span = events.last().unwrap().timestamp
                .signed_duration_since(events.first().unwrap().timestamp);
            (events.len() as f64) / (time_span.num_days() as f64).max(1.0)
        } else {
            0.0
        };

        let success_rate = if !events.is_empty() {
            1.0 // All events in history are successful (failed ones aren't recorded)
        } else {
            0.0
        };

        let energy_efficiency = if !events.is_empty() {
            let total_energy: u64 = events.iter().map(|e| e.energy_cost).sum();
            (events.len() as f64) / (total_energy as f64)
        } else {
            0.0
        };

        let trait_acquisition_rate = if !events.is_empty() {
            let total_traits: usize = events.iter().map(|e| e.traits_gained.len()).sum();
            (total_traits as f64) / (events.len() as f64)
        } else {
            0.0
        };

        // Simple prediction based on average evolution time
        let next_evolution_prediction = if events.len() > 1 && token.can_evolve() {
            let avg_time_between = events.last().unwrap().timestamp
                .signed_duration_since(events.first().unwrap().timestamp)
                / (events.len() as i32 - 1);
            Some(Utc::now() + avg_time_between)
        } else {
            None
        };

        EvolutionTrends {
            evolution_frequency,
            success_rate,
            energy_efficiency,
            trait_acquisition_rate,
            next_evolution_prediction,
        }
    }

    /// Cache analysis result
    fn cache_analysis(&mut self, token_id: uuid::Uuid, analysis: ConsciousnessAnalysis) {
        let history = self.analysis_cache.entry(token_id).or_insert_with(Vec::new);
        history.push(analysis);
        
        // Keep only recent analyses
        if history.len() > self.config.max_history {
            history.remove(0);
        }
    }

    /// Get analysis history for a token
    pub fn get_analysis_history(&self, token_id: uuid::Uuid) -> Option<&Vec<ConsciousnessAnalysis>> {
        self.analysis_cache.get(&token_id)
    }

    /// Generate summary report
    pub fn generate_summary_report(&self, analyses: &[ConsciousnessAnalysis]) -> String {
        if analyses.is_empty() {
            return "No analyses available".to_string();
        }

        let avg_consciousness = analyses.iter()
            .map(|a| a.consciousness_score)
            .sum::<f64>() / analyses.len() as f64;

        let avg_evolution_potential = analyses.iter()
            .map(|a| a.evolution_potential)
            .sum::<f64>() / analyses.len() as f64;

        format!(
            "Analysis Summary:\n\
             - Average Consciousness Score: {:.2}\n\
             - Average Evolution Potential: {:.2}\n\
             - Total Analyses: {}\n\
             - Latest Analysis: {}",
            avg_consciousness,
            avg_evolution_potential,
            analyses.len(),
            analyses.last().unwrap().timestamp.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

impl Default for ConsciousnessAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::consciousness::evolution::ConsciousnessToken;

    #[test]
    fn test_analyzer_creation() {
        let analyzer = ConsciousnessAnalyzer::new();
        assert_eq!(analyzer.config.trait_weight, 0.4);
        assert_eq!(analyzer.config.level_weight, 0.4);
        assert_eq!(analyzer.config.quantum_weight, 0.2);
    }

    #[test]
    fn test_basic_analysis() {
        let mut analyzer = ConsciousnessAnalyzer::new();
        let token = ConsciousnessToken::new();
        
        let analysis = analyzer.analyze(&token);
        assert_eq!(analysis.token_id, token.id);
        assert!(analysis.consciousness_score >= 0.0);
        assert!(analysis.evolution_potential >= 0.0);
    }

    #[test]
    fn test_trait_analysis() {
        let analyzer = ConsciousnessAnalyzer::new();
        let traits = vec![
            crate::consciousness::traits::ConsciousnessTrait::new(
                "Test Trait", "cognitive", 3, "A test trait"
            ),
        ];
        
        let analysis = analyzer.analyze_traits(&traits);
        assert!(analysis.total_power > 0.0);
        assert_eq!(analysis.category_distribution.get("cognitive"), Some(&1));
    }

    #[test]
    fn test_evolution_trends() {
        let analyzer = ConsciousnessAnalyzer::new();
        let token = ConsciousnessToken::new(); // Empty history
        
        let trends = analyzer.analyze_evolution_trends(&token);
        assert_eq!(trends.evolution_frequency, 0.0);
        assert_eq!(trends.success_rate, 0.0);
    }
}