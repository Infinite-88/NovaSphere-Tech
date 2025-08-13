//! AI consensus mechanism for contract validation
//!
//! This module provides consensus algorithms for validating and combining
//! results from multiple AI providers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::providers::{AIResponse, AIProviderType};

/// Consensus errors
#[derive(Error, Debug)]
pub enum ConsensusError {
    #[error("Insufficient responses for consensus: got {got}, need {required}")]
    InsufficientResponses { got: usize, required: usize },
    
    #[error("No consensus reached: agreement {agreement}%, threshold {threshold}%")]
    NoConsensus { agreement: f64, threshold: f64 },
    
    #[error("All responses failed validation")]
    AllResponsesInvalid,
    
    #[error("Consensus timeout exceeded")]
    Timeout,
    
    #[error("Invalid consensus configuration: {reason}")]
    InvalidConfiguration { reason: String },
}

/// Consensus result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    /// Result identifier
    pub id: Uuid,
    /// Consensus timestamp
    pub timestamp: DateTime<Utc>,
    /// Final contract code
    pub final_code: String,
    /// Consensus confidence score
    pub confidence: f64,
    /// Agreement percentage
    pub agreement: f64,
    /// Participating providers
    pub providers: Vec<AIProviderType>,
    /// Individual scores
    pub individual_scores: HashMap<AIProviderType, f64>,
    /// Consensus method used
    pub method: ConsensusMethod,
    /// Validation results
    pub validation: ValidationResult,
}

/// Consensus methods
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsensusMethod {
    /// Simple majority voting
    Majority,
    /// Weighted by provider confidence
    WeightedConfidence,
    /// Best response selection
    BestResponse,
    /// Hybrid approach combining multiple methods
    Hybrid,
    /// Custom consensus algorithm
    Custom(String),
}

/// Validation result for consensus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    /// Whether validation passed
    pub passed: bool,
    /// Validation score (0.0 to 1.0)
    pub score: f64,
    /// Validation details
    pub details: Vec<ValidationDetail>,
    /// Critical issues found
    pub critical_issues: Vec<String>,
    /// Warnings
    pub warnings: Vec<String>,
}

/// Individual validation detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDetail {
    /// Validation category
    pub category: String,
    /// Validation result
    pub passed: bool,
    /// Score for this validation
    pub score: f64,
    /// Description
    pub description: String,
}

/// Consensus configuration
#[derive(Debug, Clone)]
pub struct ConsensusConfig {
    /// Minimum number of responses required
    pub min_responses: usize,
    /// Consensus threshold (0.0 to 1.0)
    pub threshold: f64,
    /// Timeout for consensus in seconds
    pub timeout_seconds: u64,
    /// Consensus method to use
    pub method: ConsensusMethod,
    /// Provider weights for weighted consensus
    pub provider_weights: HashMap<AIProviderType, f64>,
    /// Enable code validation
    pub enable_validation: bool,
}

/// AI consensus engine
pub struct AIConsensus {
    /// Consensus configuration
    config: ConsensusConfig,
    /// Validation rules
    validators: Vec<Box<dyn CodeValidator>>,
    /// Consensus statistics
    stats: ConsensusStats,
}

/// Consensus statistics
#[derive(Debug, Clone, Default)]
pub struct ConsensusStats {
    /// Total consensus attempts
    pub total_attempts: u64,
    /// Successful consensus reached
    pub successful: u64,
    /// Failed consensus
    pub failed: u64,
    /// Average agreement percentage
    pub avg_agreement: f64,
    /// Average consensus time
    pub avg_consensus_time: f64,
}

/// Trait for code validators
pub trait CodeValidator: Send + Sync {
    /// Validate code and return score
    fn validate(&self, code: &str) -> ValidationDetail;
    
    /// Get validator name
    fn name(&self) -> &str;
}

impl AIConsensus {
    /// Create new AI consensus engine
    pub fn new(config: ConsensusConfig) -> Self {
        Self {
            config,
            validators: Self::create_default_validators(),
            stats: ConsensusStats::default(),
        }
    }

