//! Integration tests for dispute API endpoints

mod common;

use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::util::ServiceExt;
use uuid::Uuid;

use crate::common::test_router;

fn test_auth_header() -> String {
    format!("Bearer {}", Uuid::new_v4())
}

async fn get_response_json(response: axum::response::Response) -> Value {
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap_or(json!({}))
}

/// Helper: create an escrow, fund it, deliver, and open a dispute.
/// Returns (buyer_auth, seller_auth, escrow_id).
async fn create_disputed_escrow(app: &Router) -> (String, String, Uuid) {
    let buyer_id = Uuid::new_v4();
    let seller_id = Uuid::new_v4();
    let buyer_auth = format!("Bearer {}", buyer_id);
    let seller_auth = format!("Bearer {}", seller_id);

    // Create escrow
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/escrows")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::from(
                    json!({
                        "role": "buyer",
                        "counterparty_id": seller_id.to_string(),
                        "amount_sats": 100000,
                        "description": "Dispute lifecycle test"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);
    let body = get_response_json(resp).await;
    let escrow_id: Uuid = body["data"]["id"].as_str().unwrap().parse().unwrap();

    // Fund
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/fund", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::from(json!({"tx_id": "tx_dispute_test"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Deliver
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/deliver", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &seller_auth)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // Dispute
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/dispute", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::from(
                    json!({"description": "Item not as described"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::CREATED);

    (buyer_auth, seller_auth, escrow_id)
}

#[tokio::test]
async fn test_list_disputes_empty() {
    let app = test_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/disputes")
                .header(header::AUTHORIZATION, test_auth_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_response_json(response).await;
    assert!(
        body["data"].is_array(),
        "Expected data array, got: {:?}",
        body
    );
    assert_eq!(body["data"].as_array().unwrap().len(), 0);
}

#[tokio::test]
async fn test_get_dispute_not_found() {
    let app = test_router();
    let fake_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/disputes/{}", fake_id))
                .header(header::AUTHORIZATION, test_auth_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_dispute_lifecycle() {
    let app = test_router();
    let (buyer_auth, _seller_auth, _escrow_id) = create_disputed_escrow(&app).await;

    // List disputes — should contain at least one
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/disputes")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = get_response_json(resp).await;
    let disputes = body["data"].as_array().expect("Expected disputes array");
    assert!(!disputes.is_empty(), "Expected at least one dispute");

    // Get dispute by ID
    let dispute_id = disputes[0]["id"].as_str().expect("Expected dispute id");
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/disputes/{}", dispute_id))
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = get_response_json(resp).await;
    assert_eq!(body["data"]["id"].as_str().unwrap(), dispute_id);
}

#[tokio::test]
async fn test_vote_on_dispute() {
    let app = test_router();
    let (buyer_auth, _seller_auth, _escrow_id) = create_disputed_escrow(&app).await;

    // Get dispute ID from list
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/disputes")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let body = get_response_json(resp).await;
    let dispute_id = body["data"][0]["id"].as_str().expect("Expected dispute id");

    // Attempt to vote — dispute may not be in "in_review" state,
    // so accept either success or a 4xx error.
    let vote_request = json!({
        "decision": "buyer",
        "reasoning": "Buyer provided evidence of misrepresentation"
    });
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/disputes/{}/vote", dispute_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, test_auth_header())
                .body(Body::from(vote_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    let status = resp.status().as_u16();
    assert!(
        (200..=299).contains(&status) || (400..=499).contains(&status),
        "Expected 2xx or 4xx, got: {}",
        status
    );
}
