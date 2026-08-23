# Spec 005 — Transaktions-Ingest, Datei-Upload & Abfrage-Endpunkte

## Kontext
Spec 004 deckt nur `POST /api/v1/invoices` ab. Für die geforderte Drag-&-Drop-Upload-Funktion
im Frontend ("Add Invoice" / "Add Payment") und für die Anzeige von Daten im Frontend fehlen:
Banktransaktionen anlegen, Listen abrufen, und Datei-Uploads (JSON-Dateien mit mehreren
Einträgen, wie sie z. B. aus `test_data/*.json` exportiert werden).

## Ziel / Scope
1. `POST /api/v1/transactions` — legt eine einzelne Banktransaktion als N2-Fakt an und
   versucht danach automatisch ein Matching gegen offene Rechnungen (Spec 002).
2. `GET /api/v1/invoices` — liefert alle Rechnungen inkl. abgeleitetem Status (N2-View).
3. `GET /api/v1/transactions` — liefert alle importierten Banktransaktionen.
4. `POST /api/v1/invoices/upload` — Multipart-Datei-Upload (Feld `file`), JSON-Array von
   Rechnungsobjekten (gleiche Form wie `CreateInvoicePayload`, siehe Spec 004). Jeder Eintrag
   wird einzeln validiert und gespeichert; fehlerhafte Einträge werden übersprungen und in der
   Antwort als `skipped` gemeldet (kein Abbruch des gesamten Batches).
5. `POST /api/v1/transactions/upload` — Multipart-Datei-Upload (Feld `file`), JSON-Array von
   Transaktionsobjekten. Analoges Verhalten wie oben, inkl. automatischem Matching pro Eintrag.

## Datenformat Upload-Datei
Invoices (JSON-Array):
```json
[{"invoice_number": "RE-1", "issue_date": "2026-06-01", "due_date": "2026-06-14", "amount": "150.00", "currency": "EUR"}]
```
Transactions (JSON-Array):
```json
[{"booking_date": "2026-06-05", "value_date": "2026-06-05", "amount": "150.00", "currency": "EUR", "reference_text": "RE-1", "counterparty_name": "Max Mustermann"}]
```

## Verhalten & Regeln
- Jede erfolgreiche Erstellung (Invoice oder Transaction) löst ein WebSocket-Broadcast-Event aus
  (`{"type": "invoice_created", ...}` bzw. `{"type": "transaction_created", ...}`), damit das
  Frontend live aktualisiert wird (Spec 007).
- Beim Anlegen einer Transaktion wird serverseitig gegen alle offenen Rechnungen (Status `Open`)
  mit `match_transaction_to_invoice` (Spec 002) geprüft. Bei einem Treffer wird automatisch eine
  `Allocation` gespeichert und ein `allocation_created` Event gesendet.
- Ungültige Währungscodes → `400 Bad Request` (Einzel-Endpunkte) bzw. Eintrag wird beim
  Batch-Upload übersprungen.
- Upload-Antwort: `{"inserted": <n>, "skipped": <n>, "errors": [<string>, ...]}` mit Status `200`.
- Doppelte `invoice_number` (DB `UNIQUE` Constraint) führen zu einem übersprungenen Eintrag im
  Batch bzw. `409 Conflict` im Einzel-Endpunkt.

## Testpflichten
- Unit: keine neue reine Domänenlogik (Wiederverwendung von Spec 002 Matching).
- API-Smoke-Tests (Axum `oneshot`, gegen Test-Postgres via `DATABASE_URL`):
  - Transaktion einzeln anlegen → `201`.
  - Rechnungsliste abrufen nach Insert → enthält Eintrag mit korrektem Status.
  - Transaktionsliste abrufen nach Insert → enthält Eintrag.
  - Invoice-Upload mit 2 gültigen + 1 ungültigem Eintrag → `inserted=2, skipped=1`.
  - Transaction-Upload, bei dem eine Transaktion exakt eine offene Rechnung matched → nach dem
    Upload zeigt `GET /api/v1/invoices` die Rechnung als `Paid`.
