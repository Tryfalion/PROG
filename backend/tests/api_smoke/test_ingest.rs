#[path = "../common/mod.rs"]
mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // Für die `oneshot` Methode im Router

/// Test 1 (Spec 004): Eine gültige Rechnung wird angelegt -> 201 CREATED.
#[tokio::test]
async fn test_ingest_invoice_success() {
    let pool = common::setup_pool().await;
    let app = ledger_gate::api::create_router(pool);

    // Konstruiere einen simplen JSON Body
    let json_body = serde_json::json!({
        "invoice_number": "RE-TEST-1",
        "amount": "150.00",
        "currency": "EUR"
    });

    // 1. Route aufrufen per "oneshot" (Ohne TCP Listener, reiner Memory-Aufruf)
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/invoices")
        .header("content-type", "application/json")
        .body(Body::from(json_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // 2. Prüfung (Sollte 201 Created sein)
    assert_eq!(response.status(), StatusCode::CREATED);
}

/// Test 2 (Spec 004): Eine ungültige Währung wird abgelehnt -> 400 BAD REQUEST.
#[tokio::test]
async fn test_ingest_invoice_invalid_currency() {
    let pool = common::setup_pool().await;
    let app = ledger_gate::api::create_router(pool);

    let json_body = serde_json::json!({
        "invoice_number": "RE-TEST-2",
        "amount": "50.00",
        "currency": "XYZ"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/invoices")
        .header("content-type", "application/json")
        .body(Body::from(json_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

/// Test 3 (Spec 005): Nach dem Anlegen erscheint die Rechnung in `GET /api/v1/invoices`.
#[tokio::test]
async fn test_list_invoices_after_create() {
    let pool = common::setup_pool().await;
    let app = ledger_gate::api::create_router(pool);

    let create_request = Request::builder()
        .method("POST")
        .uri("/api/v1/invoices")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({"invoice_number": "RE-TEST-3", "amount": "75.00", "currency": "EUR"}).to_string(),
        ))
        .unwrap();
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    assert_eq!(create_response.status(), StatusCode::CREATED);

    let list_request = Request::builder().method("GET").uri("/api/v1/invoices").body(Body::empty()).unwrap();
    let list_response = app.oneshot(list_request).await.unwrap();
    assert_eq!(list_response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX).await.unwrap();
    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(body_text.contains("RE-TEST-3"));
}
