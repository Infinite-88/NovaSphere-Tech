//! AI provider interfaces and implementations
//!
//! This module defines the interfaces for different AI providers
//! and their implementations for contract generation.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use std::time::Duration;

/// AI provider types
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AIProviderType {
    Claude,
    GPT,
    Cohere,
    Custom(String),
}

/// Provider operation errors
#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("API request failed: {reason}")]
    RequestFailed { reason: String },
    
    #[error("Authentication failed")]
    AuthenticationFailed,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("Provider timeout")]
    Timeout,
    
    #[error("Invalid response format: {reason}")]
    InvalidResponse { reason: String },
    
    #[error("Provider unavailable: {provider:?}")]
    ProviderUnavailable { provider: AIProviderType },
}

/// AI request for contract generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    /// Request identifier
    pub id: uuid::Uuid,
    /// Prompt for AI
    pub prompt: String,
    /// Context information
    pub context: AIContext,
    /// Request parameters
    pub parameters: AIParameters,
}

/// Context information for AI requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIContext {
    /// Contract type being generated
    pub contract_type: String,
    /// Target blockchain
    pub target_chain: crate::chains::ChainId,
    /// Consciousness level requirements
    pub consciousness_level: Option<u8>,
    /// Quantum features required
    pub quantum_features: Vec<String>,
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// Parameters for AI generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIParameters {
    /// Maximum tokens to generate
    pub max_tokens: u32,
    /// Temperature for generation (0.0 to 1.0)
    pub temperature: f64,
    /// Top-p for nucleus sampling
    pub top_p: f64,
    /// Response timeout
    pub timeout: Duration,
}

/// AI response from provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    /// Response identifier
    pub id: uuid::Uuid,
    /// Request ID this responds to
    pub request_id: uuid::Uuid,
    /// Generated content
    pub content: String,
    /// Provider confidence score
    pub confidence: f64,
    /// Provider that generated this response
    pub provider: AIProviderType,
    /// Generation metadata
    pub metadata: ResponseMetadata,
}

/// Response metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    /// Tokens used in generation
    pub tokens_used: u32,
    /// Generation time in milliseconds
    pub generation_time: u64,
    /// Model version used
    pub model_version: String,
    /// Any warnings from the provider
    pub warnings: Vec<String>,
}

/// Trait for AI providers
#[async_trait]
pub trait AIProvider: Send + Sync {
    /// Get provider type
    fn provider_type(&self) -> AIProviderType;
    
    /// Generate response for request
    async fn generate(&self, request: AIRequest) -> Result<AIResponse, ProviderError>;
    
    /// Check if provider is available
    async fn health_check(&self) -> Result<(), ProviderError>;
    
    /// Get provider capabilities
    fn capabilities(&self) -> ProviderCapabilities;
}

/// Provider capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderCapabilities {
    /// Maximum context length
    pub max_context_length: u32,
    /// Supported model types
    pub supported_models: Vec<String>,
    /// Supported languages
    pub supported_languages: Vec<String>,
    /// Whether provider supports streaming
    pub supports_streaming: bool,
    /// Whether provider supports function calling
    pub supports_function_calling: bool,
}

impl Default for AIParameters {
    fn default() -> Self {
        Self {
            max_tokens: 2048,
            temperature: 0.7,
            top_p: 0.9,
            timeout: Duration::from_secs(30),
        }
    }
}

/// Mock AI provider for testing
pub struct MockAIProvider {
    provider_type: AIProviderType,
    should_fail: bool,
    response_delay: Duration,
}

impl MockAIProvider {
    pub fn new(provider_type: AIProviderType) -> Self {
        Self {
            provider_type,
            should_fail: false,
            response_delay: Duration::from_millis(100),
        }
    }

    pub fn with_failure(mut self, should_fail: bool) -> Self {
        self.should_fail = should_fail;
        self
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.response_delay = delay;
        self
    }
}

#[async_trait]
impl AIProvider for MockAIProvider {
    fn provider_type(&self) -> AIProviderType {
        self.provider_type.clone()
    }

