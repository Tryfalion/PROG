use std::{path::PathBuf, time::Duration};

use sqlx::PgPool;

/// Baut eine Postgres-Verbindung für Tests auf und wendet die Migration an.
/// Nutzt `DATABASE_URL` (Docker-Compose-Modus), sonst einen transienten Container
/// via `testcontainers` (lokaler Modus). Jeder Testlauf erhält eine frische DB, indem
/// alle Tabellen am Anfang geleert werden (`TRUNCATE ... RESTART IDENTITY CASCADE`).
pub async fn setup_pool() -> PgPool {
    let pool: PgPool = if let Ok(db_url) = std::env::var("DATABASE_URL") {
        connect_with_retry(&db_url).await
    } else {
        let docker = testcontainers::clients::Cli::default();
        let postgres_image = testcontainers_modules::postgres::Postgres::default();
        let node = docker.run(postgres_image);
        let port = node.get_host_port_ipv4(5432);
        // WICHTIG: `node` muss für die Laufzeit des Tests am Leben bleiben, sonst wird
        // der Container sofort gestoppt. Da wir den Container-Handle hier nicht an den
        // Aufrufer zurückgeben, "lecken" wir ihn absichtlich (nur für Testläufe relevant).
        std::mem::forget(node);
        connect_with_retry(&format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port)).await
    };

    apply_migration(&pool).await;
    reset_tables(&pool).await;
    pool
}

async fn connect_with_retry(url: &str) -> PgPool {
    let mut retries = 0usize;
    loop {
        match PgPool::connect(url).await {
            Ok(p) => return p,
            Err(_) if retries < 20 => {
                retries += 1;
                tokio::time::sleep(Duration::from_millis(500)).await;
            }
            Err(e) => panic!("Konnte keine Testdatenbank-Verbindung aufbauen: {e}"),
        }
    }
}

async fn apply_migration(pool: &PgPool) {
    let migration_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("migrations/20260623000001_core_n2_tables_and_views.sql");
    let sql = tokio::fs::read_to_string(&migration_path).await.expect("migration file must exist");

    for (idx, stmt) in sql.split(';').enumerate() {
        let sanitized = stmt
            .lines()
            .filter(|line| !line.trim_start().starts_with("--"))
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();

        if sanitized.is_empty() {
            continue;
        }

        sqlx::query(&sanitized)
            .execute(pool)
            .await
            .unwrap_or_else(|e| panic!("migration statement {} failed: {}\nSQL: {}", idx, e, sanitized));
    }
}

/// Leert alle Fakt-Tabellen, damit jeder Test mit einem sauberen Stand beginnt.
async fn reset_tables(pool: &PgPool) {
    sqlx::query("TRUNCATE allocations, bank_transactions, invoices RESTART IDENTITY CASCADE")
        .execute(pool)
        .await
        .unwrap_or_else(|e| panic!("reset_tables failed: {e}"));
}
