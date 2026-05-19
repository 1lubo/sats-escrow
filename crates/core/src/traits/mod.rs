//! Trait definitions for external dependencies (ports)
//!
//! These traits define the interfaces for external systems.
//! Implementations (adapters) are provided in the `adapters` crate.

pub mod arbitration;
pub mod custodian;
pub mod identity;
pub mod payment;
pub mod repository;

pub use arbitration::ArbitrationStrategy;
pub use custodian::CustodianProvider;
pub use identity::IdentityProvider;
pub use payment::PaymentProcessor;
pub use repository::{DisputeRepository, EscrowRepository, UserRepository};
