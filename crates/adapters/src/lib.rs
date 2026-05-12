//! SatsEscrow Adapters - Implementations of core traits
//!
//! This crate provides concrete implementations for the traits defined in core:
//! - Mock implementations for testing and portfolio demonstration
//! - MongoDB repository implementations
//! - (Future) Real custodian integrations

pub mod mock;
pub mod mongodb;

// Re-export mock implementations
pub use mock::{
    MockCustodian,
    MockPaymentProcessor,
    MockArbitration,
    MockIdentityProvider,
    MockEscrowRepository,
    MockDisputeRepository,
    MockUserRepository,
};

// Re-export MongoDB implementations
pub use mongodb::{
    MongoClient,
    MongoEscrowRepository,
    MongoDisputeRepository,
    MongoUserRepository,
};
