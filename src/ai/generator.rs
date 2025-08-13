//! Contract generation using multiple AI providers
//!
//! This module provides contract generation functionality with support
//! for multiple AI providers and template-based generation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::providers::{AIProvider, AIRequest, AIResponse, AIContext, AIParameters, ProviderError};
use crate::consciousness::evolution::ConsciousnessToken;
use crate::chains::ChainId;

/// Contract generation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    /// Request identifier
    pub id: Uuid,
    /// Request timestamp
    pub timestamp: DateTime<Utc>,
    /// Consciousness token to generate contract for
    pub token: ConsciousnessToken,
    /// Target blockchain
    pub target_chain: ChainId,
    /// Contract template to use
    pub template: ContractTemplate,
    /// Generation parameters
    pub parameters: GenerationParameters,
    /// Custom requirements
    pub requirements: Vec<String>,
}

/// Contract generation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    /// Result identifier
    pub id: Uuid,
    /// Request ID this responds to
    pub request_id: Uuid,
    /// Generated contract code
    pub contract_code: String,
    /// Contract metadata
    pub metadata: ContractMetadata,
    /// Generation status
    pub status: GenerationStatus,
    /// AI responses that contributed to this result
    pub ai_responses: Vec<AIResponse>,
    /// Generation timestamp
    pub generated_at: DateTime<Utc>,
}

/// Contract templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractTemplate {
    /// Basic consciousness token
    BasicConsciousness,
    /// Consciousness token with quantum features
    QuantumConsciousness,
    /// Governance token for consciousness
    GovernanceToken,
    /// Bridge contract for cross-chain operations
    CrossChainBridge,
    /// Custom template
    Custom(String),
}

/// Generation parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationParameters {
    /// Include quantum features
    pub include_quantum: bool,
    /// Include governance features
    pub include_governance: bool,
    /// Include upgrade mechanisms
    pub include_upgrades: bool,
    /// Optimization level (0-3)
    pub optimization_level: u8,
    /// Security level (0-3)
    pub security_level: u8,
    /// Gas optimization
    pub gas_optimization: bool,
}

/// Contract metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractMetadata {
    /// Contract name
    pub name: String,
    /// Contract version
    pub version: String,
    /// Solidity version
    pub solidity_version: String,
    /// Required imports
    pub imports: Vec<String>,
    /// Contract functions
    pub functions: Vec<FunctionInfo>,
    /// Contract events
    pub events: Vec<EventInfo>,
    /// Estimated gas for deployment
    pub estimated_deployment_gas: u64,
}

/// Function information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// Function name
    pub name: String,
    /// Function visibility
    pub visibility: String,
    /// Function parameters
    pub parameters: Vec<ParameterInfo>,
    /// Return type
    pub return_type: Option<String>,
    /// Estimated gas cost
    pub estimated_gas: u64,
}

/// Parameter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub param_type: String,
    /// Whether parameter is indexed (for events)
    pub indexed: bool,
}

/// Event information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventInfo {
    /// Event name
    pub name: String,
    /// Event parameters
    pub parameters: Vec<ParameterInfo>,
}

/// Generation status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GenerationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    RequiresConsensus,
}

/// Contract generator
pub struct ContractGenerator {
    /// Available AI providers
    providers: Vec<Box<dyn AIProvider>>,
    /// Template library
    templates: HashMap<String, String>,
    /// Generation statistics
    stats: GenerationStats,
}

/// Generation statistics
#[derive(Debug, Clone, Default)]
pub struct GenerationStats {
    /// Total requests processed
    pub total_requests: u64,
    /// Successful generations
    pub successful: u64,
    /// Failed generations
    pub failed: u64,
    /// Average generation time
    pub avg_generation_time: f64,
}

