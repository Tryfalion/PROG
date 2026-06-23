# Spezifikation: API Ingest (Axum REST)

## 1. Übersicht
Definiert den HTTP REST-Endpunkt, über den externe Software (Webhooks) oder unser Frontend neue Rechnungen ins System einspeisen kann.

## 2. API Format
- **POST `/api/v1/invoices`**
- **Content-Type**: `application/json`
- **Request Body Payload**:
  - `invoice_number` (String)
  - `amount` (Decimal)
  - `currency` (String/Enum)

## 3. Architekturvorgaben
Diese Route nimmt die JSON-Daten via `axum::Json` entgegen, validiert sie grundlegend (Parse auf Problemraum-Typen) und soll sie perspektivisch in der Db `store` ablegen (hier vorerst als Skeleton/Dummy Handler bis Store-Layer live ist).

## 4. Testing Obligations
- `api_smoke_test.rs`: Nutzt `axum::Router::oneshot` für einen In-Process Test (ohne echten Port). Schickt ein POST Request als `serde_json` Paylod an den Router und prüft, ob der Status `200 OK` (oder im Dummy `201 Created`) ist.