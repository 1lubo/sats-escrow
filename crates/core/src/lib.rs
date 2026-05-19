//! SatsEscrow Core - Domain logic, types, and trait definitions
//!
//! This crate contains the pure business logic with no external dependencies.
//! All external concerns (custody, payments, arbitration, identity) are defined
//! as traits to be implemented by adapters.

pub mod dispute;
pub mod error;
pub mod escrow;
pub mod traits;
pub mod types;
pub mod user;

pub use dispute::{Dispute, DisputeState, Resolution, Vote};
pub use error::{Error, Result};
pub use escrow::{Escrow, EscrowEvent, EscrowState, EscrowTerms};
pub use types::*;
pub use user::{ReputationScore, User, UserId};
