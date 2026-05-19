//! Dispute routes

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use sats_escrow_core::{
    dispute::{Dispute, DisputeState, Vote},
    types::{DisputeId, Party},
};

use crate::{
    error::{ApiError, ApiResult},
    extractors::{AuthUser, PaginationParams},
    response::{ApiResponse, PaginatedResponse},
    state::AppState,
};

// === DTOs ===

#[derive(Debug, Serialize, ToSchema)]
pub struct DisputeResponse {
    pub id: Uuid,
    pub escrow_id: Uuid,
    pub state: String,
    pub opened_by: Uuid,
    pub created_at: String,
    pub votes_count: usize,
}

impl From<&Dispute> for DisputeResponse {
    fn from(d: &Dispute) -> Self {
        Self {
            id: d.id.0,
            escrow_id: d.escrow_id.0,
            state: match &d.state {
                DisputeState::Opened => "opened".to_string(),
                DisputeState::InReview { .. } => "in_review".to_string(),
                DisputeState::Resolved { .. } => "resolved".to_string(),
            },
            opened_by: d.opened_by.0,
            created_at: d.created_at.to_rfc3339(),
            votes_count: d.votes.len(),
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VoteRequest {
    pub decision: PartyDecision,
    pub reasoning: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum PartyDecision {
    Buyer,
    Seller,
}

// === Handlers ===

/// Get a dispute by ID
#[utoipa::path(
    get,
    path = "/api/v1/disputes/{id}",
    params(("id" = Uuid, Path, description = "Dispute UUID")),
    responses(
        (status = 200, description = "Dispute details", body = DisputeResponse),
        (status = 404, description = "Dispute not found"),
    ),
    tag = "Disputes"
)]
pub async fn get_dispute(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<ApiResponse<DisputeResponse>> {
    let dispute = state
        .services
        .dispute_repo
        .find_by_id(&DisputeId(id))
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Dispute not found"))?;

    Ok(ApiResponse::new(DisputeResponse::from(&dispute)))
}

/// List open disputes (paginated)
#[utoipa::path(
    get,
    path = "/api/v1/disputes",
    params(
        ("limit" = Option<usize>, Query, description = "Max results per page"),
        ("offset" = Option<usize>, Query, description = "Number of results to skip"),
    ),
    responses(
        (status = 200, description = "Paginated list of open disputes", body = [DisputeResponse]),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "Disputes"
)]
pub async fn list_open_disputes(
    State(state): State<AppState>,
    AuthUser(_user_id): AuthUser,
    Query(pagination): Query<PaginationParams>,
) -> ApiResult<PaginatedResponse<DisputeResponse>> {
    let disputes = state
        .services
        .dispute_repo
        .find_open_disputes()
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let total = disputes.len();
    let responses: Vec<DisputeResponse> = disputes
        .iter()
        .skip(pagination.offset)
        .take(pagination.limit)
        .map(DisputeResponse::from)
        .collect();
    Ok(PaginatedResponse::new(
        responses,
        total,
        pagination.limit,
        pagination.offset,
    ))
}

/// Submit a vote on a dispute (arbitrator action)
#[utoipa::path(
    post,
    path = "/api/v1/disputes/{id}/vote",
    params(("id" = Uuid, Path, description = "Dispute UUID")),
    request_body = VoteRequest,
    responses(
        (status = 200, description = "Vote recorded", body = DisputeResponse),
        (status = 404, description = "Dispute not found"),
        (status = 409, description = "Already voted or dispute not votable"),
    ),
    security(("bearer_auth" = [])),
    tag = "Disputes"
)]
pub async fn submit_vote(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<VoteRequest>,
) -> ApiResult<ApiResponse<DisputeResponse>> {
    let mut dispute = state
        .services
        .dispute_repo
        .find_by_id(&DisputeId(id))
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Dispute not found"))?;

    let decision = match req.decision {
        PartyDecision::Buyer => Party::Buyer,
        PartyDecision::Seller => Party::Seller,
    };

    let vote = Vote {
        arbitrator: user_id,
        decision,
        reasoning: req.reasoning,
        voted_at: Utc::now(),
    };

    dispute.record_vote(vote).map_err(ApiError::from)?;

    state
        .services
        .dispute_repo
        .update(&dispute)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(ApiResponse::new(DisputeResponse::from(&dispute)))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/disputes", get(list_open_disputes))
        .route("/api/v1/disputes/:id", get(get_dispute))
        .route("/api/v1/disputes/:id/vote", post(submit_vote))
}
