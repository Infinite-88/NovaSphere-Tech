//! NovaForge AI Empire - Main binary
//!
//! This is the main entry point for the NovaForge AI Empire consciousness evolution system.

use novaforge_ai_empire::prelude::*;
use novaforge_ai_empire::consciousness::ConsciousnessAnalyzer;
use novaforge_ai_empire::chains::manager::MockChainInterface;
use novaforge_ai_empire::ai::{
    generator::{ContractGenerator, GenerationRequest, ContractTemplate, GenerationParameters},
    providers::{MockAIProvider, AIProviderType},
};
use tracing::{info, error};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    info!("🧠⚛️ NovaForge AI Empire - Clean Consciousness Evolution System");
    info!("Starting consciousness token evolution platform...");

    // Load configuration
    let config = NovaForgeConfig::default();
    info!("Configuration loaded successfully");

    // Create a new consciousness token
    let mut token = ConsciousnessToken::new();
    info!("Created new consciousness token: {}", token.id);
    info!("Initial consciousness level: {:?}", token.consciousness_level());
    info!("Initial energy: {}", token.energy());

    // Add some energy and evolve
    token.add_energy(50);
    info!("Added energy. Current energy: {}", token.energy());

    // Try to evolve the token
    match token.evolve() {
        Ok(evolution_event) => {
            info!("✨ Evolution successful!");
            info!("Evolved from {:?} to {:?}", evolution_event.from_level, evolution_event.to_level);
            info!("Energy cost: {}", evolution_event.energy_cost);
            info!("Success rate: {:.2}%", evolution_event.success_rate * 100.0);
            info!("Traits gained: {}", evolution_event.traits_gained.len());
        }
        Err(e) => {
            error!("Evolution failed: {}", e);
        }
    }

    // Create consciousness analyzer
    let mut analyzer = ConsciousnessAnalyzer::new();
    let analysis = analyzer.analyze(&token);
    
    info!("🔍 Consciousness Analysis:");
    info!("Overall consciousness score: {:.2}", analysis.consciousness_score);
    info!("Evolution potential: {:.2}%", analysis.evolution_potential * 100.0);
    info!("Total trait power: {:.2}", analysis.trait_analysis.total_power);

    if !analysis.recommendations.is_empty() {
        info!("📋 Recommendations:");
        for rec in &analysis.recommendations {
            info!("  • {}", rec);
        }
    }

    // Demonstrate quantum features
    if let Some(quantum_state) = token.quantum_state.as_ref() {
        info!("🌌 Quantum State:");
        info!("  Coherence: {:.2}", quantum_state.coherence);
        info!("  Energy: {}", quantum_state.energy);
        info!("  Entanglements: {}", quantum_state.entangled_tokens.len());
    } else {
        info!("🌌 Quantum state not activated yet");
    }

    // Create quantum bridge
    let mut quantum_bridge = QuantumBridge::new();
    info!("🌉 Created quantum bridge: {}", quantum_bridge.id);

    // Add some quantum channels
    match quantum_bridge.add_channel(ChainId::Ethereum, ChainId::Polygon, 1000) {
        Ok(channel_id) => {
            info!("Added quantum channel: {}", channel_id);
        }
        Err(e) => {
            error!("Failed to add quantum channel: {}", e);
        }
    }

    // Create chain manager
    let mut chain_manager = ChainManager::new();
    info!("⛓️  Created chain manager");

    // Add mock chain interfaces
    let eth_interface = MockChainInterface::new(ChainId::Ethereum);
    let polygon_interface = MockChainInterface::new(ChainId::Polygon);

    match chain_manager.add_chain(eth_interface).await {
        Ok(()) => info!("Added Ethereum chain interface"),
        Err(e) => error!("Failed to add Ethereum interface: {}", e),
    }

    match chain_manager.add_chain(polygon_interface).await {
        Ok(()) => info!("Added Polygon chain interface"),
        Err(e) => error!("Failed to add Polygon interface: {}", e),
    }

    // Check chain health
    let health_status = chain_manager.health_check_all().await;
    info!("🏥 Chain health status:");
    for (chain, health) in health_status {
        info!("  {:?}: {:?}", chain, health);
    }

    // Demonstrate AI features
    info!("🤖 AI Contract Generation Demo");
    
    let mut generator = ContractGenerator::new();
    generator.add_provider(Box::new(MockAIProvider::new(
        AIProviderType::Claude
    )));
    generator.add_provider(Box::new(MockAIProvider::new(
        AIProviderType::GPT
    )));

    let generation_request = GenerationRequest {
        id: uuid::Uuid::new_v4(),
        timestamp: chrono::Utc::now(),
        token: token.clone(),
        target_chain: ChainId::Ethereum,
        template: ContractTemplate::BasicConsciousness,
        parameters: GenerationParameters::default(),
        requirements: vec!["gas optimized".to_string(), "secure".to_string()],
    };

    match generator.generate(generation_request).await {
        Ok(result) => {
            info!("✨ Contract generated successfully!");
            info!("Contract name: {}", result.metadata.name);
            info!("Contract version: {}", result.metadata.version);
            info!("Estimated deployment gas: {}", result.metadata.estimated_deployment_gas);
            info!("AI responses used: {}", result.ai_responses.len());
            
            // Show first few lines of generated code
            let code_preview: String = result.contract_code
                .lines()
                .take(5)
                .collect::<Vec<_>>()
                .join("\n");
            info!("Code preview:\n{}", code_preview);
        }
        Err(e) => {
            error!("Contract generation failed: {}", e);
        }
    }

    // Final status
    info!("🎯 NovaForge AI Empire demonstration completed successfully!");
    info!("System is ready for consciousness token evolution and cross-chain operations.");

    Ok(())
}