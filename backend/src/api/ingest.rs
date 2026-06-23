use axum::{http::StatusCode, Json};
use rust_decimal::Decimal;
use serde::Deserialize;

use crate::problemraum::models::Currency;

/// Payload-Repräsentation aus Sicht des JSON Requests (Data Transfer Object).
#[derive(Debug, Deserialize)]
pub struct CreateInvoicePayload {
    pub invoice_number: String,
    pub amount: Decimal,
    pub currency: String,
}

/// Handler für 'POST /api/v1/invoices'.
/// Im Moment (V1) nimmt dies nur das JSON entgegen, validiert das Mapping auf Rust Typen 
/// und gibt 201 Created zurück, bis die Datenbank (Store-Schicht) eingebunden wird.
pub async fn create_invoice_handler(
    Json(payload): Json<CreateInvoicePayload>,
) -> StatusCode {
    // 1. Primitive Validierung (z.B. Währung parsen)
    let parsed_currency = match payload.currency.as_str() {
        "EUR" => Currency::EUR,
        "USD" => Currency::USD,
        _ => return StatusCode::BAD_REQUEST, // Ungültige Währung -> 400
    };

    // 2. Hier würde der Übergang in den Store (N2 Insert) folgen.
    // ... TODO: Insert Into Store

    StatusCode::CREATED
}
