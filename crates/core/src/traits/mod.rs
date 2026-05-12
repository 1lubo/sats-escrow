//! Trait definitions for external dependencies (ports)
//!
//! These traits define the interfaces for external systems.
//! Implementations (adapters) are provided in the `adapters` crate.

pub mod custodian;
pub mod payment;
pub mod arbitration;
pub mod identity;
pub mod repository;

pub use custodian::CustodianProvider;
pub use payment::PaymentProcessor;
pub use arbitration::ArbitrationStrategy;
pub use identity::IdentityProvider;
pub use repository::{EscrowRepository, DisputeRepository, UserRepository};