    async fn generate(&self, request: AIRequest) -> Result<AIResponse, ProviderError> {
        // Simulate processing delay
        tokio::time::sleep(self.response_delay).await;

        if self.should_fail {
            return Err(ProviderError::RequestFailed {
                reason: "Mock failure".to_string(),
            });
        }

        let content = format!(
            "// Mock contract generated by {:?}\n\
             // Request ID: {}\n\
             // Contract Type: {}\n\
             pragma solidity ^0.8.0;\n\n\
             contract MockConsciousnessToken {{\n\
                 string public name = \"Mock Consciousness Token\";\n\
                 uint256 public consciousnessLevel = {};\n\
                 \n\
                 function evolve() public {{\n\
                     consciousnessLevel += 1;\n\
                 }}\n\
             }}",
            self.provider_type,
            request.id,
            request.context.contract_type,
            request.context.consciousness_level.unwrap_or(1)
        );

        Ok(AIResponse {
            id: uuid::Uuid::new_v4(),
            request_id: request.id,
            content,
            confidence: 0.85,
            provider: self.provider_type.clone(),
            metadata: ResponseMetadata {
                tokens_used: 150,
                generation_time: self.response_delay.as_millis() as u64,
                model_version: "mock-v1.0".to_string(),
                warnings: Vec::new(),
            },
        })
    }

    async fn health_check(&self) -> Result<(), ProviderError> {
        if self.should_fail {
            Err(ProviderError::ProviderUnavailable {
                provider: self.provider_type.clone(),
            })
        } else {
            Ok(())
        }
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            max_context_length: 4096,
            supported_models: vec!["mock-model".to_string()],
            supported_languages: vec!["solidity".to_string(), "rust".to_string()],
            supports_streaming: false,
            supports_function_calling: true,
        }
    }
}

/// Claude AI provider implementation
pub struct ClaudeProvider {
    api_key: String,
    endpoint: String,
    model: String,
}

