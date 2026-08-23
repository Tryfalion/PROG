use axum::{extract::Multipart, extract::State, http::StatusCode, Json};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::events::{notify_allocation_created, notify_transaction_created};
use crate::api::invoices::UploadResult;
use crate::ingest::transactions_parser::parse_transactions_json;
use crate::problemraum::matching::match_transaction_to_invoice;
use crate::problemraum::models::{BankTransaction, Currency};
use crate::store;

/// Payload für 'POST /api/v1/transactions' (Spec 005).
#[derive(Debug, Deserialize)]
pub struct CreateTransactionPayload {
    pub booking_date: chrono::NaiveDate,
    pub value_date: chrono::NaiveDate,
    pub amount: Decimal,
    pub currency: String,
    pub reference_text: String,
    pub counterparty_name: Option<String>,
    pub counterparty_iban: Option<String>,
}

/// Versucht die neu eingetroffene Transaktion automatisch gegen alle offenen Rechnungen
/// zu matchen (Spec 002/005). Speichert die erste passende Zuweisung und meldet sie per WS.
async fn try_auto_match(pool: &PgPool, tx: &BankTransaction) {
    let open_invoices = match store::get_open_invoices(pool).await {
        Ok(list) => list,
        Err(_) => return,
    };

    for invoice in open_invoices {
        if let Ok(Some(allocation)) = match_transaction_to_invoice(tx, &invoice) {
            if store::insert_allocation(pool, &allocation).await.is_ok() {
                notify_allocation_created(&allocation);
            }
            // Ein Treffer reicht für den Prototyp; weitere Rechnungen werden nicht geprüft.
            break;
        }
    }
}

/// Handler für 'POST /api/v1/transactions' (Spec 005).
pub async fn create_transaction_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTransactionPayload>,
) -> StatusCode {
    let parsed_currency = match payload.currency.as_str() {
        "EUR" => Currency::EUR,
        "USD" => Currency::USD,
        _ => return StatusCode::BAD_REQUEST,
    };

    let tx = BankTransaction {
        id: Uuid::new_v4(),
        booking_date: payload.booking_date,
        value_date: payload.value_date,
        amount: payload.amount,
        currency: parsed_currency,
        reference_text: payload.reference_text,
        counterparty_name: payload.counterparty_name,
        counterparty_iban: payload.counterparty_iban,
    };

    match store::insert_transaction(&pool, &tx).await {
        Ok(()) => {
            notify_transaction_created(&tx);
            try_auto_match(&pool, &tx).await;
            StatusCode::CREATED
        }
        Err(_) => StatusCode::CONFLICT,
    }
}

/// Handler für 'POST /api/v1/transactions/upload' (Spec 005): Multipart-Datei-Upload mit
/// einem JSON-Array von Banktransaktionen, inkl. automatischem Matching pro Eintrag.
pub async fn upload_transactions_handler(
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> (StatusCode, Json<UploadResult>) {
    let bytes = match multipart.next_field().await {
        Ok(Some(field)) => match field.bytes().await {
            Ok(b) => b,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(UploadResult { inserted: 0, skipped: 0, errors: vec![e.to_string()] }),
                )
            }
        },
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                Json(UploadResult {
                    inserted: 0,
                    skipped: 0,
                    errors: vec!["Keine Datei im Upload gefunden ('file' Feld erwartet)".to_string()],
                }),
            )
        }
    };

    let batch = parse_transactions_json(&bytes);
    let mut errors = batch.errors;
    let mut inserted = 0usize;

    for tx in batch.transactions {
        match store::insert_transaction(&pool, &tx).await {
            Ok(()) => {
                notify_transaction_created(&tx);
                try_auto_match(&pool, &tx).await;
                inserted += 1;
            }
            Err(e) => errors.push(format!("Transaktion '{}' konnte nicht gespeichert werden: {e}", tx.reference_text)),
        }
    }

    let skipped = errors.len();
    (StatusCode::OK, Json(UploadResult { inserted, skipped, errors }))
}
