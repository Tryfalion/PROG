#[path = "../common/mod.rs"]
mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

/// Test 1 (Spec 005): Eine gültige Transaktion wird angelegt -> 201 CREATED.
#[tokio::test]
async fn test_ingest_transaction_success() {
    let pool = common::setup_pool().await;
    let app = ledger_gate::api::create_router(pool);

    let json_body = serde_json::json!({
        "booking_date": "2026-06-05",
        "value_date": "2026-06-05",
        "amount": "150.00",
        "currency": "EUR",
        "reference_text": "Zahlung RE-TX-1",
        "counterparty_name": "Max Mustermann"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/transactions")
        .header("content-type", "application/json")
        .body(Body::from(json_body.to_string()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
}

/// Test 2 (Spec 005): Nach dem Anlegen erscheint die Transaktion in `GET /api/v1/transactions`.
#[tokio::test]
async fn test_list_transactions_after_create() {
    let pool = common::setup_pool().await;
    let app = ledger_gate::api::create_router(pool);

    let create_request = Request::builder()
        .method("POST")
        .uri("/api/v1/transactions")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "booking_date": "2026-06-05", "value_date": "2026-06-05", "amount": "20.00",
                "currency": "EUR", "reference_text": "RE-TX-2", "counterparty_name": null
            })
            .to_string(),
        ))
        .unwrap();
    assert_eq!(app.clone().oneshot(create_request).await.unwrap().status(), StatusCode::CREATED);

    let list_request = Request::builder().method("GET").uri("/api/v1/transactions").body(Body::empty()).unwrap();
    let list_response = app.oneshot(list_request).await.unwrap();
    assert_eq!(list_response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX).await.unwrap();
    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(body_text.contains("RE-TX-2"));
}

/// Test 3 (Spec 005): Eine eingehende Transaktion matched automatisch eine offene Rechnung
/// -> anschließend zeigt `GET /api/v1/invoices` den Status "Paid".
#[tokio::test]
async fn test_transaction_auto_matches_open_invoice() {
    let pool = common::setup_pool().await;
    let app = ledger_gate::api::create_router(pool);

    let create_invoice = Request::builder()
        .method("POST")
        .uri("/api/v1/invoices")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({"invoice_number": "RE-MATCH-1", "amount": "99.00", "currency": "EUR"}).to_string(),
        ))
        .unwrap();
    assert_eq!(app.clone().oneshot(create_invoice).await.unwrap().status(), StatusCode::CREATED);

    let create_tx = Request::builder()
        .method("POST")
        .uri("/api/v1/transactions")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({
                "booking_date": "2026-06-06", "value_date": "2026-06-06", "amount": "99.00",
                "currency": "EUR", "reference_text": "Zahlung RE-MATCH-1", "counterparty_name": null
            })
            .to_string(),
        ))
        .unwrap();
    assert_eq!(app.clone().oneshot(create_tx).await.unwrap().status(), StatusCode::CREATED);

    let list_request = Request::builder().method("GET").uri("/api/v1/invoices").body(Body::empty()).unwrap();
    let list_response = app.oneshot(list_request).await.unwrap();
    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX).await.unwrap();
    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(body_text.contains("\"invoice_number\":\"RE-MATCH-1\""));
    assert!(body_text.contains("\"status\":\"Paid\""));
}
