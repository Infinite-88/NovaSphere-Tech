//! Synchronization protocols for consciousness tokens
//!
//! This module provides protocols for synchronizing consciousness state
//! across multiple blockchain networks.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use super::ChainId;
use super::manager::ConsciousnessState;
use crate::consciousness::evolution::TokenId;

/// Synchronization protocol types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SynchronizationProtocol {
    /// Simple state copying
    StateCopy,
    /// Consensus-based synchronization
    Consensus,
    /// Quantum entanglement sync
    QuantumSync,
    /// Event-driven synchronization
    EventDriven,
}

/// Synchronization event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncEvent {
    /// Event ID
    pub id: Uuid,
    /// Token being synchronized
    pub token_id: TokenId,
    /// Source chain
    pub source_chain: ChainId,
    /// Target chains
    pub target_chains: Vec<ChainId>,
    /// Protocol used
    pub protocol: SynchronizationProtocol,
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    /// Synchronization status
    pub status: SyncStatus,
}

/// Synchronization status
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SyncStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    PartiallyCompleted,
}

/// Consciousness synchronizer for multi-chain operations
pub struct ConsciousnessSynchronizer {
    /// Active synchronization events
    active_syncs: RwLock<HashMap<Uuid, SyncEvent>>,
    /// Synchronization statistics
    stats: RwLock<SyncStatistics>,
}

/// Synchronization statistics
#[derive(Debug, Clone, Default)]
pub struct SyncStatistics {
    /// Total sync events
    pub total_events: u64,
    /// Successful synchronizations
    pub successful: u64,
    /// Failed synchronizations
    pub failed: u64,
    /// Average sync time in milliseconds
    pub avg_sync_time: f64,
}

impl ConsciousnessSynchronizer {
    /// Create a new consciousness synchronizer
    pub fn new() -> Self {
        Self {
            active_syncs: RwLock::new(HashMap::new()),
            stats: RwLock::new(SyncStatistics::default()),
        }
    }

    /// Start synchronization of consciousness state
    pub async fn start_sync(
        &self,
        token_id: TokenId,
        source_chain: ChainId,
        target_chains: Vec<ChainId>,
        protocol: SynchronizationProtocol,
    ) -> Uuid {
        let event_id = Uuid::new_v4();
        let event = SyncEvent {
            id: event_id,
            token_id,
            source_chain,
            target_chains,
            protocol,
            timestamp: Utc::now(),
            status: SyncStatus::Pending,
        };

        let mut syncs = self.active_syncs.write().await;
        syncs.insert(event_id, event);

        // Update statistics
        let mut stats = self.stats.write().await;
        stats.total_events += 1;

        event_id
    }

    /// Update synchronization status
    pub async fn update_sync_status(&self, event_id: Uuid, status: SyncStatus) -> bool {
        let mut syncs = self.active_syncs.write().await;
        
        if let Some(event) = syncs.get_mut(&event_id) {
            event.status = status.clone();
            
            // Update statistics for completed events
            if matches!(status, SyncStatus::Completed | SyncStatus::Failed) {
                let mut stats = self.stats.write().await;
                match status {
                    SyncStatus::Completed => stats.successful += 1,
                    SyncStatus::Failed => stats.failed += 1,
                    _ => {}
                }
                
                // Calculate sync time
                let sync_time = Utc::now()
                    .signed_duration_since(event.timestamp)
                    .num_milliseconds() as f64;
                
                // Update average sync time
                let total_completed = stats.successful + stats.failed;
                if total_completed > 1 {
                    stats.avg_sync_time = (stats.avg_sync_time * (total_completed - 1) as f64 + sync_time) / total_completed as f64;
                } else {
                    stats.avg_sync_time = sync_time;
                }
            }
            
            true
        } else {
            false
        }
    }

    /// Get synchronization event
    pub async fn get_sync_event(&self, event_id: Uuid) -> Option<SyncEvent> {
        let syncs = self.active_syncs.read().await;
        syncs.get(&event_id).cloned()
    }

    /// Get all active synchronizations
    pub async fn get_active_syncs(&self) -> Vec<SyncEvent> {
        let syncs = self.active_syncs.read().await;
        syncs.values().cloned().collect()
    }

    /// Get synchronization statistics
    pub async fn get_statistics(&self) -> SyncStatistics {
        let stats = self.stats.read().await;
        stats.clone()
    }

    /// Synchronize consciousness states between chains
    pub async fn synchronize_states(
        &self,
        states: HashMap<ChainId, ConsciousnessState>,
    ) -> Result<ConsciousnessState, String> {
        if states.is_empty() {
            return Err("No states to synchronize".to_string());
        }

        // Find the most up-to-date state
        let latest_state = states
            .values()
            .max_by_key(|state| state.last_update)
            .unwrap();

        // In a real implementation, we would:
        // 1. Validate state consistency
        // 2. Resolve conflicts using consensus
        // 3. Apply quantum corrections if needed
        
        Ok(latest_state.clone())
    }

    /// Resolve state conflicts using different protocols
    pub async fn resolve_conflicts(
        &self,
        states: &HashMap<ChainId, ConsciousnessState>,
        protocol: &SynchronizationProtocol,
    ) -> Result<ConsciousnessState, String> {
        match protocol {
            SynchronizationProtocol::StateCopy => {
                // Simple: use the latest state
                self.resolve_by_timestamp(states)
            }
            SynchronizationProtocol::Consensus => {
                // Use majority consensus
                self.resolve_by_consensus(states)
            }
            SynchronizationProtocol::QuantumSync => {
                // Use quantum entanglement information
                self.resolve_by_quantum(states)
            }
            SynchronizationProtocol::EventDriven => {
                // Use event history
                self.resolve_by_events(states)
            }
        }
    }

