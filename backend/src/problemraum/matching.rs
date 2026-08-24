use rust_decimal::Decimal;
use uuid::Uuid;

use super::models::{BankTransaction, Invoice};

/// Repräsentiert die Zuweisung eines Betrags von einer Transaktion zu einer Rechnung.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allocation {
    pub transaction_id: Uuid,
    pub invoice_id: Uuid,
    pub allocated_amount: Decimal,
}

/// Spezifische Fehler, die während des Matchings auftreten können.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchingError {
    /// Die Währungen von Transaktion und Rechnung stimmen nicht überein.
    CurrencyMismatch,
}

/// Versucht eine Banktransaktion auf eine spezifische Rechnung zuzuweisen,
/// basierend auf der 'invoice_number' im 'reference_text'.
///
/// Gibt eine `Allocation` zurück, wenn der Verwendungszweck passt,
/// andernfalls `None`. Bei einem Währungskonflikt wird ein Fehler geworfen.
pub fn match_transaction_to_invoice(
    tx: &BankTransaction,
    invoice: &Invoice,
) -> Result<Option<Allocation>, MatchingError> {
    // 1. Währungs-Check (Currency Mismatch)
    if tx.currency != invoice.currency {
        return Err(MatchingError::CurrencyMismatch);
    }

    // 2. Exaktes Matching prüfen (enthält der Verwendungszweck die Rechnungsnummer?)
    if !tx.reference_text.contains(&invoice.invoice_number) {
        return Ok(None);
    }

    // 3. Betrags-Logik bestimmen
    // Gemäß Spec 002: Wenn die Transaktion > Rechnung (Overpayment) ist,
    // weisen wir den GESAEMTEN Transaktionsbetrag zu, damit die Status-Logik (N2)
    // später in der View "Overpaid" ermittelt.
    let allocated_amount = tx.amount;

    Ok(Some(Allocation {
        transaction_id: tx.id,
        invoice_id: invoice.id,
        allocated_amount,
    }))
}

// -----------------------------------------------------------------------------
// TESTS
// Entsprechend der Spec 002 Testing Obligations
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use crate::problemraum::models::{Currency, InvoiceStatus};
    use chrono::NaiveDate;
    use rust_decimal_macros::dec;

    // Hilfsfunktion zum Erstellen einer Basis-Rechnung
    fn mock_invoice(amount: Decimal, currency: Currency) -> Invoice {
        Invoice {
            id: Uuid::new_v4(),
            invoice_number: "RE-100".to_string(),
            issue_date: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            due_date: NaiveDate::from_ymd_opt(2026, 1, 14).unwrap(),
            amount,
            currency,
            status: InvoiceStatus::Open,
        }
    }

    // Hilfsfunktion zum Erstellen einer Basis-Transaktion
    fn mock_tx(amount: Decimal, currency: Currency, reference: &str) -> BankTransaction {
        BankTransaction {
            id: Uuid::new_v4(),
            booking_date: NaiveDate::from_ymd_opt(2026, 1, 10).unwrap(),
            value_date: NaiveDate::from_ymd_opt(2026, 1, 10).unwrap(),
            amount,
            currency,
            reference_text: reference.to_string(),
            counterparty_name: None,
            counterparty_iban: None,
        }
    }

    /// Unit-Test 1: Exaktes Zahlen einer Rechnung.
    #[test]
    fn test_exact_match() {
        let invoice = mock_invoice(dec!(100.0), Currency::EUR);
        let tx = mock_tx(dec!(100.0), Currency::EUR, "Zahlung RE-100");

        let result = match_transaction_to_invoice(&tx, &invoice).unwrap().unwrap();
        assert_eq!(result.allocated_amount, dec!(100.0));
    }

    /// Unit-Test 2: Teilzahlung.
    #[test]
    fn test_partial_payment() {
        let invoice = mock_invoice(dec!(100.0), Currency::EUR);
        let tx = mock_tx(dec!(50.0), Currency::EUR, "Teil-Zahlung RE-100");

        let result = match_transaction_to_invoice(&tx, &invoice).unwrap().unwrap();
        assert_eq!(result.allocated_amount, dec!(50.0));
    }

    /// Unit-Test 3: Overpayment.
    #[test]
    fn test_overpayment() {
        let invoice = mock_invoice(dec!(100.0), Currency::EUR);
        let tx = mock_tx(dec!(150.0), Currency::EUR, "Ups zu viel RE-100");

        let result = match_transaction_to_invoice(&tx, &invoice).unwrap().unwrap();
        // Hier weisen wir den generellen Transaktionsbetrag zu
        assert_eq!(result.allocated_amount, dec!(150.0));
    }

    /// Unit-Test 4: Währungs-Abweisung.
    #[test]
    fn test_currency_mismatch() {
        let invoice = mock_invoice(dec!(100.0), Currency::EUR); // Rechnung in EUR
        let tx = mock_tx(dec!(110.0), Currency::USD, "RE-100 from US"); // Zahlung in USD

        let result = match_transaction_to_invoice(&tx, &invoice);
        assert_eq!(result.unwrap_err(), MatchingError::CurrencyMismatch);
    }
}
