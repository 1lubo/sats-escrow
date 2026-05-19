//! SatsEscrow Server - Main entry point
//!
//! This binary wires together all the components and starts the HTTP server.
//!
//! Configuration via environment variables:
//! - `PORT` - HTTP server port (default: 3000)
//! - `MONGODB_URI` - MongoDB connection string (optional)
//! - `DATABASE_NAME` - MongoDB database name (default: sats_escrow)

mod config;

use std::net::SocketAddr;

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use sats_escrow_adapters::{
    MockArbitration, MockCustodian, MockDisputeRepository, MockEscrowRepository,
    MockIdentityProvider, MockPaymentProcessor, MockUserRepository, MongoClient,
    MongoDisputeRepository, MongoEscrowRepository, MongoUserRepository,
};
use sats_escrow_api::{create_router, state::Services, AppState};

use crate::config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sats_escrow=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env();

    tracing::info!("Starting SatsEscrow server...");
    tracing::info!("Port: {}", config.port);
    tracing::info!(
        "Database: {}",
        if config.use_mongodb() {
            "MongoDB"
        } else {
            "In-Memory (Mock)"
        }
    );

    // Build application state based on configuration
    let state = if let Some(ref uri) = config.mongodb_uri {
        create_mongo_state(uri, &config.database_name).await?
    } else {
        create_mock_state()
    };

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
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// Create application state with mock adapters (for development/testing)
fn create_mock_state() -> AppState {
    let services = Services::new(
        MockCustodian::new(),
        MockPaymentProcessor::default(),
        MockArbitration::default(),
        MockIdentityProvider::new(),
        MockEscrowRepository::new(),
        MockDisputeRepository::new(),
        MockUserRepository::new(),
    );
    AppState::new(services)
}

/// Create application state with MongoDB adapters (for production)
async fn create_mongo_state(uri: &str, database_name: &str) -> anyhow::Result<AppState> {
    tracing::info!("Connecting to MongoDB...");

    let client = MongoClient::connect(uri)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to MongoDB: {}", e))?;

    // Verify connection
    client
        .ping()
        .await
        .map_err(|e| anyhow::anyhow!("MongoDB ping failed: {}", e))?;

    tracing::info!("MongoDB connected successfully");

    let db = client.database(database_name);

    let services = Services::new(
        MockCustodian::new(),            // TODO: Real Bitcoin custodian
        MockPaymentProcessor::default(), // TODO: Real payment processor
        MockArbitration::default(),      // TODO: Real arbitration service
        MockIdentityProvider::new(),     // TODO: Real identity provider
        MongoEscrowRepository::new(db.clone()),
        MongoDisputeRepository::new(db.clone()),
        MongoUserRepository::new(db),
    );

    Ok(AppState::new(services))
}
