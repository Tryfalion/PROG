# Spezifikation: Store Layer (PostgreSQL)

## 1. Übersicht
Der Store-Layer ist für das Schreiben und Lesen der N2-Fakten zuständig. Er abstrahiert die `sqlx` Datenbankzugriffe für das restliche Backend. Da SQL-Statements keine Laufzeitfehler werfen sollen, nutzen wir `sqlx` in Rust.

## 2. API des Repositories
- `insert_invoice`: Speichert eine offene Rechnung als Fakt.
- `insert_transaction`: Speichert einen Geldeingang/Ausgang als Fakt.
- `insert_allocation`: Mappt eine Transaktion zu einer Rechnung.
- `get_invoices_with_status`: Liest aus der View `invoice_statuses_view` und parst das Mapping auf `Invoice` & `InvoiceStatus`.

## 3. Testing Obligations
- `test_db_store.rs`: Ein **Integration Test**, der mittels `testcontainers` eine frische PostgreSQL-Instanz hochfährt, die Migrations (`migrations/`) anwendet, Test-Daten einfügt, ein Allocation-Mapping speichert und prüft, ob die SQL-View den korrekten Rechnungs-Status (Paid/Overpaid/Open) berechnet und an Rust zurückspielt.