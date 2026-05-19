//! Integration tests for escrow API endpoints

mod common;

use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
    Router,
};
use http_body_util::BodyExt;
use tower::util::ServiceExt;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::common::test_router;

/// Generate a test auth header value
fn test_auth_header() -> String {
    format!("Bearer {}", Uuid::new_v4())
}

async fn get_response_json(response: axum::response::Response) -> Value {
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap_or(json!({}))
}

// === Health Check Tests ===

#[tokio::test]
async fn test_health_check() {
    let app = test_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_response_json(response).await;
    assert_eq!(body["status"], "healthy");
}

// === Escrow Creation Tests ===

#[tokio::test]
async fn test_create_escrow_as_buyer() {
    let app = test_router();
    let seller_id = Uuid::new_v4();

    let create_request = json!({
        "role": "buyer",
        "counterparty_id": seller_id.to_string(),
        "amount_sats": 100000,
        "description": "Test escrow for integration test"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/escrows")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, test_auth_header())
                .body(Body::from(create_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = get_response_json(response).await;

    // Response is wrapped in { data: { ... } }
    let data = &body["data"];
    assert!(
        data["id"].is_string(),
        "Expected id to be a string, got: {:?}",
        data
    );
    assert_eq!(data["state"], "created");
    assert_eq!(data["amount_sats"], 100000);
}

#[tokio::test]
async fn test_create_escrow_as_seller() {
    let app = test_router();
    let buyer_id = Uuid::new_v4();

    let create_request = json!({
        "role": "seller",
        "counterparty_id": buyer_id.to_string(),
        "amount_sats": 50000,
        "description": "Seller-initiated escrow"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/escrows")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, test_auth_header())
                .body(Body::from(create_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = get_response_json(response).await;

    // Response is wrapped in { data: { ... } }
    let data = &body["data"];
    assert!(data["id"].is_string());
    assert_eq!(data["state"], "created");
}

#[tokio::test]
async fn test_create_escrow_missing_auth_header() {
    let app = test_router();

    let create_request = json!({
        "role": "buyer",
        "counterparty_id": Uuid::new_v4().to_string(),
        "amount_sats": 100000,
        "description": "Test escrow"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/escrows")
                .header(header::CONTENT_TYPE, "application/json")
                // No Authorization header
                .body(Body::from(create_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

// === List Escrows Tests ===

#[tokio::test]
async fn test_list_escrows_empty() {
    let app = test_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/escrows")
                .header(header::AUTHORIZATION, test_auth_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_response_json(response).await;

    // Response format: { data: [...] }
    assert!(
        body["data"].is_array(),
        "Expected data to be an array, got: {:?}",
        body
    );
}

// === Get Escrow Tests ===

#[tokio::test]
async fn test_get_escrow_not_found() {
    let app = test_router();
    // Use a valid UUID format for the escrow ID
    let fake_escrow_id = Uuid::new_v4();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/escrows/{}", fake_escrow_id))
                .header(header::AUTHORIZATION, test_auth_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

// === Invalid Request Tests ===

#[tokio::test]
async fn test_create_escrow_invalid_json() {
    let app = test_router();

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/escrows")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, test_auth_header())
                .body(Body::from("{ invalid json }"))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 400 or 422 for invalid JSON
    assert!(
        response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNPROCESSABLE_ENTITY
    );
}

#[tokio::test]
async fn test_create_escrow_missing_required_field() {
    let app = test_router();

    let create_request = json!({
        "role": "buyer",
        // Missing counterparty_id
        "amount_sats": 100000,
        "description": "Test"
    });

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/escrows")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, test_auth_header())
                .body(Body::from(create_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Should return 400 or 422 for missing field
    assert!(
        response.status() == StatusCode::BAD_REQUEST
            || response.status() == StatusCode::UNPROCESSABLE_ENTITY
    );
}

// === Multi-step Escrow Lifecycle Tests ===

/// Helper: create an escrow via API and return (buyer_auth, seller_id, escrow_id)
async fn create_test_escrow_via_api(app: &Router) -> (String, Uuid, Uuid) {
    let buyer_id = Uuid::new_v4();
    let seller_id = Uuid::new_v4();
    let buyer_auth = format!("Bearer {}", buyer_id);

    let create_request = json!({
        "role": "buyer",
        "counterparty_id": seller_id.to_string(),
        "amount_sats": 100000,
        "description": "Test escrow for lifecycle"
    });

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/api/v1/escrows")
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::from(create_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = get_response_json(response).await;
    let escrow_id: Uuid = body["data"]["id"].as_str().unwrap().parse().unwrap();
    (buyer_auth, seller_id, escrow_id)
}

#[tokio::test]
async fn test_fund_escrow() {
    let app = test_router();
    let (buyer_auth, _seller_id, escrow_id) = create_test_escrow_via_api(&app).await;

    let fund_request = json!({ "tx_id": "txid_fund_001" });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/fund", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::from(fund_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_response_json(response).await;
    assert_eq!(body["data"]["escrow"]["state"], "funded");
}

#[tokio::test]
async fn test_mark_delivered() {
    let app = test_router();
    let (buyer_auth, seller_id, escrow_id) = create_test_escrow_via_api(&app).await;
    let seller_auth = format!("Bearer {}", seller_id);

    // Fund first
    let fund_request = json!({ "tx_id": "txid_deliver_001" });
    app.clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/fund", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::from(fund_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Mark delivered as seller
    let response = app
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

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_response_json(response).await;
    assert_eq!(body["data"]["escrow"]["state"], "awaiting_delivery");
}

#[tokio::test]
async fn test_confirm_escrow() {
    let app = test_router();
    let (buyer_auth, seller_id, escrow_id) = create_test_escrow_via_api(&app).await;
    let seller_auth = format!("Bearer {}", seller_id);

    // Fund
    app.clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/fund", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::from(json!({"tx_id": "tx_confirm_001"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Deliver
    app.clone()
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

    // Confirm as buyer
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/confirm", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_response_json(response).await;
    assert_eq!(body["data"]["escrow"]["state"], "released_to_seller");
}

#[tokio::test]
async fn test_cancel_escrow() {
    let app = test_router();
    let (buyer_auth, _seller_id, escrow_id) = create_test_escrow_via_api(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/cancel", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::from(json!({}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_response_json(response).await;
    assert_eq!(body["data"]["escrow"]["state"], "cancelled");
}

#[tokio::test]
async fn test_dispute_escrow() {
    let app = test_router();
    let (buyer_auth, seller_id, escrow_id) = create_test_escrow_via_api(&app).await;
    let seller_auth = format!("Bearer {}", seller_id);

    // Fund
    app.clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/fund", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::from(json!({"tx_id": "tx_dispute_001"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    // Deliver
    app.clone()
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

    // Dispute as buyer
    let dispute_request = json!({
        "description": "Item not as described"
    });
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/dispute", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::from(dispute_request.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = get_response_json(response).await;
    assert_eq!(body["data"]["escrow"]["state"], "disputed");
}

#[tokio::test]
async fn test_get_single_escrow() {
    let app = test_router();
    let (_buyer_auth, _seller_id, escrow_id) = create_test_escrow_via_api(&app).await;

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/escrows/{}", escrow_id))
                .header(header::AUTHORIZATION, test_auth_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = get_response_json(response).await;
    assert_eq!(body["data"]["id"], escrow_id.to_string());
}

#[tokio::test]
async fn test_full_escrow_lifecycle() {
    let app = test_router();
    let (buyer_auth, seller_id, escrow_id) = create_test_escrow_via_api(&app).await;
    let seller_auth = format!("Bearer {}", seller_id);

    // 1. Fund
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/fund", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::from(json!({"tx_id": "tx_lifecycle_001"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);

    // 2. Deliver
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

    // 3. Confirm
    let resp = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri(format!("/api/v1/escrows/{}/confirm", escrow_id))
                .header(header::CONTENT_TYPE, "application/json")
                .header(header::AUTHORIZATION, &buyer_auth)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
    let body = get_response_json(resp).await;
    assert_eq!(body["data"]["escrow"]["state"], "released_to_seller");
}