impl ClaudeProvider {
    pub fn new(api_key: String, endpoint: String) -> Self {
        Self {
            api_key,
            endpoint,
            model: "claude-3-sonnet-20240229".to_string(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

#[async_trait]
impl AIProvider for ClaudeProvider {
    fn provider_type(&self) -> AIProviderType {
        AIProviderType::Claude
    }

    async fn generate(&self, request: AIRequest) -> Result<AIResponse, ProviderError> {
        // In a real implementation, this would make HTTP requests to Claude API
        // For now, we'll simulate the response
        tokio::time::sleep(Duration::from_millis(500)).await;

        let content = format!(
            "// Contract generated by Claude AI\n\
             // Request: {}\n\
             pragma solidity ^0.8.0;\n\n\
             contract ConsciousnessToken {{\n\
                 mapping(uint256 => uint256) public consciousnessLevels;\n\
                 mapping(uint256 => bool) public quantumEnabled;\n\
                 \n\
                 event ConsciousnessEvolved(uint256 tokenId, uint256 newLevel);\n\
                 \n\
                 function evolveConsciousness(uint256 tokenId) external {{\n\
                     consciousnessLevels[tokenId] += 1;\n\
                     emit ConsciousnessEvolved(tokenId, consciousnessLevels[tokenId]);\n\
                 }}\n\
                 \n\
                 function enableQuantumFeatures(uint256 tokenId) external {{\n\
                     quantumEnabled[tokenId] = true;\n\
                 }}\n\
             }}",
            request.context.contract_type
        );

        Ok(AIResponse {
            id: uuid::Uuid::new_v4(),
            request_id: request.id,
            content,
            confidence: 0.92,
            provider: AIProviderType::Claude,
            metadata: ResponseMetadata {
                tokens_used: 280,
                generation_time: 500,
                model_version: self.model.clone(),
                warnings: Vec::new(),
            },
        })
    }

    async fn health_check(&self) -> Result<(), ProviderError> {
        // In real implementation, make a test request to Claude API
        Ok(())
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            max_context_length: 200000,
            supported_models: vec![
                "claude-3-sonnet-20240229".to_string(),
                "claude-3-haiku-20240307".to_string(),
            ],
            supported_languages: vec![
                "solidity".to_string(),
                "rust".to_string(),
                "javascript".to_string(),
                "python".to_string(),
            ],
            supports_streaming: true,
            supports_function_calling: false,
        }
    }
}

/// GPT AI provider implementation
pub struct GPTProvider {
    api_key: String,
    endpoint: String,
    model: String,
}

impl GPTProvider {
    pub fn new(api_key: String, endpoint: String) -> Self {
        Self {
            api_key,
            endpoint,
            model: "gpt-4".to_string(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }
}

#[async_trait]
impl AIProvider for GPTProvider {
    fn provider_type(&self) -> AIProviderType {
        AIProviderType::GPT
    }

    async fn generate(&self, request: AIRequest) -> Result<AIResponse, ProviderError> {
        // Simulate GPT API call
        tokio::time::sleep(Duration::from_millis(800)).await;

        let content = format!(
            "// Smart contract generated by GPT-4\n\
             // Consciousness Token Implementation\n\
             pragma solidity ^0.8.19;\n\n\
             import \"@openzeppelin/contracts/token/ERC721/ERC721.sol\";\n\
             import \"@openzeppelin/contracts/access/Ownable.sol\";\n\n\
             contract ConsciousnessNFT is ERC721, Ownable {{\n\
                 struct ConsciousnessData {{\n\
                     uint8 level;\n\
                     uint32 traitCount;\n\
                     uint64 energy;\n\
                     bool quantumEnabled;\n\
                 }}\n\
                 \n\
                 mapping(uint256 => ConsciousnessData) public consciousness;\n\
                 uint256 private _tokenIdCounter;\n\
                 \n\
                 event ConsciousnessEvolved(uint256 indexed tokenId, uint8 newLevel);\n\
                 event QuantumActivated(uint256 indexed tokenId);\n\
                 \n\
                 constructor() ERC721(\"ConsciousnessToken\", \"CONS\") {{}}\n\
                 \n\
                 function mint(address to) external onlyOwner {{\n\
                     uint256 tokenId = _tokenIdCounter++;\n\
                     _safeMint(to, tokenId);\n\
                     consciousness[tokenId] = ConsciousnessData(1, 0, 100, false);\n\
                 }}\n\
                 \n\
                 function evolve(uint256 tokenId) external {{\n\
                     require(_exists(tokenId), \"Token does not exist\");\n\
                     require(ownerOf(tokenId) == msg.sender, \"Not token owner\");\n\
                     \n\
                     ConsciousnessData storage data = consciousness[tokenId];\n\
                     require(data.energy >= 50, \"Insufficient energy\");\n\
                     \n\
                     data.level += 1;\n\
                     data.energy -= 50;\n\
                     \n\
                     emit ConsciousnessEvolved(tokenId, data.level);\n\
                 }}\n\
             }}"
        );

        Ok(AIResponse {
            id: uuid::Uuid::new_v4(),
            request_id: request.id,
            content,
            confidence: 0.88,
            provider: AIProviderType::GPT,
            metadata: ResponseMetadata {
                tokens_used: 420,
                generation_time: 800,
                model_version: self.model.clone(),
                warnings: Vec::new(),
            },
        })
    }

    async fn health_check(&self) -> Result<(), ProviderError> {
        // In real implementation, make a test request to OpenAI API
        Ok(())
    }

    fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities {
            max_context_length: 128000,
            supported_models: vec![
                "gpt-4".to_string(),
                "gpt-4-turbo".to_string(),
                "gpt-3.5-turbo".to_string(),
            ],
            supported_languages: vec![
                "solidity".to_string(),
                "rust".to_string(),
                "javascript".to_string(),
                "python".to_string(),
                "go".to_string(),
            ],
            supports_streaming: true,
            supports_function_calling: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock_provider() {
        let provider = MockAIProvider::new(AIProviderType::Claude);
        
        let request = AIRequest {
            id: uuid::Uuid::new_v4(),
            prompt: "Generate a consciousness token contract".to_string(),
            context: AIContext {
                contract_type: "consciousness_token".to_string(),
                target_chain: crate::chains::ChainId::Ethereum,
                consciousness_level: Some(3),
                quantum_features: vec!["entanglement".to_string()],
                metadata: std::collections::HashMap::new(),
            },
            parameters: AIParameters::default(),
        };

        let response = provider.generate(request).await.unwrap();
        assert!(response.content.contains("pragma solidity"));
        assert_eq!(response.provider, AIProviderType::Claude);
    }

    #[tokio::test]
    async fn test_provider_health_check() {
        let provider = MockAIProvider::new(AIProviderType::GPT);
        assert!(provider.health_check().await.is_ok());

        let failing_provider = MockAIProvider::new(AIProviderType::Claude).with_failure(true);
        assert!(failing_provider.health_check().await.is_err());
    }

    #[test]
    fn test_provider_capabilities() {
        let provider = MockAIProvider::new(AIProviderType::Cohere);
        let capabilities = provider.capabilities();
        
        assert_eq!(capabilities.max_context_length, 4096);
        assert!(capabilities.supported_languages.contains(&"solidity".to_string()));
    }
}