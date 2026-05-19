//! Application state for dependency injection

use std::sync::Arc;

use sats_escrow_core::traits::{
    ArbitrationStrategy, CustodianProvider, DisputeRepository, EscrowRepository, IdentityProvider,
    PaymentProcessor, UserRepository,
};

/// Application state containing all dependencies
#[derive(Clone)]
pub struct AppState {
    pub services: Arc<Services>,
}

impl AppState {
    pub fn new(services: Services) -> Self {
        Self {
            services: Arc::new(services),
        }
    }
}

/// Container for all service dependencies using trait objects
pub struct Services {
    pub custodian: Arc<dyn CustodianProvider>,
    pub payment: Arc<dyn PaymentProcessor>,
    pub arbitration: Arc<dyn ArbitrationStrategy>,
    pub identity: Arc<dyn IdentityProvider>,
    pub escrow_repo: Arc<dyn EscrowRepository>,
    pub dispute_repo: Arc<dyn DisputeRepository>,
    pub user_repo: Arc<dyn UserRepository>,
}

impl Services {
    pub fn new(
        custodian: impl CustodianProvider + 'static,
        payment: impl PaymentProcessor + 'static,
        arbitration: impl ArbitrationStrategy + 'static,
        identity: impl IdentityProvider + 'static,
        escrow_repo: impl EscrowRepository + 'static,
        dispute_repo: impl DisputeRepository + 'static,
        user_repo: impl UserRepository + 'static,
    ) -> Self {
        Self {
            custodian: Arc::new(custodian),
            payment: Arc::new(payment),
            arbitration: Arc::new(arbitration),
            identity: Arc::new(identity),
            escrow_repo: Arc::new(escrow_repo),
            dispute_repo: Arc::new(dispute_repo),
            user_repo: Arc::new(user_repo),
        }
    }
}
