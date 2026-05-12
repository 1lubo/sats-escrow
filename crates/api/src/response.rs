//! Unified API response types

use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

/// Wrapper for successful API responses
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T: Serialize> IntoResponse for ApiResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

/// Response with status code
pub struct CreatedResponse<T: Serialize>(pub T);

impl<T: Serialize> IntoResponse for CreatedResponse<T> {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::CREATED, Json(ApiResponse::new(self.0))).into_response()
    }
}

/// Empty success response (204 No Content)
pub struct NoContent;

impl IntoResponse for NoContent {
    fn into_response(self) -> axum::response::Response {
        StatusCode::NO_CONTENT.into_response()
    }
}

/// Paginated response
#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub data: Vec<T>,
    pub pagination: Pagination,
}

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
    pub has_more: bool,
}

impl<T: Serialize> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: usize, limit: usize, offset: usize) -> Self {
        Self {
            pagination: Pagination {
                total,
                limit,
                offset,
                has_more: offset + data.len() < total,
            },
            data,
        }
    }
}

impl<T: Serialize> IntoResponse for PaginatedResponse<T> {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}
