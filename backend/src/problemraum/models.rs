use chrono::NaiveDate;
use rust_decimal::Decimal;
use uuid::Uuid;

/// Repräsentiert die unterstützten Währungen in unserem System.
/// Wir halten dies als Enum, um Typensicherheit zu garantieren und
/// ungültige Währungen wie "XYZ" bereits zur Compile-Zeit abzufangen.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Currency {
    EUR,
    USD,
}

/// Der abgeleitete Status einer Rechnung.
/// WICHTIG: Gemäß N2-Architektur wird dieser Zustand nicht hart in der
/// Datenbank gespeichert, sondern berechnet (z. B. durch SQL-Views
/// oder indem wir Zuweisungen im Speicher summieren).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvoiceStatus {
    /// Rechnung ist noch nicht oder nur teilweise bezahlt.
    Open,
    /// Rechnung ist exakt und vollständig bezahlt.
    Paid,
    /// Der zugewiesene Betrag übersteigt den Rechnungsbetrag (Special Case).
    Overpaid,
}

/// Eine Ausgangsrechnung, die ein Nutzer im System erfasst hat.
/// Diese Struktur agiert primär als Daten-Container im Problemraum (DDD).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invoice {
    /// Eindeutiger System-Identifikator (für Zuweisungen).
    pub id: Uuid,
    /// Vom Nutzer vergebene Rechnungsnummer (z. B. "RE-2026-001").
    pub invoice_number: String,
    /// Das Datum, an dem die Rechnung ausgestellt wurde.
    pub issue_date: NaiveDate,
    /// Das Datum, bis zu dem die Rechnung bezahlt werden muss.
    pub due_date: NaiveDate,
    /// Der Brutto-Rechnungsbetrag. Wir nutzen `Decimal`, um
    /// Rundungsfehler bei Finanz-Kalkulationen zu verhindern.
    pub amount: Decimal,
    /// Die verwendete Währung der Rechnung.
    pub currency: Currency,
    /// Der aktuelle Zustand. In der realen Anwendung oft abgeleitet.
    pub status: InvoiceStatus,
}

impl Invoice {
    /// Hilfsfunktion, um schnell zu prüfen, ob die Rechnung noch offen ist.
    /// Dies ist nützlich für Filter-Listen im Frontend oder Matching-Algorithmen.
    pub fn is_open(&self) -> bool {
        self.status == InvoiceStatus::Open
    }
}

/// Eine rohe Banktransaktion, die wir über einen Provider importiert haben.
/// Dies ist ein unveränderlicher Fakt (N2 Database Layer).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BankTransaction {
    /// Eindeutiger System-Identifikator.
    pub id: Uuid,
    /// Das Datum, an dem die Transaktion verbucht wurde.
    pub booking_date: NaiveDate,
    /// Das Datum der eigentlichen Wertstellung.
    pub value_date: NaiveDate,
    /// Der Transaktionsbetrag. Positiv für Einnahmen, negativ für Ausgaben.
    pub amount: Decimal,
    /// Die Währung der Banktransaktion.
    pub currency: Currency,
    /// Der Verwendungszweck. Dies ist unser Haupt-Datenfeld für das
    /// Rechnungs-Matching.
    pub reference_text: String,
    /// Der Name des Senders (optional, weil nicht immer vorhanden).
    pub counterparty_name: Option<String>,
    /// IBAN des Senders (optional).
    pub counterparty_iban: Option<String>,
}

// -----------------------------------------------------------------------------
// TESTS
// Die Testpflichten (Testing Obligations) aus der Spec werden hier direkt
// als In-File Unit-Tests umgesetzt.
// -----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec; // Hilfs-Makro für einfache Decimal-Erstellung

    /// Testet, ob eine Rechnung erfolgreich mit korrekten Typen erstellt wird.
    #[test]
    fn test_invoice_creation_and_status() {
        let invoice = Invoice {
            id: Uuid::new_v4(),
            invoice_number: "RE-2026-001".to_string(),
            issue_date: NaiveDate::from_ymd_opt(2026, 6, 23).unwrap(),
            due_date: NaiveDate::from_ymd_opt(2026, 7, 7).unwrap(),
            // Nutzung von dec! Makro ist sauberer und sicherer als from_f64!
            amount: dec!(150.50), 
            currency: Currency::EUR,
            status: InvoiceStatus::Open,
        };

        // Verifizieren, dass die `is_open` Funktion wie erwartet arbeitet.
        assert!(invoice.is_open(), "Rechnung sollte offen sein.");
        assert_eq!(invoice.amount.to_string(), "150.50");
    }

    /// Testet die Datenhaltung für eine importierte Banktransaktion.
    #[test]
    fn test_bank_transaction_creation() {
        let tx = BankTransaction {
            id: Uuid::new_v4(),
            booking_date: NaiveDate::from_ymd_opt(2026, 6, 24).unwrap(),
            value_date: NaiveDate::from_ymd_opt(2026, 6, 24).unwrap(),
            amount: dec!(150.50),
            currency: Currency::EUR,
            reference_text: "Rechnung RE-2026-001 Danke".to_string(),
            counterparty_name: Some("Erika Musterfrau".to_string()),
            counterparty_iban: None,
        };

        assert_eq!(tx.currency, Currency::EUR);
        assert!(tx.reference_text.contains("RE-2026-001"));
    }
}
