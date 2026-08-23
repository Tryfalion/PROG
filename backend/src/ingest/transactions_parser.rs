use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::Deserialize;
use uuid::Uuid;

use crate::problemraum::models::{BankTransaction, Currency};

/// Rohes JSON-Objekt, wie es in einer hochgeladenen Transaktions-Datei vorkommt
/// (gleiche Feldnamen wie `test_data/transactions_mock.json`).
#[derive(Debug, Deserialize)]
pub struct RawTransactionRecord {
    pub booking_date: NaiveDate,
    pub value_date: NaiveDate,
    pub amount: Decimal,
    pub currency: String,
    pub reference_text: String,
    pub counterparty_name: Option<String>,
    pub counterparty_iban: Option<String>,
}

/// Ergebnis eines Batch-Parse-Vorgangs, analog zu `ParsedInvoiceBatch`.
pub struct ParsedTransactionBatch {
    pub transactions: Vec<BankTransaction>,
    pub errors: Vec<String>,
}

/// Parst den Inhalt einer hochgeladenen Datei (JSON-Array von Banktransaktionen).
pub fn parse_transactions_json(bytes: &[u8]) -> ParsedTransactionBatch {
    let mut transactions = Vec::new();
    let mut errors = Vec::new();

    let raw_records: Vec<RawTransactionRecord> = match serde_json::from_slice(bytes) {
        Ok(records) => records,
        Err(e) => {
            errors.push(format!("Datei ist kein gültiges JSON-Array: {e}"));
            return ParsedTransactionBatch { transactions, errors };
        }
    };

    for record in raw_records {
        let currency = match record.currency.as_str() {
            "EUR" => Currency::EUR,
            "USD" => Currency::USD,
            other => {
                errors.push(format!(
                    "Transaktion '{}': unbekannte Währung '{other}' übersprungen",
                    record.reference_text
                ));
                continue;
            }
        };

        transactions.push(BankTransaction {
            id: Uuid::new_v4(),
            booking_date: record.booking_date,
            value_date: record.value_date,
            amount: record.amount,
            currency,
            reference_text: record.reference_text,
            counterparty_name: record.counterparty_name,
            counterparty_iban: record.counterparty_iban,
        });
    }

    ParsedTransactionBatch { transactions, errors }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_and_skips_invalid_currency() {
        let json = br#"[
            {"booking_date":"2026-06-05","value_date":"2026-06-05","amount":"150.00","currency":"EUR","reference_text":"RE-1","counterparty_name":"Max"},
            {"booking_date":"2026-06-05","value_date":"2026-06-05","amount":"10.00","currency":"XYZ","reference_text":"RE-2"}
        ]"#;

        let batch = parse_transactions_json(json);
        assert_eq!(batch.transactions.len(), 1);
        assert_eq!(batch.errors.len(), 1);
        assert_eq!(batch.transactions[0].reference_text, "RE-1");
    }
}