impl ContractGenerator {
    /// Create a new contract generator
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            templates: Self::load_default_templates(),
            stats: GenerationStats::default(),
        }
    }

    /// Add an AI provider
    pub fn add_provider(&mut self, provider: Box<dyn AIProvider>) {
        self.providers.push(provider);
    }

    /// Generate contract using AI providers
    pub async fn generate(&mut self, request: GenerationRequest) -> Result<GenerationResult, GenerationError> {
        let start_time = std::time::Instant::now();
        self.stats.total_requests += 1;

        // Prepare AI request
        let ai_request = self.prepare_ai_request(&request)?;
        
        // Get responses from all available providers
        let mut ai_responses = Vec::new();
        let mut errors = Vec::new();

        for provider in &self.providers {
            match provider.generate(ai_request.clone()).await {
                Ok(response) => ai_responses.push(response),
                Err(e) => {
                    tracing::warn!("Provider {:?} failed: {}", provider.provider_type(), e);
                    errors.push(e);
                }
            }
        }

        if ai_responses.is_empty() {
            self.stats.failed += 1;
            return Err(GenerationError::AllProvidersFailed { errors });
        }

        // Select best response or combine responses
        let contract_code = if ai_responses.len() == 1 {
            ai_responses[0].content.clone()
        } else {
            self.combine_responses(&ai_responses)?
        };

        // Generate metadata
        let metadata = self.generate_metadata(&contract_code, &request)?;

        // Update statistics
        let generation_time = start_time.elapsed().as_millis() as f64;
        self.update_stats(generation_time, true);

        Ok(GenerationResult {
            id: Uuid::new_v4(),
            request_id: request.id,
            contract_code,
            metadata,
            status: GenerationStatus::Completed,
            ai_responses,
            generated_at: Utc::now(),
        })
    }

    /// Prepare AI request from generation request
    fn prepare_ai_request(&self, request: &GenerationRequest) -> Result<AIRequest, GenerationError> {
        let template_prompt = self.get_template_prompt(&request.template)?;
        let consciousness_info = self.format_consciousness_info(&request.token);
        
        let prompt = format!(
            "{}\n\n\
             Consciousness Token Information:\n\
             {}\n\n\
             Target Chain: {:?}\n\
             Requirements: {}\n\n\
             Please generate a complete Solidity smart contract that implements the specified functionality.",
            template_prompt,
            consciousness_info,
            request.target_chain,
            request.requirements.join(", ")
        );

        Ok(AIRequest {
            id: Uuid::new_v4(),
            prompt,
            context: AIContext {
                contract_type: format!("{:?}", request.template),
                target_chain: request.target_chain,
                consciousness_level: Some(request.token.level.level_value()),
                quantum_features: if request.parameters.include_quantum {
                    vec!["entanglement".to_string(), "tunneling".to_string()]
                } else {
                    Vec::new()
                },
                metadata: HashMap::new(),
            },
            parameters: AIParameters::default(),
        })
    }

    /// Get template prompt for contract type
    fn get_template_prompt(&self, template: &ContractTemplate) -> Result<String, GenerationError> {
        let template_key = match template {
            ContractTemplate::BasicConsciousness => "basic_consciousness",
            ContractTemplate::QuantumConsciousness => "quantum_consciousness",
            ContractTemplate::GovernanceToken => "governance",
            ContractTemplate::CrossChainBridge => "bridge",
            ContractTemplate::Custom(name) => name,
        };

        self.templates.get(template_key)
            .cloned()
            .ok_or_else(|| GenerationError::TemplateNotFound {
                template: template_key.to_string(),
            })
    }

    /// Format consciousness token information for AI
    fn format_consciousness_info(&self, token: &ConsciousnessToken) -> String {
        format!(
            "- Token ID: {}\n\
             - Consciousness Level: {:?}\n\
             - Traits: {}\n\
             - Evolution Energy: {}\n\
             - Quantum State: {}",
            token.id,
            token.level,
            token.traits.len(),
            token.evolution_energy,
            if token.quantum_state.is_some() { "Enabled" } else { "Disabled" }
        )
    }

    /// Combine responses from multiple AI providers
    fn combine_responses(&self, responses: &[AIResponse]) -> Result<String, GenerationError> {
        // For now, select the response with highest confidence
        // In a more sophisticated implementation, we could merge features
        let best_response = responses
            .iter()
            .max_by(|a, b| a.confidence.partial_cmp(&b.confidence).unwrap())
            .ok_or(GenerationError::NoBestResponse)?;

        Ok(best_response.content.clone())
    }

    /// Generate contract metadata
    fn generate_metadata(&self, _contract_code: &str, request: &GenerationRequest) -> Result<ContractMetadata, GenerationError> {
        // Simple metadata extraction - in a real implementation, this would parse the Solidity code
        Ok(ContractMetadata {
            name: format!("ConsciousnessToken_{}", request.token.id),
            version: "1.0.0".to_string(),
            solidity_version: "^0.8.19".to_string(),
            imports: vec!["@openzeppelin/contracts/token/ERC721/ERC721.sol".to_string()],
            functions: vec![
                FunctionInfo {
                    name: "evolve".to_string(),
                    visibility: "external".to_string(),
                    parameters: vec![],
                    return_type: None,
                    estimated_gas: 50000,
                },
                FunctionInfo {
                    name: "mint".to_string(),
                    visibility: "external".to_string(),
                    parameters: vec![
                        ParameterInfo {
                            name: "to".to_string(),
                            param_type: "address".to_string(),
                            indexed: false,
                        }
                    ],
                    return_type: None,
                    estimated_gas: 80000,
                },
            ],
            events: vec![
                EventInfo {
                    name: "ConsciousnessEvolved".to_string(),
                    parameters: vec![
                        ParameterInfo {
                            name: "tokenId".to_string(),
                            param_type: "uint256".to_string(),
                            indexed: true,
                        },
                        ParameterInfo {
                            name: "newLevel".to_string(),
                            param_type: "uint8".to_string(),
                            indexed: false,
                        },
                    ],
                },
            ],
            estimated_deployment_gas: 2_000_000,
        })
    }

    /// Update generation statistics
    fn update_stats(&mut self, generation_time: f64, success: bool) {
        if success {
            self.stats.successful += 1;
        } else {
            self.stats.failed += 1;
        }

        // Update average generation time
        let total_completed = self.stats.successful + self.stats.failed;
        if total_completed > 1 {
            self.stats.avg_generation_time = 
                (self.stats.avg_generation_time * (total_completed - 1) as f64 + generation_time) 
                / total_completed as f64;
        } else {
            self.stats.avg_generation_time = generation_time;
        }
    }

    /// Load default contract templates
    fn load_default_templates() -> HashMap<String, String> {
        let mut templates = HashMap::new();
        
        templates.insert(
            "basic_consciousness".to_string(),
            "Generate a basic ERC721 consciousness token contract with:\n\
             - Consciousness levels (0-4)\n\
             - Evolution mechanism\n\
             - Trait storage\n\
             - Energy management".to_string(),
        );

        templates.insert(
            "quantum_consciousness".to_string(),
            "Generate a quantum-enabled consciousness token contract with:\n\
             - All basic consciousness features\n\
             - Quantum entanglement capabilities\n\
             - Cross-chain synchronization\n\
             - Quantum state management".to_string(),
        );

        templates.insert(
            "governance".to_string(),
            "Generate a governance token contract for consciousness tokens with:\n\
             - Voting mechanisms\n\
             - Proposal creation and execution\n\
             - Consciousness-weighted voting\n\
             - Timelock functionality".to_string(),
        );

        templates.insert(
            "bridge".to_string(),
            "Generate a cross-chain bridge contract with:\n\
             - Token locking/unlocking\n\
             - Cross-chain message verification\n\
             - Quantum entanglement support\n\
             - Security mechanisms".to_string(),
        );

        templates
    }

    /// Get generation statistics
    pub fn stats(&self) -> &GenerationStats {
        &self.stats
    }

    /// Add custom template
    pub fn add_template(&mut self, name: String, prompt: String) {
        self.templates.insert(name, prompt);
    }
}

