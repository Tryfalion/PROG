use ledger_gate::api;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialisiere Tracing (Logging)
    tracing_subscriber::fmt::init();

    // Verbindung zur Datenbank aufbauen. Fällt auf einen lokalen Default zurück,
    // falls DATABASE_URL nicht gesetzt ist (z.B. lokale Entwicklung ohne Docker).
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/postgres".to_string());
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Konnte keine Verbindung zur Datenbank aufbauen");

    // Migrationen automatisch anwenden, damit Tabellen/Views beim Start existieren.
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Datenbank-Migrationen konnten nicht angewendet werden");

    // Setup Router
    let app = api::create_router(pool);

    // Start Server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tracing::info!("LedgerGate Backend listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
