//! Domain error types

use crate::{EscrowId, EscrowState, DisputeId};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Escrow not found: {0:?}")]
    EscrowNotFound(EscrowId),

    #[error("Dispute not found: {0:?}")]
    DisputeNotFound(DisputeId),

    #[error("Invalid state transition from {from:?} to {to:?}")]
    InvalidStateTransition {
        from: EscrowState,
        to: &'static str,
    },

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Escrow already has an active dispute")]
    DisputeAlreadyExists,

    #[error("Dispute is not in a votable state")]
    DisputeNotVotable,

    #[error("User has already voted on this dispute")]
    AlreadyVoted,

    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u64, available: u64 },

    #[error("Custodian error: {0}")]
    Custodian(String),

    #[error("Payment processor error: {0}")]
    PaymentProcessor(String),

    #[error("Identity provider error: {0}")]
    IdentityProvider(String),

    #[error("Repository error: {0}")]
    Repository(String),
}
