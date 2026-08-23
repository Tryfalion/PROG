use axum::{extract::Multipart, extract::State, http::StatusCode, Json};
use rust_decimal::Decimal;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use crate::api::events::notify_invoice_created;
use crate::ingest::invoices_parser::parse_invoices_json;
use crate::problemraum::models::{Currency, Invoice, InvoiceStatus};
use crate::store;

/// Payload-Repräsentation aus Sicht des JSON Requests (Data Transfer Object).
#[derive(Debug, Deserialize)]
pub struct CreateInvoicePayload {
    pub invoice_number: String,
    pub amount: Decimal,
    pub currency: String,
}

/// Handler für 'POST /api/v1/invoices' (Spec 004).
/// Validiert das JSON, wandelt es in unseren Domänen-Typ um und speichert es als N2-Fakt.
pub async fn create_invoice_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateInvoicePayload>,
) -> StatusCode {
    // 1. Primitive Validierung (z.B. Währung parsen)
    let parsed_currency = match payload.currency.as_str() {
        "EUR" => Currency::EUR,
        "USD" => Currency::USD,
        _ => return StatusCode::BAD_REQUEST, // Ungültige Währung -> 400
    };

    // 2. Domänen-Objekt bauen. Für die V1-Demo nutzen wir "heute" als Ausstellungs-
    // und Fälligkeitsdatum, da das Frontend diese Felder aktuell nicht mitschickt.
    let today = chrono::Utc::now().date_naive();
    let invoice = Invoice {
        id: Uuid::new_v4(),
        invoice_number: payload.invoice_number,
        issue_date: today,
        due_date: today,
        amount: payload.amount,
        currency: parsed_currency,
        status: InvoiceStatus::Open,
    };

    // 3. Persistieren. Ein doppelter invoice_number Unique-Constraint-Verstoß -> 409.
    match store::insert_invoice(&pool, &invoice).await {
        Ok(()) => {
            notify_invoice_created(&invoice);
            StatusCode::CREATED
        }
        Err(_) => StatusCode::CONFLICT,
    }
}

/// Antwort-Format für Batch-Uploads (Spec 005).
#[derive(Debug, serde::Serialize)]
pub struct UploadResult {
    pub inserted: usize,
    pub skipped: usize,
    pub errors: Vec<String>,
}

/// Handler für 'POST /api/v1/invoices/upload' (Spec 005): Multipart-Datei-Upload,
/// erwartet ein Feld "file" mit einem JSON-Array von Rechnungen (Drag & Drop im Frontend).
pub async fn upload_invoices_handler(
    State(pool): State<PgPool>,
    mut multipart: Multipart,
) -> (StatusCode, Json<UploadResult>) {
    // Wir lesen den ersten Datei-Teil des Multipart-Requests aus.
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

    let batch = parse_invoices_json(&bytes);
    let mut errors = batch.errors;
    let mut inserted = 0usize;

    for invoice in batch.invoices {
        match store::insert_invoice(&pool, &invoice).await {
            Ok(()) => {
                notify_invoice_created(&invoice);
                inserted += 1;
            }
            Err(e) => errors.push(format!(
                "Rechnung {} konnte nicht gespeichert werden (evtl. Duplikat): {e}",
                invoice.invoice_number
            )),
        }
    }

    let skipped = errors.len();
    (StatusCode::OK, Json(UploadResult { inserted, skipped, errors }))
}
