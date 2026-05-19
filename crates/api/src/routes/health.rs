//! Health check routes

use std::sync::OnceLock;
use std::time::Instant;

use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::state::AppState;

static START_TIME: OnceLock<Instant> = OnceLock::new();

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    version: &'static str,
    uptime_seconds: u64,
    commit: &'static str,
    timestamp: String,
}

async fn health_check() -> Json<HealthResponse> {
    let start = START_TIME.get_or_init(Instant::now);
    let uptime_seconds = start.elapsed().as_secs();

    Json(HealthResponse {
        status: "healthy",
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds,
        commit: option_env!("GIT_COMMIT_SHA").unwrap_or("dev"),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}
