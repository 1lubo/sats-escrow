//! Escrow routes

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use sats_escrow_core::{
    escrow::{CancelReason, Escrow, EscrowState, EscrowTerms},
    types::{Evidence, Party, Satoshis},
    user::UserId,
};

use crate::{
    error::{ApiError, ApiResult},
    extractors::AuthUser,
    response::{ApiResponse, CreatedResponse},
    state::AppState,
};

// === Request/Response DTOs ===

#[derive(Debug, Deserialize)]
pub struct CreateEscrowRequest {
    pub role: PartyDto,
    pub counterparty_id: Uuid,
    pub amount_sats: u64,
    pub description: String,
    #[serde(default)]
    pub terms: Option<EscrowTermsDto>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PartyDto {
    Buyer,
    Seller,
}

#[derive(Debug, Deserialize)]
pub struct EscrowTermsDto {
    pub auto_release_days: Option<i64>,
    pub dispute_window_days: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct EscrowResponse {
    pub id: Uuid,
    pub state: String,
    pub buyer: Uuid,
    pub seller: Uuid,
    pub amount_sats: u64,
    pub description: String,
    pub deposit_address: Option<String>,
    pub created_at: String,
    pub funded_at: Option<String>,
}

impl From<&Escrow> for EscrowResponse {
    fn from(e: &Escrow) -> Self {
        Self {
            id: e.id.0,
            state: format!("{:?}", e.state).to_lowercase(),
            buyer: e.buyer.0,
            seller: e.seller.0,
            amount_sats: e.amount.0,
            description: e.description.clone(),
            deposit_address: e.deposit_address.as_ref().map(|a| a.0.clone()),
            created_at: e.created_at.to_rfc3339(),
            funded_at: e.funded_at.map(|t| t.to_rfc3339()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DisputeRequest {
    pub description: String,
    #[serde(default)]
    pub attachments: Vec<String>,
}

// === Route Handlers ===

async fn create_escrow(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(req): Json<CreateEscrowRequest>,
) -> ApiResult<CreatedResponse<EscrowResponse>> {
    let (initiator, buyer, seller) = match req.role {
        PartyDto::Buyer => (Party::Buyer, user_id.clone(), UserId(req.counterparty_id)),
        PartyDto::Seller => (Party::Seller, UserId(req.counterparty_id), user_id.clone()),
    };

    let terms = req.terms.map(|t| EscrowTerms {
        auto_release_after: chrono::Duration::days(t.auto_release_days.unwrap_or(14)),
        dispute_window: chrono::Duration::days(t.dispute_window_days.unwrap_or(7)),
    }).unwrap_or_default();

    let mut escrow = Escrow::new(
        initiator,
        buyer,
        seller,
        Satoshis(req.amount_sats),
        req.description,
        terms,
    );

    // Create deposit address via custodian
    let address = state.services.custodian.create_deposit_address(&escrow.id).await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    escrow.set_deposit_address(address);

    // Persist
    state.services.escrow_repo.create(&escrow).await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(CreatedResponse(EscrowResponse::from(&escrow)))
}

async fn get_escrow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<ApiResponse<EscrowResponse>> {
    let escrow = state.services.escrow_repo
        .find_by_id(&sats_escrow_core::types::EscrowId(id))
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Escrow not found"))?;

    Ok(ApiResponse::new(EscrowResponse::from(&escrow)))
}

async fn list_user_escrows(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> ApiResult<ApiResponse<Vec<EscrowResponse>>> {
    let escrows = state.services.escrow_repo
        .find_by_user(&user_id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let responses: Vec<EscrowResponse> = escrows.iter().map(EscrowResponse::from).collect();
    Ok(ApiResponse::new(responses))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/v1/escrows", post(create_escrow).get(list_user_escrows))
        .route("/api/v1/escrows/{id}", get(get_escrow))
}