    /// Reach consensus on AI responses
    pub async fn reach_consensus(&mut self, responses: Vec<AIResponse>) -> Result<ConsensusResult, ConsensusError> {
        let start_time = std::time::Instant::now();
        self.stats.total_attempts += 1;

        // Validate minimum responses
        if responses.len() < self.config.min_responses {
            self.stats.failed += 1;
            return Err(ConsensusError::InsufficientResponses {
                got: responses.len(),
                required: self.config.min_responses,
            });
        }

        // Validate all responses if enabled
        let validated_responses = if self.config.enable_validation {
            self.validate_responses(&responses)?
        } else {
            responses
        };

        if validated_responses.is_empty() {
            self.stats.failed += 1;
            return Err(ConsensusError::AllResponsesInvalid);
        }

        // Apply consensus method
        let consensus_result = match &self.config.method {
            ConsensusMethod::Majority => self.majority_consensus(&validated_responses),
            ConsensusMethod::WeightedConfidence => self.weighted_confidence_consensus(&validated_responses),
            ConsensusMethod::BestResponse => self.best_response_consensus(&validated_responses),
            ConsensusMethod::Hybrid => self.hybrid_consensus(&validated_responses),
            ConsensusMethod::Custom(name) => self.custom_consensus(name, &validated_responses),
        }?;

        // Check if consensus threshold is met
        if consensus_result.agreement < self.config.threshold {
            self.stats.failed += 1;
            return Err(ConsensusError::NoConsensus {
                agreement: consensus_result.agreement * 100.0,
                threshold: self.config.threshold * 100.0,
            });
        }

        // Update statistics
        let consensus_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(consensus_result.agreement, consensus_time, true);

        Ok(consensus_result)
    }

    /// Validate responses using configured validators
    fn validate_responses(&self, responses: &[AIResponse]) -> Result<Vec<AIResponse>, ConsensusError> {
        let mut validated = Vec::new();

        for response in responses {
            let validation_result = self.validate_code(&response.content);
            
            // Accept responses that pass basic validation
            if validation_result.passed || validation_result.score > 0.6 {
                validated.push(response.clone());
            } else {
                tracing::warn!(
                    "Response from {:?} failed validation: score {:.2}",
                    response.provider,
                    validation_result.score
                );
            }
        }

        Ok(validated)
    }

    /// Validate code using all validators
    fn validate_code(&self, code: &str) -> ValidationResult {
        let mut details = Vec::new();
        let mut total_score = 0.0;
        let mut critical_issues = Vec::new();
        let mut warnings = Vec::new();

        for validator in &self.validators {
            let detail = validator.validate(code);
            total_score += detail.score;
            
            if !detail.passed && detail.score < 0.5 {
                critical_issues.push(format!("{}: {}", validator.name(), detail.description));
            } else if detail.score < 0.8 {
                warnings.push(format!("{}: {}", validator.name(), detail.description));
            }
            
            details.push(detail);
        }

        let avg_score = if self.validators.is_empty() {
            1.0
        } else {
            total_score / self.validators.len() as f64
        };

        ValidationResult {
            passed: avg_score > 0.7 && critical_issues.is_empty(),
            score: avg_score,
            details,
            critical_issues,
            warnings,
        }
    }

    /// Majority consensus - select most common response
    fn majority_consensus(&self, responses: &[AIResponse]) -> Result<ConsensusResult, ConsensusError> {
        // Group responses by similarity (simplified - would use more sophisticated comparison)
        let mut response_groups: HashMap<String, Vec<&AIResponse>> = HashMap::new();
        
        for response in responses {
            // Use first 100 characters as a simple similarity key
            let key = response.content.chars().take(100).collect::<String>();
            response_groups.entry(key).or_default().push(response);
        }

        // Find the largest group
        let (_, largest_group) = response_groups
            .into_iter()
            .max_by_key(|(_, group)| group.len())
            .ok_or(ConsensusError::AllResponsesInvalid)?;

        let agreement = largest_group.len() as f64 / responses.len() as f64;
        let representative = largest_group[0];

        Ok(ConsensusResult {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            final_code: representative.content.clone(),
            confidence: representative.confidence,
            agreement,
            providers: responses.iter().map(|r| r.provider.clone()).collect(),
            individual_scores: responses.iter()
                .map(|r| (r.provider.clone(), r.confidence))
                .collect(),
            method: ConsensusMethod::Majority,
            validation: self.validate_code(&representative.content),
        })
    }

