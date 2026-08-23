#[path = "../common/mod.rs"]
mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

/// Integrationstest (Spec 007 Backend): eine Rechnung, die per REST angelegt wird, muss ein
/// `invoice_created` Event auf dem WS-Broadcast-Kanal auslösen. Da der `/ws` Handler direkt an
/// den globalen Broadcast-Channel gebunden ist, prüfen wir hier den Broadcast-Layer isoliert,
/// ohne eine echte TCP/WebSocket-Verbindung aufzubauen (siehe `ws::broadcast_event`).
#[tokio::test]
async fn creating_invoice_broadcasts_event_to_subscribers() {
    let pool = common::setup_pool().await;
    let app = ledger_gate::api::create_router(pool);

    // Wir abonnieren den internen Broadcast-Kanal, bevor die Anfrage gesendet wird.
    let mut subscription = ledger_gate::ws::subscribe();

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/invoices")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({"invoice_number": "RE-WS-1", "amount": "42.00", "currency": "EUR"}).to_string(),
        ))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let received = tokio::time::timeout(std::time::Duration::from_secs(2), subscription.recv())
        .await
        .expect("timed out waiting for broadcast event")
        .expect("broadcast channel closed unexpectedly");

    assert!(received.contains("invoice_created"));
    assert!(received.contains("RE-WS-1"));
}
