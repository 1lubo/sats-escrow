//! SatsEscrow API - HTTP layer using Axum
//!
//! This crate provides the REST API for the escrow service.

pub mod error;
pub mod extractors;
pub mod response;
pub mod routes;
pub mod state;

pub use state::AppState;

use axum::Router;

/// Create the API router with all routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(routes::escrow::router())
        .merge(routes::dispute::router())
        .merge(routes::user::router())
        .merge(routes::health::router())
        .with_state(state)
}
