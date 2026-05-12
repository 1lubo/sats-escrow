//! API error handling

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use sats_escrow_core::Error as CoreError;

/// API error type
#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: ErrorBody,
}

#[derive(Serialize)]
struct ErrorBody {
    code: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = ErrorResponse {
            error: ErrorBody {
                code: self.code,
                message: self.message,
                details: self.details,
            },
        };

        (self.status, Json(body)).into_response()
    }
}

impl From<CoreError> for ApiError {
    fn from(err: CoreError) -> Self {
        match &err {
            CoreError::EscrowNotFound(_) => ApiError {
                status: StatusCode::NOT_FOUND,
                code: "ESCROW_NOT_FOUND".to_string(),
                message: err.to_string(),
                details: None,
            },
            CoreError::DisputeNotFound(_) => ApiError {
                status: StatusCode::NOT_FOUND,
                code: "DISPUTE_NOT_FOUND".to_string(),
                message: err.to_string(),
                details: None,
            },
            CoreError::InvalidStateTransition { from, to } => ApiError {
                status: StatusCode::CONFLICT,
                code: "INVALID_STATE_TRANSITION".to_string(),
                message: err.to_string(),
                details: Some(serde_json::json!({
                    "current_state": format!("{:?}", from),
                    "attempted_transition": to
                })),
            },
            CoreError::Unauthorized(_) => ApiError {
                status: StatusCode::UNAUTHORIZED,
                code: "UNAUTHORIZED".to_string(),
                message: err.to_string(),
                details: None,
            },
            CoreError::Validation(_) => ApiError {
                status: StatusCode::BAD_REQUEST,
                code: "VALIDATION_ERROR".to_string(),
                message: err.to_string(),
                details: None,
            },
            CoreError::DisputeAlreadyExists => ApiError {
                status: StatusCode::CONFLICT,
                code: "DISPUTE_EXISTS".to_string(),
                message: err.to_string(),
                details: None,
            },
            CoreError::DisputeNotVotable => ApiError {
                status: StatusCode::CONFLICT,
                code: "DISPUTE_NOT_VOTABLE".to_string(),
                message: err.to_string(),
                details: None,
            },
            CoreError::AlreadyVoted => ApiError {
                status: StatusCode::CONFLICT,
                code: "ALREADY_VOTED".to_string(),
                message: err.to_string(),
                details: None,
            },
            CoreError::InsufficientFunds { required, available } => ApiError {
                status: StatusCode::PAYMENT_REQUIRED,
                code: "INSUFFICIENT_FUNDS".to_string(),
                message: err.to_string(),
                details: Some(serde_json::json!({
                    "required": required,
                    "available": available
                })),
            },
            CoreError::Custodian(_) | CoreError::PaymentProcessor(_) | CoreError::IdentityProvider(_) => ApiError {
                status: StatusCode::SERVICE_UNAVAILABLE,
                code: "EXTERNAL_SERVICE_ERROR".to_string(),
                message: "An external service is unavailable".to_string(),
                details: None,
            },
        }
    }
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            code: "BAD_REQUEST".to_string(),
            message: message.into(),
            details: None,
        }
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            code: "NOT_FOUND".to_string(),
            message: message.into(),
            details: None,
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            code: "INTERNAL_ERROR".to_string(),
            message: message.into(),
            details: None,
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            status: StatusCode::FORBIDDEN,
            code: "FORBIDDEN".to_string(),
            message: message.into(),
            details: None,
        }
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
