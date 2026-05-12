//! Mock arbitration strategy

use std::collections::HashMap;
use std::sync::RwLock;

use sats_escrow_core::{
    dispute::{Dispute, Vote},
    traits::arbitration::{ArbitrationStrategy, BoxFuture},
    types::{DisputeId, Evidence},
    user::UserId,
    Result,
};

/// Mock arbitration that auto-assigns from a pool of arbitrators
pub struct MockArbitration {
    /// Pool of available arbitrators
    arbitrator_pool: RwLock<Vec<UserId>>,
    /// Votes stored by dispute ID
    votes: RwLock<HashMap<String, Vec<Vote>>>,
}

impl MockArbitration {
    pub fn new() -> Self {
        Self {
            arbitrator_pool: RwLock::new(Vec::new()),
            votes: RwLock::new(HashMap::new()),
        }
    }

    /// Add an arbitrator to the pool
    pub fn add_arbitrator(&self, user_id: UserId) {
        let mut pool = self.arbitrator_pool.write().unwrap();
        if !pool.contains(&user_id) {
            pool.push(user_id);
        }
    }

    /// Seed with some mock arbitrators
    pub fn seed_arbitrators(&self, count: usize) {
        let mut pool = self.arbitrator_pool.write().unwrap();
        for _ in 0..count {
            pool.push(UserId::new());
        }
    }
}

impl Default for MockArbitration {
    fn default() -> Self {
        let arb = Self::new();
        arb.seed_arbitrators(10); // Pre-seed with 10 mock arbitrators
        arb
    }
}

impl ArbitrationStrategy for MockArbitration {
    fn select_arbitrators(&self, _dispute: &Dispute, count: usize) -> BoxFuture<'_, Result<Vec<UserId>>> {
        Box::pin(async move {
            let pool = self.arbitrator_pool.read().unwrap();

            // Simple selection: take first N arbitrators (in real impl, would be random)
            let selected: Vec<UserId> = pool.iter().take(count).cloned().collect();

            if selected.len() < count {
                // Not enough arbitrators, but for mock we'll allow it
                tracing::warn!("Not enough arbitrators in pool: {} < {}", selected.len(), count);
            }

            Ok(selected)
        })
    }

    fn submit_vote(&self, dispute_id: &DisputeId, vote: Vote) -> BoxFuture<'_, Result<()>> {
        let dispute_id = dispute_id.clone();
        Box::pin(async move {
            let mut votes = self.votes.write().unwrap();
            votes
                .entry(dispute_id.0.to_string())
                .or_insert_with(Vec::new)
                .push(vote);
            Ok(())
        })
    }

    fn can_resolve(&self, dispute: &Dispute) -> BoxFuture<'_, Result<bool>> {
        let dispute_id = dispute.id.clone();
        Box::pin(async move {
            let votes = self.votes.read().unwrap();
            let dispute_votes = votes.get(&dispute_id.0.to_string());

            match dispute_votes {
                Some(v) => {
                    // For 3 arbitrators, need 2 votes for majority
                    let required = 2;
                    Ok(v.len() >= required)
                }
                None => Ok(false),
            }
        })
    }

    fn get_available_disputes(&self, _arbitrator: &UserId) -> BoxFuture<'_, Result<Vec<Dispute>>> {
        Box::pin(async move {
            // In real impl, would query repository
            // For mock, return empty (should be populated by service layer)
            Ok(Vec::new())
        })
    }

    fn validate_evidence(&self, evidence: &Evidence) -> BoxFuture<'_, Result<()>> {
        let evidence = evidence.clone();
        Box::pin(async move {
            // Basic validation: description must not be empty
            if evidence.description.is_empty() {
                return Err(sats_escrow_core::Error::Validation(
                    "Evidence description cannot be empty".to_string(),
                ));
            }
            Ok(())
        })
    }
}
