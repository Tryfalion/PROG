use axum::{routing::post, Router};

pub mod ingest;

/// Erstellt den zentralen Router für alle Backend HTTP APIs.
pub fn create_router() -> Router {
    Router::new()
        // Registriere den Ingest-Endpoint aus Spec 004
        .route("/api/v1/invoices", post(ingest::create_invoice_handler))
}
