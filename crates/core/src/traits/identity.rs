//! Identity provider trait for authentication and reputation

use std::pin::Pin;
use std::future::Future;

use crate::{
    error::Result,
    user::{ReputationScore, User, UserId},
};

/// Boxed future type for dyn compatibility
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Credentials for authentication
#[derive(Debug, Clone)]
pub enum Credentials {
    /// Username and password
    UsernamePassword { username: String, password: String },
    /// API key
    ApiKey(String),
    /// Nostr public key with signed challenge
    Nostr { pubkey: String, signature: String },
}

/// Session information after successful authentication
#[derive(Debug, Clone)]
pub struct Session {
    pub user_id: UserId,
    pub token: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

/// Trait for identity providers
///
/// Different implementations could include:
/// - Simple JWT-based auth
/// - Nostr-based decentralized identity
/// - OAuth providers
pub trait IdentityProvider: Send + Sync {
    /// Authenticate a user with credentials
    fn authenticate(&self, credentials: Credentials) -> BoxFuture<'_, Result<Session>>;

    /// Validate a session token
    fn validate_session(&self, token: &str) -> BoxFuture<'_, Result<UserId>>;

    /// Get user by ID
    fn get_user(&self, user_id: &UserId) -> BoxFuture<'_, Result<Option<User>>>;

    /// Get reputation score for a user
    fn get_reputation(&self, user_id: &UserId) -> BoxFuture<'_, Result<ReputationScore>>;

    /// Update reputation after a transaction
    fn update_reputation(&self, user_id: &UserId, score: ReputationScore) -> BoxFuture<'_, Result<()>>;

    /// Register a new user
    fn register(&self, display_name: String, credentials: Credentials) -> BoxFuture<'_, Result<User>>;
}
