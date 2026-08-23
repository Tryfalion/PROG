use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::problemraum::models::{BankTransaction, Currency, Invoice, InvoiceStatus};
use crate::problemraum::matching::Allocation;

/// Wandelt unseren Currency-Enum in den in der DB gespeicherten String um (z.B. "EUR").
/// Zentral an einer Stelle gehalten, damit Insert und Read immer das gleiche Format nutzen.
fn currency_to_code(currency: &Currency) -> &'static str {
    match currency {
        Currency::EUR => "EUR",
        Currency::USD => "USD",
    }
}

/// Wandelt einen in der DB gespeicherten Währungscode zurück in unseren Enum.
/// Unbekannte Codes fallen auf EUR zurück (Prototyp-Vereinfachung, siehe Spec 003).
fn code_to_currency(code: &str) -> Currency {
    match code {
        "USD" => Currency::USD,
        _ => Currency::EUR,
    }
}

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
    .bind(currency_to_code(&invoice.currency))
    .execute(pool)
    .await?;

    Ok(())
}

/// Fügt eine Banktransaktion als reinen N2-Fakt ein (Spec 005).
pub async fn insert_transaction(pool: &PgPool, tx: &BankTransaction) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO bank_transactions
            (id, booking_date, value_date, amount, currency_code, reference_text, counterparty_name, counterparty_iban)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#
    )
    .bind(tx.id)
    .bind(tx.booking_date)
    .bind(tx.value_date)
    .bind(tx.amount)
    .bind(currency_to_code(&tx.currency))
    .bind(&tx.reference_text)
    .bind(&tx.counterparty_name)
    .bind(&tx.counterparty_iban)
    .execute(pool)
    .await?;

    Ok(())
}

/// Liest alle importierten Banktransaktionen aus (Spec 005), neueste zuerst.
pub async fn get_transactions(pool: &PgPool) -> Result<Vec<BankTransaction>, sqlx::Error> {
    let rows = sqlx::query(
        r#"
        SELECT id, booking_date, value_date, amount, currency_code, reference_text, counterparty_name, counterparty_iban
        FROM bank_transactions
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for row in rows {
        let currency_code: String = row.get("currency_code");
        result.push(BankTransaction {
            id: row.get("id"),
            booking_date: row.get("booking_date"),
            value_date: row.get("value_date"),
            amount: row.get("amount"),
            currency: code_to_currency(&currency_code),
            reference_text: row.get("reference_text"),
            counterparty_name: row.get("counterparty_name"),
            counterparty_iban: row.get("counterparty_iban"),
        });
    }
    Ok(result)
}

/// Liest alle Rechnungen, deren N2-Status aktuell "Open" ist.
/// Wird beim automatischen Matching neu eintreffender Transaktionen genutzt (Spec 005).
pub async fn get_open_invoices(pool: &PgPool) -> Result<Vec<Invoice>, sqlx::Error> {
    let all = get_invoices_with_status(pool).await?;
    Ok(all
        .into_iter()
        .filter(|(_, status)| *status == InvoiceStatus::Open)
        .map(|(inv, _)| inv)
        .collect())
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

        // Rekonstruktion der Rechnung inkl. dem in der DB abgelegten Währungscode.
        let currency_code: String = row.get("currency_code");
        let inv = Invoice {
            id: row.get("id"),
            invoice_number: row.get("invoice_number"),
            issue_date: row.get("issue_date"),
            due_date: row.get("due_date"),
            amount: row.get("amount"),
            currency: code_to_currency(&currency_code),
            status: status.clone(),
        };

        result.push((inv, status));
    }
    Ok(result)
}