    /// Weighted confidence consensus
    fn weighted_confidence_consensus(&self, responses: &[AIResponse]) -> Result<ConsensusResult, ConsensusError> {
        // Select response with highest weighted confidence
        let best_response = responses
            .iter()
            .max_by(|a, b| {
                let weight_a = self.config.provider_weights.get(&a.provider).unwrap_or(&1.0);
                let weight_b = self.config.provider_weights.get(&b.provider).unwrap_or(&1.0);
                
                let weighted_conf_a = a.confidence * weight_a;
                let weighted_conf_b = b.confidence * weight_b;
                
                weighted_conf_a.partial_cmp(&weighted_conf_b).unwrap()
            })
            .ok_or(ConsensusError::AllResponsesInvalid)?;

        // Calculate agreement based on confidence distribution
        let total_confidence: f64 = responses.iter()
            .map(|r| {
                let weight = self.config.provider_weights.get(&r.provider).unwrap_or(&1.0);
                r.confidence * weight
            })
            .sum();

        let best_weight = self.config.provider_weights.get(&best_response.provider).unwrap_or(&1.0);
        let agreement = (best_response.confidence * best_weight) / total_confidence;

        Ok(ConsensusResult {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            final_code: best_response.content.clone(),
            confidence: best_response.confidence,
            agreement,
            providers: responses.iter().map(|r| r.provider.clone()).collect(),
            individual_scores: responses.iter()
                .map(|r| (r.provider.clone(), r.confidence))
                .collect(),
            method: ConsensusMethod::WeightedConfidence,
            validation: self.validate_code(&best_response.content),
        })
    }

    /// Best response consensus - simply select highest confidence
    fn best_response_consensus(&self, responses: &[AIResponse]) -> Result<ConsensusResult, ConsensusError> {
        let best_response = responses
            .iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
            .ok_or(ConsensusError::AllResponsesInvalid)?;

        // Agreement is the ratio of best confidence to average confidence
        let avg_confidence: f64 = responses.iter().map(|r| r.confidence).sum::<f64>() / responses.len() as f64;
        let agreement = best_response.confidence / avg_confidence.max(0.1);

        Ok(ConsensusResult {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            final_code: best_response.content.clone(),
            confidence: best_response.confidence,
            agreement: agreement.min(1.0),
            providers: responses.iter().map(|r| r.provider.clone()).collect(),
            individual_scores: responses.iter()
                .map(|r| (r.provider.clone(), r.confidence))
                .collect(),
            method: ConsensusMethod::BestResponse,
            validation: self.validate_code(&best_response.content),
        })
    }

    /// Hybrid consensus combining multiple methods
    fn hybrid_consensus(&self, responses: &[AIResponse]) -> Result<ConsensusResult, ConsensusError> {
        // Try multiple methods and select the best result
        let majority_result = self.majority_consensus(responses)?;
        let confidence_result = self.weighted_confidence_consensus(responses)?;
        let best_result = self.best_response_consensus(responses)?;

        // Select the result with highest validation score
        let results = vec![majority_result, confidence_result, best_result];
        let best_hybrid = results
            .into_iter()
            .max_by(|a, b| a.validation.score.partial_cmp(&b.validation.score).unwrap())
            .ok_or(ConsensusError::AllResponsesInvalid)?;

        Ok(ConsensusResult {
            method: ConsensusMethod::Hybrid,
            ..best_hybrid
        })
    }

