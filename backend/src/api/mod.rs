use axum::{routing::post, Router};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};

pub mod events;
pub mod invoices;
pub mod queries;
pub mod transactions;

/// Erstellt den zentralen Router für alle Backend HTTP APIs (Spec 004 & 005).
/// `pool` wird als geteilter Axum-State an alle Handler durchgereicht.
pub fn create_router(pool: PgPool) -> Router {
    // Erlaubt Aufrufe vom Vite Dev-Server (anderer Port) aus dem Frontend heraus.
    let cors = CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any);

    let api_routes = Router::new()
        .route(
            "/api/v1/invoices",
            post(invoices::create_invoice_handler).get(queries::list_invoices_handler),
        )
        .route("/api/v1/invoices/upload", post(invoices::upload_invoices_handler))
        .route(
            "/api/v1/transactions",
            post(transactions::create_transaction_handler).get(queries::list_transactions_handler),
        )
        .route("/api/v1/transactions/upload", post(transactions::upload_transactions_handler))
        .layer(cors)
        .with_state(pool);

    // Der WebSocket-Router (Spec 007) benötigt keinen DB-State und wird erst nach
    // `with_state` gemerged, damit beide Router-Typen übereinstimmen.
    api_routes.merge(crate::ws::create_ws_router())
}