    /// Resolve by timestamp (latest wins)
    fn resolve_by_timestamp(&self, states: &HashMap<ChainId, ConsciousnessState>) -> Result<ConsciousnessState, String> {
        states
            .values()
            .max_by_key(|state| state.last_update)
            .cloned()
            .ok_or_else(|| "No states available".to_string())
    }

    /// Resolve by consensus (majority rules)
    fn resolve_by_consensus(&self, states: &HashMap<ChainId, ConsciousnessState>) -> Result<ConsciousnessState, String> {
        // Group states by their hash
        let mut state_votes: HashMap<String, Vec<&ConsciousnessState>> = HashMap::new();
        
        for state in states.values() {
            state_votes.entry(state.state_hash.clone()).or_default().push(state);
        }

        // Find the state with the most votes
        let consensus_state = state_votes
            .into_iter()
            .max_by_key(|(_, votes)| votes.len())
            .and_then(|(_, votes)| votes.into_iter().next())
            .cloned()
            .ok_or_else(|| "No consensus reached".to_string())?;

        Ok(consensus_state)
    }

    /// Resolve using quantum information
    fn resolve_by_quantum(&self, states: &HashMap<ChainId, ConsciousnessState>) -> Result<ConsciousnessState, String> {
        // Prioritize states with quantum information
        let quantum_states: Vec<&ConsciousnessState> = states
            .values()
            .filter(|state| state.quantum_state.is_some())
            .collect();

        if !quantum_states.is_empty() {
            // Use the quantum state with highest coherence
            let best_quantum = quantum_states
                .into_iter()
                .max_by_key(|state| {
                    state.quantum_state.as_ref().map(|qs| qs.coherence).unwrap_or(0)
                })
                .cloned()
                .ok_or_else(|| "No quantum state found".to_string())?;
            
            Ok(best_quantum)
        } else {
            // Fall back to timestamp resolution
            self.resolve_by_timestamp(states)
        }
    }

    /// Resolve using event history
    fn resolve_by_events(&self, states: &HashMap<ChainId, ConsciousnessState>) -> Result<ConsciousnessState, String> {
        // For now, use timestamp as a proxy for event ordering
        // In a real implementation, we would track actual evolution events
        self.resolve_by_timestamp(states)
    }

    /// Clean up completed synchronization events
    pub async fn cleanup_completed_syncs(&self) {
        let mut syncs = self.active_syncs.write().await;
        
        syncs.retain(|_, event| {
            !matches!(event.status, SyncStatus::Completed | SyncStatus::Failed)
        });
    }

    /// Get sync success rate
    pub async fn success_rate(&self) -> f64 {
        let stats = self.stats.read().await;
        let total_completed = stats.successful + stats.failed;
        
        if total_completed == 0 {
            1.0
        } else {
            stats.successful as f64 / total_completed as f64
        }
    }
}

impl Default for ConsciousnessSynchronizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_synchronizer_creation() {
        let sync = ConsciousnessSynchronizer::new();
        let stats = sync.get_statistics().await;
        assert_eq!(stats.total_events, 0);
    }

    #[tokio::test]
    async fn test_start_sync() {
        let sync = ConsciousnessSynchronizer::new();
        let token_id = Uuid::new_v4();
        
        let event_id = sync.start_sync(
            token_id,
            ChainId::Ethereum,
            vec![ChainId::Polygon],
            SynchronizationProtocol::StateCopy,
        ).await;

        let event = sync.get_sync_event(event_id).await;
        assert!(event.is_some());
        assert_eq!(event.unwrap().token_id, token_id);
    }

    #[tokio::test]
    async fn test_update_status() {
        let sync = ConsciousnessSynchronizer::new();
        let token_id = Uuid::new_v4();
        
        let event_id = sync.start_sync(
            token_id,
            ChainId::Ethereum,
            vec![ChainId::Polygon],
            SynchronizationProtocol::StateCopy,
        ).await;

        let updated = sync.update_sync_status(event_id, SyncStatus::Completed).await;
        assert!(updated);

        let stats = sync.get_statistics().await;
        assert_eq!(stats.successful, 1);
    }

    #[test]
    fn test_timestamp_resolution() {
        let sync = ConsciousnessSynchronizer::new();
        let mut states = HashMap::new();
        
        let state1 = ConsciousnessState {
            token_id: Uuid::new_v4(),
            level: 1,
            trait_count: 2,
            energy: 100,
            quantum_state: None,
            last_update: 1000,
            state_hash: "hash1".to_string(),
        };
        
        let state2 = ConsciousnessState {
            token_id: Uuid::new_v4(),
            level: 2,
            trait_count: 3,
            energy: 150,
            quantum_state: None,
            last_update: 2000, // Later timestamp
            state_hash: "hash2".to_string(),
        };
        
        states.insert(ChainId::Ethereum, state1);
        states.insert(ChainId::Polygon, state2.clone());
        
        let result = sync.resolve_by_timestamp(&states).unwrap();
        assert_eq!(result.last_update, 2000);
        assert_eq!(result.level, 2);
    }
}