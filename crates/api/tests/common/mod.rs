//! Test helpers for API integration tests

use axum::Router;
use sats_escrow_adapters::{
    MockArbitration, MockCustodian, MockDisputeRepository, MockEscrowRepository,
    MockIdentityProvider, MockPaymentProcessor, MockUserRepository,
};
use sats_escrow_api::{create_router, state::Services, AppState};

/// Create a test router with mock services
pub fn test_router() -> Router {
    let services = Services::new(
        MockCustodian::new(),
        MockPaymentProcessor::default(),
        MockArbitration::new(),
        MockIdentityProvider::new(),
        MockEscrowRepository::new(),
        MockDisputeRepository::new(),
        MockUserRepository::new(),
    );
    let state = AppState::new(services);
    create_router(state)
}

/// Helper to make JSON POST request body
#[allow(dead_code)]
pub fn json_body(json: serde_json::Value) -> String {
    json.to_string()
}