/// Contract generation errors
#[derive(thiserror::Error, Debug)]
pub enum GenerationError {
    #[error("Template not found: {template}")]
    TemplateNotFound { template: String },
    
    #[error("All AI providers failed")]
    AllProvidersFailed { errors: Vec<ProviderError> },
    
    #[error("No best response found")]
    NoBestResponse,
    
    #[error("Metadata generation failed: {reason}")]
    MetadataFailed { reason: String },
    
    #[error("Invalid generation parameters: {reason}")]
    InvalidParameters { reason: String },
}

impl Default for ContractGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for GenerationParameters {
    fn default() -> Self {
        Self {
            include_quantum: false,
            include_governance: false,
            include_upgrades: true,
            optimization_level: 2,
            security_level: 2,
            gas_optimization: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::providers::{MockAIProvider, AIProviderType};
    use crate::consciousness::evolution::ConsciousnessToken;

    #[tokio::test]
    async fn test_contract_generation() {
        let mut generator = ContractGenerator::new();
        generator.add_provider(Box::new(MockAIProvider::new(AIProviderType::Claude)));

        let token = ConsciousnessToken::new();
        let request = GenerationRequest {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            token,
            target_chain: ChainId::Ethereum,
            template: ContractTemplate::BasicConsciousness,
            parameters: GenerationParameters::default(),
            requirements: vec!["gas optimized".to_string()],
        };

        let result = generator.generate(request).await;
        assert!(result.is_ok());
        
        let generation = result.unwrap();
        assert_eq!(generation.status, GenerationStatus::Completed);
        assert!(generation.contract_code.contains("pragma solidity"));
    }

    #[test]
    fn test_template_loading() {
        let generator = ContractGenerator::new();
        assert!(generator.templates.contains_key("basic_consciousness"));
        assert!(generator.templates.contains_key("quantum_consciousness"));
    }

    #[test]
    fn test_consciousness_info_formatting() {
        let generator = ContractGenerator::new();
        let token = ConsciousnessToken::new();
        let info = generator.format_consciousness_info(&token);
        
        assert!(info.contains(&token.id.to_string()));
        assert!(info.contains("Consciousness Level"));
        assert!(info.contains("Evolution Energy"));
    }
}