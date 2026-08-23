use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::problemraum::models::{Currency, Invoice, InvoiceStatus};

/// Rohes JSON-Objekt, wie es in einer hochgeladenen Rechnungs-Datei vorkommt
/// (gleiche Feldnamen wie `test_data/invoices_mock.json`).
#[derive(Debug, Deserialize)]
pub struct RawInvoiceRecord {
    pub invoice_number: String,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub amount: Decimal,
    pub currency: String,
}

/// Ergebnis eines Batch-Parse-Vorgangs: erfolgreich umgewandelte Rechnungen sowie
/// eine Liste menschenlesbarer Fehlermeldungen zu übersprungenen Einträgen.
pub struct ParsedInvoiceBatch {
    pub invoices: Vec<Invoice>,
    pub errors: Vec<String>,
}

/// Parst den Inhalt einer hochgeladenen Datei (JSON-Array von Rechnungen).
/// Ungültige Einzel-Einträge (z.B. unbekannte Währung) werden übersprungen statt den
/// gesamten Batch abzubrechen (siehe Spec 005 "Verhalten & Regeln").
pub fn parse_invoices_json(bytes: &[u8]) -> ParsedInvoiceBatch {
    let mut invoices = Vec::new();
    let mut errors = Vec::new();

    let raw_records: Vec<RawInvoiceRecord> = match serde_json::from_slice(bytes) {
        Ok(records) => records,
        Err(e) => {
            errors.push(format!("Datei ist kein gültiges JSON-Array: {e}"));
            return ParsedInvoiceBatch { invoices, errors };
        }
    };

    for record in raw_records {
        let currency = match record.currency.as_str() {
            "EUR" => Currency::EUR,
            "USD" => Currency::USD,
            other => {
                errors.push(format!(
                    "Rechnung {}: unbekannte Währung '{other}' übersprungen",
                    record.invoice_number
                ));
                continue;
            }
        };

        invoices.push(Invoice {
            id: Uuid::new_v4(),
            invoice_number: record.invoice_number,
            issue_date: record.issue_date,
            due_date: record.due_date,
            amount: record.amount,
            currency,
            status: InvoiceStatus::Open,
        });
    }

    ParsedInvoiceBatch { invoices, errors }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_and_skips_invalid_currency() {
        let json = br#"[
            {"invoice_number":"RE-1","issue_date":"2026-06-01","due_date":"2026-06-14","amount":"150.00","currency":"EUR"},
            {"invoice_number":"RE-2","issue_date":"2026-06-01","due_date":"2026-06-14","amount":"10.00","currency":"XYZ"}
        ]"#;

        let batch = parse_invoices_json(json);
        assert_eq!(batch.invoices.len(), 1);
        assert_eq!(batch.errors.len(), 1);
        assert_eq!(batch.invoices[0].invoice_number, "RE-1");
    }

    #[test]
    fn rejects_malformed_json() {
        let batch = parse_invoices_json(b"not json");
        assert!(batch.invoices.is_empty());
        assert_eq!(batch.errors.len(), 1);
    }
}
