//! User routes

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};
use serde::Serialize;
use uuid::Uuid;

use sats_escrow_core::user::{ReputationScore, User};

use crate::{
    error::{ApiError, ApiResult},
    extractors::AuthUser,
    response::ApiResponse,
    state::AppState,
};

// === DTOs ===

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub display_name: String,
    pub reputation: ReputationDto,
    pub role: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ReputationDto {
    pub score: f64,
    pub successful_transactions: u32,
    pub disputes_lost: u32,
}

impl From<&User> for UserResponse {
    fn from(u: &User) -> Self {
        Self {
            id: u.id.0,
            display_name: u.display_name.clone(),
            reputation: ReputationDto {
                score: u.reputation.score,
                successful_transactions: u.reputation.successful_transactions,
                disputes_lost: u.reputation.disputes_lost,
            },
            role: format!("{:?}", u.role).to_lowercase(),
            created_at: u.created_at.to_rfc3339(),
        }
    }
}

// === Handlers ===

async fn get_current_user(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> ApiResult<ApiResponse<UserResponse>> {
    let user = state.services.identity
        .get_user(&user_id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("User not found"))?;

    Ok(ApiResponse::new(UserResponse::from(&user)))
}

async fn get_user_reputation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<ApiResponse<ReputationDto>> {
    let user_id = sats_escrow_core::user::UserId(id);
    let reputation = state.services.identity
        .get_reputation(&user_id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(ApiResponse::new(ReputationDto {
        score: reputation.score,
        successful_transactions: reputation.successful_transactions,
        disputes_lost: reputation.disputes_lost,
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/users/me", get(get_current_user))
        .route("/api/v1/users/:id/reputation", get(get_user_reputation))
}
