#[path = "../common/mod.rs"]
mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

/// Baut einen minimalen multipart/form-data Body mit einem Feld "file" von Hand,
/// damit wir keine zusätzliche Client-Bibliothek als Testabhängigkeit brauchen.
fn multipart_body(boundary: &str, filename: &str, content: &str) -> String {
    format!(
        "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"{filename}\"\r\nContent-Type: application/json\r\n\r\n{content}\r\n--{boundary}--\r\n"
    )
}

/// Test 1 (Spec 005): Ein Batch-Upload mit 2 gültigen + 1 ungültigem Eintrag -> inserted=2, skipped=1.
#[tokio::test]
async fn test_invoice_upload_partial_success() {
    let pool = common::setup_pool().await;
    let app = ledger_gate::api::create_router(pool);

    let payload = r#"[
        {"invoice_number":"RE-UP-1","issue_date":"2026-06-01","due_date":"2026-06-14","amount":"150.00","currency":"EUR"},
        {"invoice_number":"RE-UP-2","issue_date":"2026-06-01","due_date":"2026-06-14","amount":"300.50","currency":"EUR"},
        {"invoice_number":"RE-UP-3","issue_date":"2026-06-01","due_date":"2026-06-14","amount":"10.00","currency":"XYZ"}
    ]"#;

    let boundary = "TESTBOUNDARY";
    let body = multipart_body(boundary, "invoices.json", payload);

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/invoices/upload")
        .header("content-type", format!("multipart/form-data; boundary={boundary}"))
        .body(Body::from(body))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(json["inserted"], 2);
    assert_eq!(json["skipped"], 1);
}

/// Test 2 (Spec 005): Ein Transaktions-Upload matched eine bereits vorhandene offene Rechnung.
#[tokio::test]
async fn test_transaction_upload_auto_matches() {
    let pool = common::setup_pool().await;
    let app = ledger_gate::api::create_router(pool);

    let create_invoice = Request::builder()
        .method("POST")
        .uri("/api/v1/invoices")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({"invoice_number": "RE-UP-MATCH", "amount": "150.00", "currency": "EUR"}).to_string(),
        ))
        .unwrap();
    assert_eq!(app.clone().oneshot(create_invoice).await.unwrap().status(), StatusCode::CREATED);

    let payload = r#"[
        {"booking_date":"2026-06-05","value_date":"2026-06-05","amount":"150.00","currency":"EUR","reference_text":"Zahlung RE-UP-MATCH","counterparty_name":"Max Mustermann"}
    ]"#;
    let boundary = "TESTBOUNDARY2";
    let body = multipart_body(boundary, "transactions.json", payload);

    let upload_request = Request::builder()
        .method("POST")
        .uri("/api/v1/transactions/upload")
        .header("content-type", format!("multipart/form-data; boundary={boundary}"))
        .body(Body::from(body))
        .unwrap();
    let upload_response = app.clone().oneshot(upload_request).await.unwrap();
    assert_eq!(upload_response.status(), StatusCode::OK);

    let list_request = Request::builder().method("GET").uri("/api/v1/invoices").body(Body::empty()).unwrap();
    let list_response = app.oneshot(list_request).await.unwrap();
    let body_bytes = axum::body::to_bytes(list_response.into_body(), usize::MAX).await.unwrap();
    let body_text = String::from_utf8(body_bytes.to_vec()).unwrap();
    assert!(body_text.contains("\"status\":\"Paid\""));
}
