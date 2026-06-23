#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt; // Für die `oneshot` Methode im Router

    // Diese Tests greifen in einem echten Cargo Workspace auf `crate::api::create_router()` zu.
    // Wir setzen hierfür das Skeleton auf.
    #[tokio::test]
    async fn test_ingest_invoice_success() {
        let app = crate::api::create_router();

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
}
