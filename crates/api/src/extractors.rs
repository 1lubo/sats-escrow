//! Custom Axum extractors

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
};

use sats_escrow_core::user::UserId;

use crate::error::ApiError;

/// Extractor for authenticated user
pub struct AuthUser(pub UserId);

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| ApiError {
                status: StatusCode::UNAUTHORIZED,
                code: "MISSING_AUTH".to_string(),
                message: "Authorization header required".to_string(),
                details: None,
            })?;

        // For MVP, we accept Bearer tokens in format "Bearer <user_id>"
        // In production, this would validate JWT or session tokens
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(|| ApiError {
                status: StatusCode::UNAUTHORIZED,
                code: "INVALID_AUTH".to_string(),
                message: "Invalid authorization format. Use: Bearer <token>".to_string(),
                details: None,
            })?;

        // For mock purposes, parse the token as a UUID user ID
        // In production, this would validate against IdentityProvider
        let user_id = uuid::Uuid::parse_str(token)
            .map(UserId)
            .map_err(|_| ApiError {
                status: StatusCode::UNAUTHORIZED,
                code: "INVALID_TOKEN".to_string(),
                message: "Invalid token format".to_string(),
                details: None,
            })?;

        Ok(AuthUser(user_id))
    }
}

/// Optional authentication
pub struct OptionalAuthUser(pub Option<UserId>);

#[async_trait]
impl<S> FromRequestParts<S> for OptionalAuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match AuthUser::from_request_parts(parts, state).await {
            Ok(AuthUser(user_id)) => Ok(OptionalAuthUser(Some(user_id))),
            Err(_) => Ok(OptionalAuthUser(None)),
        }
    }
}

/// Pagination parameters
#[derive(Debug, Clone, serde::Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_limit")]
    pub limit: usize,
    #[serde(default)]
    pub offset: usize,
}

fn default_limit() -> usize {
    20
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            limit: default_limit(),
            offset: 0,
        }
    }
}
