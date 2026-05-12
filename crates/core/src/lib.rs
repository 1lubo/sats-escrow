//! SatsEscrow Core - Domain logic, types, and trait definitions
//!
//! This crate contains the pure business logic with no external dependencies.
//! All external concerns (custody, payments, arbitration, identity) are defined
//! as traits to be implemented by adapters.

pub mod error;
pub mod escrow;
pub mod dispute;
pub mod user;
pub mod traits;
pub mod types;

pub use error::{Error, Result};
pub use escrow::{Escrow, EscrowState, EscrowTerms, EscrowEvent};
pub use dispute::{Dispute, DisputeState, Vote, Resolution};
pub use user::{User, UserId, ReputationScore};
pub use types::*;
