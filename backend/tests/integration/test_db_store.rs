#[path = "../common/mod.rs"]
mod common;

use chrono::NaiveDate;
use rust_decimal_macros::dec;
use uuid::Uuid;

use ledger_gate::problemraum::matching::Allocation;
use ledger_gate::problemraum::models::{Currency, Invoice, InvoiceStatus};
use ledger_gate::store::{
    get_invoices_with_status, get_open_invoices, get_transactions, insert_allocation, insert_invoice,
    insert_transaction,
};

/// Store-Layer-Integrationstest (Spec 006): Rechnung + Transaktion + Allocation
/// schreiben und über die N2-View lesen -> Status muss "Paid" sein.
#[tokio::test]
async fn test_database_insert_and_view() {
    let pool = common::setup_pool().await;

    let invoice = Invoice {
        id: Uuid::new_v4(),
        invoice_number: "RE-STORE-1".to_string(),
        issue_date: NaiveDate::from_ymd_opt(2026, 6, 23).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2026, 7, 7).unwrap(),
        amount: dec!(100.00),
        currency: Currency::EUR,
        status: InvoiceStatus::Open,
    };
    insert_invoice(&pool, &invoice).await.unwrap();

    let tx = ledger_gate::problemraum::models::BankTransaction {
        id: Uuid::new_v4(),
        booking_date: NaiveDate::from_ymd_opt(2026, 6, 24).unwrap(),
        value_date: NaiveDate::from_ymd_opt(2026, 6, 24).unwrap(),
        amount: dec!(100.00),
        currency: Currency::EUR,
        reference_text: "Payment RE-STORE-1".to_string(),
        counterparty_name: Some("John Doe".to_string()),
        counterparty_iban: None,
    };
    insert_transaction(&pool, &tx).await.unwrap();

    let alloc = Allocation { transaction_id: tx.id, invoice_id: invoice.id, allocated_amount: dec!(100.00) };
    insert_allocation(&pool, &alloc).await.unwrap();

    let result = get_invoices_with_status(&pool).await.unwrap();
    let (_, status) = result.into_iter().find(|(inv, _)| inv.id == invoice.id).expect("invoice must exist");
    assert_eq!(status, InvoiceStatus::Paid);

    let open_invoices = get_open_invoices(&pool).await.unwrap();
    assert!(open_invoices.iter().all(|inv| inv.id != invoice.id), "bezahlte Rechnung darf nicht mehr offen sein");

    let transactions = get_transactions(&pool).await.unwrap();
    assert!(transactions.iter().any(|t| t.id == tx.id));
}

/// Zweiter Test: eine unbezahlte Rechnung bleibt in `get_open_invoices` sichtbar.
#[tokio::test]
async fn test_open_invoice_without_allocation_stays_open() {
    let pool = common::setup_pool().await;

    let invoice = Invoice {
        id: Uuid::new_v4(),
        invoice_number: "RE-STORE-2".to_string(),
        issue_date: NaiveDate::from_ymd_opt(2026, 6, 23).unwrap(),
        due_date: NaiveDate::from_ymd_opt(2026, 7, 7).unwrap(),
        amount: dec!(50.00),
        currency: Currency::EUR,
        status: InvoiceStatus::Open,
    };
    insert_invoice(&pool, &invoice).await.unwrap();

    let open_invoices = get_open_invoices(&pool).await.unwrap();
    assert!(open_invoices.iter().any(|inv| inv.id == invoice.id));
}
