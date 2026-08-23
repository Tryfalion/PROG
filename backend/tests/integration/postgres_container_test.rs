#[path = "../common/mod.rs"]
mod common;

use chrono::NaiveDate;
use rust_decimal_macros::dec;
use uuid::Uuid;

// We import the application types and store helpers to run an end-to-end check
use ledger_gate::problemraum::{
    matching::Allocation,
    models::{BankTransaction, Currency, Invoice, InvoiceStatus},
};
use ledger_gate::store::{get_invoices_with_status, insert_allocation, insert_invoice, insert_transaction};

/// Integration test that supports two modes:
/// - If `DATABASE_URL` env var is set (docker-compose mode), the test connects to that DB.
/// - Otherwise it falls back to starting a transient Postgres container via `testcontainers`.
/// Connection setup, migration and table reset are shared via `tests/common/mod.rs`.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn postgres_container_integration() {
    let pool = common::setup_pool().await;

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
    insert_invoice(&pool, &invoice).await.unwrap();

    // Insert a bank transaction using the store helper (Spec 005)
    let tx = BankTransaction {
        id: Uuid::new_v4(),
        booking_date: NaiveDate::from_ymd_opt(2026, 6, 24).unwrap(),
        value_date: NaiveDate::from_ymd_opt(2026, 6, 24).unwrap(),
        amount: dec!(100.00),
        currency: Currency::EUR,
        reference_text: "Payment RE-TEST-001".to_string(),
        counterparty_name: Some("John Doe".to_string()),
        counterparty_iban: None,
    };
    insert_transaction(&pool, &tx).await.unwrap();

    // Create an Allocation via the store helper (this writes to allocations table)
    let alloc = Allocation {
        transaction_id: tx.id,
        invoice_id: invoice.id,
        allocated_amount: dec!(100.00),
    };
    insert_allocation(&pool, &alloc).await.unwrap();

    // Read invoices with derived status and assert invoice is Paid
    let result = get_invoices_with_status(&pool).await.unwrap();
    let found = result.into_iter().find(|(inv, _)| inv.id == invoice.id);
    assert!(found.is_some(), "Inserted invoice must appear in view");
    let (_inv, status) = found.unwrap();
    assert_eq!(status, InvoiceStatus::Paid);
}
