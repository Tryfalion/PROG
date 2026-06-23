# Spezifikation: Kern-Datenstrukturen (Domain Models) für Rechnungen und Banktransaktionen

## 1. Übersicht
Diese Spezifikation definiert die reinen Rust-Domänenmodelle für `Invoice` (Rechnung) und `BankTransaction` (Bank-Transaktion) im `problemraum`-Modul. Gemäß der Architekturgaben nutzen wir strikt `rust_decimal` für Geldbeträge (keine Floats) und setzen auf Domain-Driven Design (DDD).

## 2. Constraints & Architekturgaben
- **Währungen & Beträge:** Verwendung von `rust_decimal::Decimal`. Floats sind strikt verboten. Dies verhindert Rundungsfehler bei Finanzdaten.
- **IDs:** Verwendung von `uuid::Uuid` für alle systemweiten Identifikatoren.
- **Zeitstempel/Daten:** Verwendung von `chrono::NaiveDate` für kalendarische Buchungs-/Rechnungsdaten und `chrono::DateTime<Utc>` für Systemzeitstempel (z.B. Erstellung).
- **Zustandsableitung (N2):** Die Domänenmodelle repräsentieren den reinen Zustand in Rust. Später wird der `InvoiceStatus` nicht als hartes Feld in der DB gespeichert, sondern in Rust basierend auf Zuweisungen (Allocations) abgeleitet oder über SQL-Views (N2) befüllt. Für das reine Rust-Typ-Modell definieren wir ihn aber als Enum mit, um Geschäftslogik abbilden zu können.

## 3. Datenstrukturen (Problem Space)

### 3.1. Currency (Enum)
Eine Aufzählung der unterstützten Währungen.
- `EUR`
- `USD`
- (Erweiterbar)

### 3.2. InvoiceStatus (Enum)
Der berechnete oder auslesbare Zustand einer Rechnung.
- `Open` (Noch nicht oder nicht vollständig bezahlt)
- `Paid` (Vollständig bezahlt)
- `Overpaid` (Zu viel bezahlt - Special Case)

### 3.3. Invoice (Struct)
Repräsentiert eine Ausgangsrechnung im System.
- `id`: `Uuid` (Eindeutige System-ID)
- `invoice_number`: `String` (Die vom Nutzer vergebene Rechnungsnummer)
- `issue_date`: `NaiveDate` (Ausstellungsdatum)
- `due_date`: `NaiveDate` (Fälligkeitsdatum)
- `amount`: `Decimal` (Brutto-Betrag der Rechnung)
- `currency`: `Currency` (Währung)
- `status`: `InvoiceStatus` (Der aktuelle Status der Rechnung)

### 3.4. BankTransaction (Struct)
Repräsentiert eine tatsächliche Kontobewegung (Income oder Expense).
- `id`: `Uuid` (Eindeutige System-ID)
- `booking_date`: `NaiveDate` (Datum der Buchung auf dem Bankkonto)
- `value_date`: `NaiveDate` (Wertstellungsdatum)
- `amount`: `Decimal` (Betrag; positiv für Zahlungseingänge, negativ für Ausgaben)
- `currency`: `Currency` (Währung)
- `reference_text`: `String` (Verwendungszweck, wichtig für das spatere Matching)
- `counterparty_name`: `Option<String>` (Name des Auftraggebers/Empfängers, optional da nicht immer von Banken geliefert)
- `counterparty_iban`: `Option<String>` (IBAN der Gegenseite, optional)

## 4. Testing Obligations (Test-Pflichten)

Bevor oder während diese Structs implementiert werden, müssen folgende Unit-Tests im Problem Space geschrieben werden:
1. **Initialisierung & Typ-Validierung:** Tests, die sicherstellen, dass `Invoice` und `BankTransaction` korrekt mit gültigen `rust_decimal::Decimal` Beträgen instanziiert werden können.
2. **Status-Logik (sofern als Methoden implementiert):** Unit-Tests für etwaige Hilfsfunktionen (z. B. eine Methode `is_open() -> bool` auf `Invoice`, welche den Status prüft).
3. (Integration-Tests für die DB fallen hier noch weg, da diese Schicht noch komplett N2-frei (ohne DB-Infrastruktur) entwickelt wird).

## 5. Freigabe
Bitte lies dir diese Spezifikation durch. Sobald du das "Go" gibst, werde ich im Rahmen des Spec-Driven Developments:
1. Die Unit-Tests in einem `problemraum`-Testmodul anlegen.
2. Die reinen Rust-Structs (inklusive Dokumentation) implementieren.