use axum::{extract::State, Json};
use serde::Serialize;
use sqlx::PgPool;

use crate::store;

/// Response-DTO für eine Rechnung inkl. abgeleitetem N2-Status (Spec 005).
#[derive(Serialize)]
pub struct InvoiceView {
    pub id: String,
    pub invoice_number: String,
    pub issue_date: String,
    pub due_date: String,
    pub amount: String,
    pub currency: String,
    pub status: String,
}

/// Response-DTO für eine Banktransaktion.
#[derive(Serialize)]
pub struct TransactionView {
    pub id: String,
    pub booking_date: String,
    pub value_date: String,
    pub amount: String,
    pub currency: String,
    pub reference_text: String,
    pub counterparty_name: Option<String>,
    pub counterparty_iban: Option<String>,
}

fn currency_label(c: &crate::problemraum::models::Currency) -> String {
    format!("{:?}", c)
}

/// Handler für 'GET /api/v1/invoices' (Spec 005): liefert alle Rechnungen mit N2-Status.
pub async fn list_invoices_handler(State(pool): State<PgPool>) -> Json<Vec<InvoiceView>> {
    let rows = store::get_invoices_with_status(&pool).await.unwrap_or_default();
    let views = rows
        .into_iter()
        .map(|(inv, status)| InvoiceView {
            id: inv.id.to_string(),
            invoice_number: inv.invoice_number,
            issue_date: inv.issue_date.to_string(),
            due_date: inv.due_date.to_string(),
            amount: inv.amount.to_string(),
            currency: currency_label(&inv.currency),
            status: format!("{:?}", status),
        })
        .collect();
    Json(views)
}

/// Handler für 'GET /api/v1/transactions' (Spec 005): liefert alle Banktransaktionen.
pub async fn list_transactions_handler(State(pool): State<PgPool>) -> Json<Vec<TransactionView>> {
    let rows = store::get_transactions(&pool).await.unwrap_or_default();
    let views = rows
        .into_iter()
        .map(|tx| TransactionView {
            id: tx.id.to_string(),
            booking_date: tx.booking_date.to_string(),
            value_date: tx.value_date.to_string(),
            amount: tx.amount.to_string(),
            currency: currency_label(&tx.currency),
            reference_text: tx.reference_text,
            counterparty_name: tx.counterparty_name,
            counterparty_iban: tx.counterparty_iban,
        })
        .collect();
    Json(views)
}
