//! Repository traits for data persistence

use std::pin::Pin;
use std::future::Future;

use crate::{
    dispute::Dispute,
    error::Result,
    escrow::Escrow,
    types::{DisputeId, EscrowId},
    user::{User, UserId},
};

/// Boxed future type for dyn compatibility
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Repository for escrow persistence
pub trait EscrowRepository: Send + Sync {
    /// Save a new escrow
    fn create(&self, escrow: &Escrow) -> BoxFuture<'_, Result<()>>;

    /// Update an existing escrow
    fn update(&self, escrow: &Escrow) -> BoxFuture<'_, Result<()>>;

    /// Find escrow by ID
    fn find_by_id(&self, id: &EscrowId) -> BoxFuture<'_, Result<Option<Escrow>>>;

    /// Find escrows by buyer
    fn find_by_buyer(&self, buyer: &UserId) -> BoxFuture<'_, Result<Vec<Escrow>>>;

    /// Find escrows by seller
    fn find_by_seller(&self, seller: &UserId) -> BoxFuture<'_, Result<Vec<Escrow>>>;

    /// Find all escrows for a user (as buyer or seller)
    fn find_by_user(&self, user: &UserId) -> BoxFuture<'_, Result<Vec<Escrow>>>;

    /// Find escrows that should auto-release
    fn find_pending_auto_release(&self) -> BoxFuture<'_, Result<Vec<Escrow>>>;
}

/// Repository for dispute persistence
pub trait DisputeRepository: Send + Sync {
    /// Save a new dispute
    fn create(&self, dispute: &Dispute) -> BoxFuture<'_, Result<()>>;

    /// Update an existing dispute
    fn update(&self, dispute: &Dispute) -> BoxFuture<'_, Result<()>>;

    /// Find dispute by ID
    fn find_by_id(&self, id: &DisputeId) -> BoxFuture<'_, Result<Option<Dispute>>>;

    /// Find dispute by escrow ID
    fn find_by_escrow(&self, escrow_id: &EscrowId) -> BoxFuture<'_, Result<Option<Dispute>>>;

    /// Find disputes awaiting arbitrators
    fn find_open_disputes(&self) -> BoxFuture<'_, Result<Vec<Dispute>>>;

    /// Find disputes assigned to an arbitrator
    fn find_by_arbitrator(&self, arbitrator: &UserId) -> BoxFuture<'_, Result<Vec<Dispute>>>;
}

/// Repository for user persistence
pub trait UserRepository: Send + Sync {
    /// Save a new user
    fn create(&self, user: &User) -> BoxFuture<'_, Result<()>>;

    /// Update an existing user
    fn update(&self, user: &User) -> BoxFuture<'_, Result<()>>;

    /// Find user by ID
    fn find_by_id(&self, id: &UserId) -> BoxFuture<'_, Result<Option<User>>>;

    /// Find arbitrators (users with arbitrator role)
    fn find_arbitrators(&self) -> BoxFuture<'_, Result<Vec<User>>>;
}
