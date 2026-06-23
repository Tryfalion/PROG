use axum::Router;
use ledger_gate::api;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialisiere Tracing (Logging)
    tracing_subscriber::fmt::init();

    // Setup Router
    let app = api::create_router();

    // Start Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("LedgerGate Backend listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