    /// Custom consensus method
    fn custom_consensus(&self, _method_name: &str, responses: &[AIResponse]) -> Result<ConsensusResult, ConsensusError> {
        // Fallback to best response for custom methods
        self.best_response_consensus(responses)
    }

    /// Update consensus statistics
    fn update_stats(&mut self, agreement: f64, consensus_time: f64, success: bool) {
        if success {
            self.stats.successful += 1;
        } else {
            self.stats.failed += 1;
        }

        // Update average agreement
        let total_successful = self.stats.successful;
        if total_successful > 1 {
            self.stats.avg_agreement = 
                (self.stats.avg_agreement * (total_successful - 1) as f64 + agreement) / total_successful as f64;
            self.stats.avg_consensus_time = 
                (self.stats.avg_consensus_time * (total_successful - 1) as f64 + consensus_time) / total_successful as f64;
        } else {
            self.stats.avg_agreement = agreement;
            self.stats.avg_consensus_time = consensus_time;
        }
    }

    /// Create default code validators
    fn create_default_validators() -> Vec<Box<dyn CodeValidator>> {
        vec![
            Box::new(SyntaxValidator),
            Box::new(SecurityValidator),
            Box::new(GasOptimizationValidator),
            Box::new(ConsciousnessFeatureValidator),
        ]
    }

    /// Get consensus statistics
    pub fn stats(&self) -> &ConsensusStats {
        &self.stats
    }

    /// Add custom validator
    pub fn add_validator(&mut self, validator: Box<dyn CodeValidator>) {
        self.validators.push(validator);
    }
}

/// Basic syntax validator
struct SyntaxValidator;

impl CodeValidator for SyntaxValidator {
    fn validate(&self, code: &str) -> ValidationDetail {
        let has_pragma = code.contains("pragma solidity");
        let has_contract = code.contains("contract ");
        let has_basic_structure = has_pragma && has_contract;

        ValidationDetail {
            category: "Syntax".to_string(),
            passed: has_basic_structure,
            score: if has_basic_structure { 1.0 } else { 0.0 },
            description: if has_basic_structure {
                "Valid Solidity syntax structure".to_string()
            } else {
                "Missing basic Solidity structure".to_string()
            },
        }
    }

    fn name(&self) -> &str {
        "SyntaxValidator"
    }
}

/// Security validator
struct SecurityValidator;

impl CodeValidator for SecurityValidator {
    fn validate(&self, code: &str) -> ValidationDetail {
        let mut score: f64 = 1.0;
        let mut issues = Vec::new();

        // Check for common security issues
        if code.contains("tx.origin") {
            score -= 0.3;
            issues.push("Uses tx.origin");
        }

        if code.contains("block.timestamp") && !code.contains("block.timestamp + ") {
            score -= 0.1;
            issues.push("Direct timestamp usage");
        }

        if !code.contains("require(") && !code.contains("revert(") {
            score -= 0.2;
            issues.push("No input validation");
        }

        ValidationDetail {
            category: "Security".to_string(),
            passed: score > 0.7,
            score: score.max(0.0),
            description: if issues.is_empty() {
                "No security issues found".to_string()
            } else {
                format!("Security issues: {}", issues.join(", "))
            },
        }
    }

    fn name(&self) -> &str {
        "SecurityValidator"
    }
}

/// Gas optimization validator
struct GasOptimizationValidator;

impl CodeValidator for GasOptimizationValidator {
    fn validate(&self, code: &str) -> ValidationDetail {
        let mut score: f64 = 1.0;
        let mut optimizations = Vec::new();

        // Check for gas optimizations
        if code.contains("string") && !code.contains("bytes") {
            score -= 0.1;
            optimizations.push("Could use bytes instead of string");
        }

        if code.contains("uint256") {
            score += 0.1; // Good practice
        }

        if code.contains("memory") || code.contains("calldata") {
            score += 0.1; // Explicit memory usage
        }

        ValidationDetail {
            category: "Gas Optimization".to_string(),
            passed: score > 0.8,
            score: score.min(1.0),
            description: if optimizations.is_empty() {
                "Good gas optimization practices".to_string()
            } else {
                format!("Optimization opportunities: {}", optimizations.join(", "))
            },
        }
    }

