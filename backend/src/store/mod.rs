use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::problemraum::models::{Currency, Invoice, InvoiceStatus};
use crate::problemraum::matching::Allocation;
use rust_decimal::Decimal;

/// Fügt eine Rechnung als reinen N2-Fakt ein (Ohne Status!)
pub async fn insert_invoice(pool: &PgPool, invoice: &Invoice) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO invoices (id, invoice_number, issue_date, due_date, amount, currency_code)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#
    )
    .bind(invoice.id)
    .bind(&invoice.invoice_number)
    .bind(invoice.issue_date)
    .bind(invoice.due_date)
    .bind(invoice.amount)
    .bind(format!("{:?}", invoice.currency)) // enum to string
    .execute(pool)
    .await?;

    Ok(())
}

/// Speichert das Allocation-Faktum (Die Bezahlung einer Rechnung)
pub async fn insert_allocation(pool: &PgPool, alloc: &Allocation) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO allocations (id, transaction_id, invoice_id, allocated_amount)
        VALUES ($1, $2, $3, $4)
        "#
    )
    .bind(Uuid::new_v4())
    .bind(alloc.transaction_id)
    .bind(alloc.invoice_id)
    .bind(alloc.allocated_amount)
    .execute(pool)
    .await?;

    Ok(())
}

/// Liest Rechnungen INKLUSIVE ihres N2-Status über die SQL-View aus.
pub async fn get_invoices_with_status(pool: &PgPool) -> Result<Vec<(Invoice, InvoiceStatus)>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT i.id, i.invoice_number, i.issue_date, i.due_date, i.amount, i.currency_code, v.status
        FROM invoices i
        JOIN invoice_statuses_view v ON i.id = v.invoice_id
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for row in rows {
        let status_str: String = row.get("status");
        let status = match status_str.as_str() {
            "Paid" => InvoiceStatus::Paid,
            "Overpaid" => InvoiceStatus::Overpaid,
            _ => InvoiceStatus::Open,
        };

        // Rekonstruktion (Für Demo extrem vereinfacht)
        let inv = Invoice {
            id: row.get("id"),
            invoice_number: row.get("invoice_number"),
            issue_date: row.get("issue_date"),
            due_date: row.get("due_date"),
            amount: row.get("amount"),
            currency: Currency::EUR, // Für den Prototyp hier hardcoded, im Prod Code parsing der spalte
            status: status.clone(),
        };

        result.push((inv, status));
    }
    Ok(result)
}
