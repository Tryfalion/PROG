use std::{error::Error, path::PathBuf, time::Duration};

use chrono::NaiveDate;
use rust_decimal_macros::dec;
use sqlx::PgPool;
use uuid::Uuid;

// We import the application types and store helpers to run an end-to-end check
use ledger_gate::problemraum::{
    matching::Allocation,
    models::{Currency, Invoice, InvoiceStatus},
};
use ledger_gate::store::{get_invoices_with_status, insert_allocation, insert_invoice};

/// Integration test that supports two modes:
/// - If `DATABASE_URL` env var is set (docker-compose mode), the test connects to that DB.
/// - Otherwise it falls back to starting a transient Postgres container via `testcontainers`.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn postgres_container_integration() -> Result<(), Box<dyn Error>> {
    // Determine connection: prefer DATABASE_URL (useful for docker-compose), else start testcontainers
    let pool: PgPool = if let Ok(db_url) = std::env::var("DATABASE_URL") {
        // Wait for the DB to accept connections with a small retry loop
        let mut retries = 0usize;
        loop {
            match PgPool::connect(&db_url).await {
                Ok(p) => break p,
                Err(e) => {
                    if retries >= 20 {
                        return Err(Box::new(e));
                    }
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }
    } else {
        // Use testcontainers for local, isolated runs (requires Docker running too).
        let docker = testcontainers::clients::Cli::default();
        let postgres_image = testcontainers::images::postgres::Postgres::default();
        let node = docker.run(postgres_image);
        let port = node.get_host_port_ipv4(5432);
        let conn = format!("postgres://postgres:postgres@127.0.0.1:{}/postgres", port);

        let mut retries = 0usize;
        loop {
            match PgPool::connect(&conn).await {
                Ok(p) => break p,
                Err(e) => {
                    if retries >= 20 {
                        return Err(Box::new(e));
                    }
                    retries += 1;
                    tokio::time::sleep(Duration::from_millis(300)).await;
                }
            }
        }
    };

    // Apply the migration SQL file from the crate so the test mirrors real schema
    let migration_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "migrations/20260623000001_core_n2_tables_and_views.sql",
    );
    let sql = tokio::fs::read_to_string(&migration_path).await?;
    for stmt in sql.split(';') {
        let s = stmt.trim();
        if s.is_empty() || s.starts_with("--") {
            continue;
        }
        sqlx::query(s).execute(&pool).await?;
    }

    // Insert an Invoice using the store helper (ensures store layer wiring works)
    let invoice = Invoice {
        id: Uuid::new_v4(),
        invoice_number: "RE-TEST-001".to_string(),
        issue_date: NaiveDate::from_ymd_opt(2026, 6, 23).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2026, 7, 7).unwrap(),
        amount: dec!(100.00),
        currency: Currency::EUR,
        status: InvoiceStatus::Open,
    };
    insert_invoice(&pool, &invoice).await?;

    // Insert a bank transaction directly via SQL (no store helper exists for this yet)
    let tx_id = Uuid::new_v4();
    let booking = NaiveDate::from_ymd_opt(2026, 6, 24).unwrap();
    sqlx::query(
        r#"
        INSERT INTO bank_transactions 
            (id, booking_date, value_date, amount, currency_code, reference_text, counterparty_name, counterparty_iban)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
        "#,
    )
    .bind(tx_id)
    .bind(booking)
    .bind(booking)
    .bind(dec!(100.00))
    .bind("EUR")
    .bind("Payment RE-TEST-001")
    .bind(Some("John Doe"))
    .bind::<Option<String>, _>(None)
    .execute(&pool)
    .await?;

    // Create an Allocation via the store helper (this writes to allocations table)
    let alloc = Allocation {
        transaction_id: tx_id,
        invoice_id: invoice.id,
        allocated_amount: dec!(100.00),
    };
    insert_allocation(&pool, &alloc).await?;

    // Read invoices with derived status and assert invoice is Paid
    let result = get_invoices_with_status(&pool).await?;
    let found = result.into_iter().find(|(inv, _)| inv.id == invoice.id);
    assert!(found.is_some(), "Inserted invoice must appear in view");
    let (_inv, status) = found.unwrap();
    assert_eq!(status, InvoiceStatus::Paid);

    Ok(())
}
