//! Arbitration strategy trait for dispute resolution

use std::future::Future;
use std::pin::Pin;

use crate::{
    dispute::{Dispute, Vote},
    error::Result,
    types::{DisputeId, Evidence},
    user::UserId,
};

/// Boxed future type for dyn compatibility
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Trait for arbitration strategies
///
/// Different implementations could include:
/// - 3-person jury (default)
/// - Single arbitrator
/// - AI-assisted resolution
/// - Professional tier with expert arbitrators
pub trait ArbitrationStrategy: Send + Sync {
    /// Select arbitrators for a dispute
    fn select_arbitrators(
        &self,
        dispute: &Dispute,
        count: usize,
    ) -> BoxFuture<'_, Result<Vec<UserId>>>;

    /// Submit a vote for a dispute
    fn submit_vote(&self, dispute_id: &DisputeId, vote: Vote) -> BoxFuture<'_, Result<()>>;

    /// Check if dispute can be resolved (enough votes)
    fn can_resolve(&self, dispute: &Dispute) -> BoxFuture<'_, Result<bool>>;

    /// Get disputes available for a specific arbitrator
    fn get_available_disputes(&self, arbitrator: &UserId) -> BoxFuture<'_, Result<Vec<Dispute>>>;

    /// Validate evidence format
    fn validate_evidence(&self, evidence: &Evidence) -> BoxFuture<'_, Result<()>>;
}
