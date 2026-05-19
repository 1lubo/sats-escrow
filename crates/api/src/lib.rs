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
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "SatsEscrow API",
        version = "0.1.0",
        description = "Bitcoin escrow service API — create, fund, deliver, confirm, dispute, and cancel peer-to-peer escrow contracts.",
        license(name = "MIT"),
    ),
    paths(
        // Escrow
        routes::escrow::create_escrow,
        routes::escrow::get_escrow,
        routes::escrow::list_user_escrows,
        routes::escrow::fund_escrow,
        routes::escrow::mark_delivered,
        routes::escrow::confirm_escrow,
        routes::escrow::open_dispute,
        routes::escrow::cancel_escrow,
        // Disputes
        routes::dispute::get_dispute,
        routes::dispute::list_open_disputes,
        routes::dispute::submit_vote,
        // Users
        routes::user::get_current_user,
        routes::user::get_user_reputation,
        // Health
        routes::health::health_check,
    ),
    components(schemas(
        routes::escrow::CreateEscrowRequest,
        routes::escrow::PartyDto,
        routes::escrow::EscrowTermsDto,
        routes::escrow::EscrowResponse,
        routes::escrow::DisputeRequest,
        routes::escrow::FundRequest,
        routes::escrow::CancelRequest,
        routes::escrow::ActionResponse,
        routes::dispute::DisputeResponse,
        routes::dispute::VoteRequest,
        routes::dispute::PartyDecision,
        routes::user::UserResponse,
        routes::user::ReputationDto,
        routes::health::HealthResponse,
    )),
    security(
        ("bearer_auth" = [])
    ),
    modifiers(&SecurityAddon),
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearer_auth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            );
        }
    }
}

/// Create the API router with all routes
pub fn create_router(state: AppState) -> Router {
    Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(routes::escrow::router())
        .merge(routes::dispute::router())
        .merge(routes::user::router())
        .merge(routes::health::router())
        .with_state(state)
}
