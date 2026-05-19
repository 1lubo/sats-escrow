//! Escrow routes

use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use sats_escrow_core::{
    dispute::Dispute,
    escrow::{CancelReason, Escrow, EscrowTerms},
    types::{EscrowId, Evidence, Party, Satoshis, TxId},
    user::UserId,
};

use crate::{
    error::{ApiError, ApiResult},
    extractors::{AuthUser, PaginationParams},
    response::{ApiResponse, CreatedResponse, PaginatedResponse},
    state::AppState,
};

// === Request/Response DTOs ===

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateEscrowRequest {
    pub role: PartyDto,
    pub counterparty_id: Uuid,
    pub amount_sats: u64,
    pub description: String,
    #[serde(default)]
    pub terms: Option<EscrowTermsDto>,
}

impl CreateEscrowRequest {
    #[allow(clippy::result_large_err)]
    fn validate(&self) -> Result<(), ApiError> {
        if self.amount_sats == 0 {
            return Err(ApiError::bad_request("Amount must be greater than 0"));
        }
        if self.description.trim().is_empty() {
            return Err(ApiError::bad_request("Description cannot be empty"));
        }
        if self.description.len() > 1000 {
            return Err(ApiError::bad_request(
                "Description must not exceed 1000 characters",
            ));
        }
        if let Some(terms) = &self.terms {
            if let Some(days) = terms.auto_release_days {
                if !(1..=90).contains(&days) {
                    return Err(ApiError::bad_request(
                        "auto_release_days must be between 1 and 90",
                    ));
                }
            }
            if let Some(days) = terms.dispute_window_days {
                if !(1..=30).contains(&days) {
                    return Err(ApiError::bad_request(
                        "dispute_window_days must be between 1 and 30",
                    ));
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum PartyDto {
    Buyer,
    Seller,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct EscrowTermsDto {
    pub auto_release_days: Option<i64>,
    pub dispute_window_days: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
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
            state: match &e.state {
                sats_escrow_core::escrow::EscrowState::Created => "created",
                sats_escrow_core::escrow::EscrowState::Funded => "funded",
                sats_escrow_core::escrow::EscrowState::AwaitingDelivery { .. } => {
                    "awaiting_delivery"
                }
                sats_escrow_core::escrow::EscrowState::Disputed { .. } => "disputed",
                sats_escrow_core::escrow::EscrowState::Cancelled { .. } => "cancelled",
                sats_escrow_core::escrow::EscrowState::ReleasedToSeller { .. } => {
                    "released_to_seller"
                }
                sats_escrow_core::escrow::EscrowState::ReleasedToBuyer { .. } => {
                    "released_to_buyer"
                }
            }
            .to_string(),
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

#[derive(Debug, Deserialize, ToSchema)]
pub struct DisputeRequest {
    pub description: String,
    #[serde(default)]
    pub attachments: Vec<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FundRequest {
    pub tx_id: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CancelRequest {
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ActionResponse {
    pub success: bool,
    pub message: String,
    pub escrow: EscrowResponse,
}

// === Route Handlers ===

/// Create a new escrow contract
#[utoipa::path(
    post,
    path = "/api/v1/escrows",
    request_body = CreateEscrowRequest,
    responses(
        (status = 201, description = "Escrow created", body = EscrowResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "Escrows"
)]
#[tracing::instrument(skip_all)]
pub async fn create_escrow(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(req): Json<CreateEscrowRequest>,
) -> ApiResult<CreatedResponse<EscrowResponse>> {
    req.validate()?;

    if user_id.0 == req.counterparty_id {
        return Err(ApiError::bad_request("Cannot create escrow with yourself"));
    }

    let (initiator, buyer, seller) = match req.role {
        PartyDto::Buyer => (Party::Buyer, user_id.clone(), UserId(req.counterparty_id)),
        PartyDto::Seller => (Party::Seller, UserId(req.counterparty_id), user_id.clone()),
    };

    let terms = req
        .terms
        .map(|t| EscrowTerms {
            auto_release_after: chrono::Duration::days(t.auto_release_days.unwrap_or(14)),
            dispute_window: chrono::Duration::days(t.dispute_window_days.unwrap_or(7)),
        })
        .unwrap_or_default();

    let mut escrow = Escrow::new(
        initiator,
        buyer,
        seller,
        Satoshis(req.amount_sats),
        req.description,
        terms,
    );

    // Create deposit address via custodian
    let address = state
        .services
        .custodian
        .create_deposit_address(&escrow.id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;
    escrow.set_deposit_address(address);

    // Persist
    state
        .services
        .escrow_repo
        .create(&escrow)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(CreatedResponse(EscrowResponse::from(&escrow)))
}

/// Get an escrow by ID
#[utoipa::path(
    get,
    path = "/api/v1/escrows/{id}",
    params(("id" = Uuid, Path, description = "Escrow UUID")),
    responses(
        (status = 200, description = "Escrow details", body = EscrowResponse),
        (status = 404, description = "Escrow not found"),
    ),
    tag = "Escrows"
)]
#[tracing::instrument(skip_all, fields(escrow_id = %id))]
pub async fn get_escrow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<ApiResponse<EscrowResponse>> {
    let escrow = state
        .services
        .escrow_repo
        .find_by_id(&sats_escrow_core::types::EscrowId(id))
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Escrow not found"))?;

    Ok(ApiResponse::new(EscrowResponse::from(&escrow)))
}

/// List escrows for the authenticated user
#[utoipa::path(
    get,
    path = "/api/v1/escrows",
    params(
        ("limit" = Option<usize>, Query, description = "Max results per page"),
        ("offset" = Option<usize>, Query, description = "Number of results to skip"),
    ),
    responses(
        (status = 200, description = "Paginated list of escrows", body = [EscrowResponse]),
        (status = 401, description = "Unauthorized"),
    ),
    security(("bearer_auth" = [])),
    tag = "Escrows"
)]
#[tracing::instrument(skip_all)]
pub async fn list_user_escrows(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Query(pagination): Query<PaginationParams>,
) -> ApiResult<PaginatedResponse<EscrowResponse>> {
    let escrows = state
        .services
        .escrow_repo
        .find_by_user(&user_id)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    let total = escrows.len();
    let responses: Vec<EscrowResponse> = escrows
        .iter()
        .skip(pagination.offset)
        .take(pagination.limit)
        .map(EscrowResponse::from)
        .collect();
    Ok(PaginatedResponse::new(
        responses,
        total,
        pagination.limit,
        pagination.offset,
    ))
}

/// Mark escrow as funded (typically called via webhook from custodian)
#[utoipa::path(
    post,
    path = "/api/v1/escrows/{id}/fund",
    params(("id" = Uuid, Path, description = "Escrow UUID")),
    request_body = FundRequest,
    responses(
        (status = 200, description = "Escrow funded", body = ActionResponse),
        (status = 404, description = "Escrow not found"),
        (status = 409, description = "Invalid state transition"),
    ),
    tag = "Escrows"
)]
#[tracing::instrument(skip_all, fields(escrow_id = %id))]
pub async fn fund_escrow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<FundRequest>,
) -> ApiResult<ApiResponse<ActionResponse>> {
    let mut escrow = state
        .services
        .escrow_repo
        .find_by_id(&EscrowId(id))
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Escrow not found"))?;

    escrow
        .mark_funded(TxId(req.tx_id))
        .map_err(ApiError::from)?;

    state
        .services
        .escrow_repo
        .update(&escrow)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(ApiResponse::new(ActionResponse {
        success: true,
        message: "Escrow marked as funded".to_string(),
        escrow: EscrowResponse::from(&escrow),
    }))
}

/// Seller marks delivery complete
#[utoipa::path(
    post,
    path = "/api/v1/escrows/{id}/deliver",
    params(("id" = Uuid, Path, description = "Escrow UUID")),
    responses(
        (status = 200, description = "Delivery marked", body = ActionResponse),
        (status = 403, description = "Only seller can mark delivery"),
        (status = 404, description = "Escrow not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "Escrows"
)]
#[tracing::instrument(skip_all, fields(escrow_id = %id))]
pub async fn mark_delivered(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> ApiResult<ApiResponse<ActionResponse>> {
    let mut escrow = state
        .services
        .escrow_repo
        .find_by_id(&EscrowId(id))
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Escrow not found"))?;

    // Verify caller is the seller
    if escrow.seller != user_id {
        return Err(ApiError::forbidden("Only the seller can mark as delivered"));
    }

    escrow.mark_delivered(user_id).map_err(ApiError::from)?;

    state
        .services
        .escrow_repo
        .update(&escrow)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(ApiResponse::new(ActionResponse {
        success: true,
        message: "Escrow marked as delivered".to_string(),
        escrow: EscrowResponse::from(&escrow),
    }))
}

/// Buyer confirms receipt and releases funds to seller
#[utoipa::path(
    post,
    path = "/api/v1/escrows/{id}/confirm",
    params(("id" = Uuid, Path, description = "Escrow UUID")),
    responses(
        (status = 200, description = "Escrow confirmed, funds released", body = ActionResponse),
        (status = 403, description = "Only buyer can confirm"),
        (status = 404, description = "Escrow not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "Escrows"
)]
#[tracing::instrument(skip_all, fields(escrow_id = %id))]
pub async fn confirm_escrow(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> ApiResult<ApiResponse<ActionResponse>> {
    let mut escrow = state
        .services
        .escrow_repo
        .find_by_id(&EscrowId(id))
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Escrow not found"))?;

    // Verify caller is the buyer
    if escrow.buyer != user_id {
        return Err(ApiError::forbidden("Only the buyer can confirm receipt"));
    }

    // Release funds to seller via custodian
    let tx_id = state
        .services
        .custodian
        .transfer(&escrow.id, &escrow.seller, escrow.amount)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    escrow.confirm(user_id, tx_id).map_err(ApiError::from)?;

    state
        .services
        .escrow_repo
        .update(&escrow)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(ApiResponse::new(ActionResponse {
        success: true,
        message: "Escrow confirmed, funds released to seller".to_string(),
        escrow: EscrowResponse::from(&escrow),
    }))
}

/// Buyer opens a dispute on the escrow
#[utoipa::path(
    post,
    path = "/api/v1/escrows/{id}/dispute",
    params(("id" = Uuid, Path, description = "Escrow UUID")),
    request_body = DisputeRequest,
    responses(
        (status = 201, description = "Dispute opened", body = ActionResponse),
        (status = 403, description = "Only buyer can dispute"),
        (status = 404, description = "Escrow not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "Escrows"
)]
#[tracing::instrument(skip_all, fields(escrow_id = %id))]
pub async fn open_dispute(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<DisputeRequest>,
) -> ApiResult<CreatedResponse<ActionResponse>> {
    let mut escrow = state
        .services
        .escrow_repo
        .find_by_id(&EscrowId(id))
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Escrow not found"))?;

    // Verify caller is the buyer
    if escrow.buyer != user_id {
        return Err(ApiError::forbidden("Only the buyer can open a dispute"));
    }

    // Create the dispute record
    let evidence = Evidence {
        description: req.description,
        attachments: req.attachments,
    };

    let dispute = Dispute::new(escrow.id.clone(), user_id.clone(), evidence);
    let dispute_id = dispute.id.clone();

    // Update escrow state
    escrow
        .open_dispute(user_id, dispute_id)
        .map_err(ApiError::from)?;

    // Persist both
    state
        .services
        .dispute_repo
        .create(&dispute)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    state
        .services
        .escrow_repo
        .update(&escrow)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(CreatedResponse(ActionResponse {
        success: true,
        message: "Dispute opened".to_string(),
        escrow: EscrowResponse::from(&escrow),
    }))
}

/// Cancel escrow (only allowed before funding)
#[utoipa::path(
    post,
    path = "/api/v1/escrows/{id}/cancel",
    params(("id" = Uuid, Path, description = "Escrow UUID")),
    request_body = CancelRequest,
    responses(
        (status = 200, description = "Escrow cancelled", body = ActionResponse),
        (status = 403, description = "Only buyer or seller can cancel"),
        (status = 404, description = "Escrow not found"),
    ),
    security(("bearer_auth" = [])),
    tag = "Escrows"
)]
#[tracing::instrument(skip_all, fields(escrow_id = %id))]
pub async fn cancel_escrow(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
    Json(_req): Json<CancelRequest>,
) -> ApiResult<ApiResponse<ActionResponse>> {
    let mut escrow = state
        .services
        .escrow_repo
        .find_by_id(&EscrowId(id))
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?
        .ok_or_else(|| ApiError::not_found("Escrow not found"))?;

    // Verify caller is buyer or seller
    let reason = if escrow.buyer == user_id {
        CancelReason::BuyerCancelled
    } else if escrow.seller == user_id {
        CancelReason::SellerCancelled
    } else {
        return Err(ApiError::forbidden("Only buyer or seller can cancel"));
    };

    escrow.cancel(reason, user_id).map_err(ApiError::from)?;

    state
        .services
        .escrow_repo
        .update(&escrow)
        .await
        .map_err(|e| ApiError::internal(e.to_string()))?;

    Ok(ApiResponse::new(ActionResponse {
        success: true,
        message: "Escrow cancelled".to_string(),
        escrow: EscrowResponse::from(&escrow),
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/api/v1/escrows",
            post(create_escrow).get(list_user_escrows),
        )
        .route("/api/v1/escrows/:id", get(get_escrow))
        .route("/api/v1/escrows/:id/fund", post(fund_escrow))
        .route("/api/v1/escrows/:id/deliver", post(mark_delivered))
        .route("/api/v1/escrows/:id/confirm", post(confirm_escrow))
        .route("/api/v1/escrows/:id/dispute", post(open_dispute))
        .route("/api/v1/escrows/:id/cancel", post(cancel_escrow))
}
