//! SatsEscrow Server - Main entry point
//!
//! This binary wires together all the components and starts the HTTP server.

use std::net::SocketAddr;

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sats_escrow_adapters::{
    MockArbitration, MockCustodian, MockDisputeRepository, MockEscrowRepository,
    MockIdentityProvider, MockPaymentProcessor, MockUserRepository,
};
use sats_escrow_api::{create_router, state::Services, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sats_escrow=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting SatsEscrow server...");

    // Create mock services (in production, these would be real implementations)
    let custodian = MockCustodian::new();
    let payment = MockPaymentProcessor::default();
    let arbitration = MockArbitration::default();
    let identity = MockIdentityProvider::new();
    let escrow_repo = MockEscrowRepository::new();
    let dispute_repo = MockDisputeRepository::new();
    let user_repo = MockUserRepository::new();

    // Wire up services
    let services = Services::new(
        custodian,
        payment,
        arbitration,
        identity,
        escrow_repo,
        dispute_repo,
        user_repo,
    );

    let state = AppState::new(services);

    // Create router with middleware
    let app = create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