    fn name(&self) -> &str {
        "GasOptimizationValidator"
    }
}

/// Consciousness feature validator
struct ConsciousnessFeatureValidator;

impl CodeValidator for ConsciousnessFeatureValidator {
    fn validate(&self, code: &str) -> ValidationDetail {
        let has_consciousness = code.contains("consciousness") || code.contains("Consciousness");
        let has_evolution = code.contains("evolve") || code.contains("Evolution");
        let has_traits = code.contains("trait") || code.contains("Trait");

        let feature_count = [has_consciousness, has_evolution, has_traits]
            .iter()
            .filter(|&&x| x)
            .count();

        let score = feature_count as f64 / 3.0;

        ValidationDetail {
            category: "Consciousness Features".to_string(),
            passed: score > 0.5,
            score,
            description: format!("Consciousness features present: {}/3", feature_count),
        }
    }

    fn name(&self) -> &str {
        "ConsciousnessFeatureValidator"
    }
}

impl Default for ConsensusConfig {
    fn default() -> Self {
        Self {
            min_responses: 2,
            threshold: 0.6,
            timeout_seconds: 60,
            method: ConsensusMethod::Hybrid,
            provider_weights: HashMap::new(),
            enable_validation: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::providers::AIProviderType;

    #[tokio::test]
    async fn test_consensus_basic() {
        let config = ConsensusConfig::default();
        let mut consensus = AIConsensus::new(config);

        let responses = vec![
            AIResponse {
                id: Uuid::new_v4(),
                request_id: Uuid::new_v4(),
                content: "pragma solidity ^0.8.0; contract Test { }".to_string(),
                confidence: 0.9,
                provider: AIProviderType::Claude,
                metadata: crate::ai::providers::ResponseMetadata {
                    tokens_used: 100,
                    generation_time: 500,
                    model_version: "test".to_string(),
                    warnings: Vec::new(),
                },
            },
            AIResponse {
                id: Uuid::new_v4(),
                request_id: Uuid::new_v4(),
                content: "pragma solidity ^0.8.0; contract Test2 { }".to_string(),
                confidence: 0.8,
                provider: AIProviderType::GPT,
                metadata: crate::ai::providers::ResponseMetadata {
                    tokens_used: 120,
                    generation_time: 600,
                    model_version: "test".to_string(),
                    warnings: Vec::new(),
                },
            },
        ];

        let result = consensus.reach_consensus(responses).await;
        assert!(result.is_ok());
        
        let consensus_result = result.unwrap();
        assert!(consensus_result.final_code.contains("pragma solidity"));
        assert!(consensus_result.agreement > 0.0);
    }

    #[test]
    fn test_syntax_validator() {
        let validator = SyntaxValidator;
        
        let valid_code = "pragma solidity ^0.8.0; contract Test { }";
        let invalid_code = "invalid solidity code";
        
        let valid_result = validator.validate(valid_code);
        let invalid_result = validator.validate(invalid_code);
        
        assert!(valid_result.passed);
        assert_eq!(valid_result.score, 1.0);
        assert!(!invalid_result.passed);
        assert_eq!(invalid_result.score, 0.0);
    }

    #[test]
    fn test_security_validator() {
        let validator = SecurityValidator;
        
        let secure_code = "pragma solidity ^0.8.0; contract Test { function test() { require(msg.sender != address(0)); } }";
        let insecure_code = "pragma solidity ^0.8.0; contract Test { function test() { require(tx.origin == owner); } }";
        
        let secure_result = validator.validate(secure_code);
        let insecure_result = validator.validate(insecure_code);
        
        assert!(secure_result.score > insecure_result.score);
        assert!(insecure_result.description.contains("tx.origin"));
    }
}