# Spezifikation: PostgreSQL Datenbank-Modelle & N2 Architektur

## 1. Übersicht
Als N2-System darf es keinen fest veränderbaren Status (`state`) in unseren Kern-Tabellen geben. Alle Tabellen loggen Fakten (Immutable Facts wo möglich). Der Rechnungsstatus (`Open`, `Paid`, `Overpaid`) wird über SQL-Views aus den Fakten abgeleitet.

## 2. Tabellen-Struktur

### 2.1 Table: `invoices`
Speichert den initialen Fakt der Rechnungsstellung.
- `id` (UUID, Primary Key)
- `invoice_number` (VARCHAR, Unique)
- `issue_date` (DATE)
- `due_date` (DATE)
- `amount` (NUMERIC(18,6)) -- Strikte Vermeidung von Floats
- `currency_code` (VARCHAR(3)) -- Z.B. 'EUR'
- `created_at` (TIMESTAMPTZ, Default NOW())
**KEIN Status-Feld!**

### 2.2 Table: `bank_transactions`
Speichert den initialen Fakt des Geldeingangs/-ausgangs.
- `id` (UUID, Primary Key)
- `booking_date` (DATE)
- `value_date` (DATE)
- `amount` (NUMERIC(18,6))
- `currency_code` (VARCHAR(3))
- `reference_text` (TEXT)
- `counterparty_name` (VARCHAR, nullable)
- `counterparty_iban` (VARCHAR, nullable)
- `created_at` (TIMESTAMPTZ, Default NOW())

### 2.3 Table: `allocations`
Speichert den Fakt, dass eine Zahlung (oder ein Teil davon) einer Rechnung zugewiesen wurde.
- `id` (UUID, Primary Key)
- `transaction_id` (UUID, Foreign Key -> `bank_transactions`)
- `invoice_id` (UUID, Foreign Key -> `invoices`)
- `allocated_amount` (NUMERIC(18,6))
- `created_at` (TIMESTAMPTZ)

## 3. Views

### 3.1 View: `invoice_statuses_view`
Diese View berechnet dynamisch für jede `invoice` die Summe aller gepaarten `allocated_amount` aus der Tabelle `allocations`.
**Logik:**
- `SUM(allocated_amount) >= amount` -> `Paid`
- `SUM(allocated_amount) > amount` -> `Overpaid`
- `SUM(allocated_amount) < amount` ODER `NULL` -> `Open`

## 4. Testing Obligations
- **DB Integration Test 1:** Insert einer Rechnung und einer Transaktion. Insert einer Allocation. Selektieren der `invoice_statuses_view` um zu prüfen, ob Postgres sauber `Paid` oder `Open` ableitet.
- Setzt voraus, dass Testcontainers zum Einsatz kommt (wie in N2/SDD-Architektur vorgegeben).

## 5. Freigabe
Bitte N2-Fakten-Struktur und View-Ansatz sichten und freigeben, bevor die passenden `sqlx` Migrations-Dateien erstellt werden!