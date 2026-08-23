# LedgerGate — Backend-Dokumentationshandbuch

Dieses Dokument erklärt **Datei für Datei und Funktion für Funktion**, wie eine Anfrage durch das
Backend läuft. Ziel: den kompletten Prozess nachvollziehen können, ohne selbst grep/suchen zu
müssen. Ergänzt `copilot-instructions.md` (Architekturprinzipien) und `docs/DEVELOPER_GUIDE.md`
(Spec-Übersicht).

## 1. Startvorgang (`main.rs`)

`backend/src/main.rs::main()` läuft in dieser Reihenfolge ab:

1. `tracing_subscriber::fmt::init()` — aktiviert Logging (`tracing::info!` etc. werden sichtbar).
2. Liest `DATABASE_URL` aus der Umgebung (Fallback: lokaler Default-Connection-String).
3. `PgPoolOptions::new().connect(&database_url)` — baut den Connection-Pool auf (`sqlx::PgPool`).
4. `sqlx::migrate!("./migrations").run(&pool)` — wendet alle SQL-Dateien aus
   `backend/migrations/` an (aktuell: `20260623000001_core_n2_tables_and_views.sql`, erstellt
   `invoices`, `bank_transactions`, `allocations` und die View `invoice_statuses_view`).
5. `api::create_router(pool)` — baut den kompletten HTTP-Router (siehe Abschnitt 2).
6. `axum::serve(listener, app)` — startet den Server auf `0.0.0.0:3000`.

## 2. Router-Aufbau (`api/mod.rs::create_router`)

`create_router(pool: PgPool) -> Router` macht drei Dinge:

1. Registriert alle REST-Routen (Tabelle unten) und hängt eine offene CORS-Policy an
   (`tower_http::cors::CorsLayer` mit `Any`), damit das Frontend (anderer Port) Anfragen senden darf.
2. Ruft `.with_state(pool)` auf — ab hier ist der `PgPool` per `State<PgPool>`-Extractor in jedem
   Handler verfügbar (siehe `axum::extract::State`).
3. Merged den WebSocket-Router aus `ws::create_ws_router()` (stateless) in den Haupt-Router.

| Route | Methode | Handler | Datei |
|---|---|---|---|
| `/api/v1/invoices` | POST | `invoices::create_invoice_handler` | `api/invoices.rs` |
| `/api/v1/invoices` | GET | `queries::list_invoices_handler` | `api/queries.rs` |
| `/api/v1/invoices/upload` | POST | `invoices::upload_invoices_handler` | `api/invoices.rs` |
| `/api/v1/transactions` | POST | `transactions::create_transaction_handler` | `api/transactions.rs` |
| `/api/v1/transactions` | GET | `queries::list_transactions_handler` | `api/queries.rs` |
| `/api/v1/transactions/upload` | POST | `transactions::upload_transactions_handler` | `api/transactions.rs` |
| `/ws` | GET (Upgrade) | `ws_handler` → `handle_socket` | `ws.rs` |

## 3. Ablauf: Rechnung einzeln anlegen (`POST /api/v1/invoices`)

Aufrufkette, Schritt für Schritt:

1. Axum extrahiert `State<PgPool>` und deserialisiert den JSON-Body in
   `invoices::CreateInvoicePayload { invoice_number, amount, currency }`
   (`api/invoices.rs::create_invoice_handler`).
2. Die Währung (`payload.currency: String`) wird in den Domänen-Typ
   `problemraum::models::Currency` übersetzt (`"EUR"`/`"USD"` → Enum-Variante). Ungültige Werte
   führen sofort zu `400 Bad Request` — es wird **nichts** in die Datenbank geschrieben.
3. Ein `problemraum::models::Invoice`-Wert wird gebaut (neue `Uuid`, `issue_date`/`due_date` =
   heutiges Datum als V1-Vereinfachung, Status `InvoiceStatus::Open`).
4. `store::insert_invoice(&pool, &invoice)` führt das `INSERT INTO invoices (...)` aus
   (`store.rs`). Ein Verstoß gegen den `UNIQUE`-Constraint auf `invoice_number` (doppelte
   Rechnung) liefert einen `sqlx::Error` zurück → Handler antwortet mit `409 Conflict`.
5. Bei Erfolg: `api::events::notify_invoice_created(&invoice)` baut ein JSON-Event
   `{"type": "invoice_created", "invoice_number": ..., "amount": ...}` und ruft
   `ws::broadcast_event(json)` auf — das schickt die Nachricht an **alle** aktuell verbundenen
   WebSocket-Clients (siehe Abschnitt 6).
6. Handler antwortet mit `201 Created`.

## 4. Ablauf: Zahlung anlegen + automatisches Matching (`POST /api/v1/transactions`)

Datei: `api/transactions.rs::create_transaction_handler`.

1. Wie oben: Payload deserialisieren, Währung validieren (`400` bei Fehler).
2. `problemraum::models::BankTransaction` bauen, `store::insert_transaction(&pool, &tx)`
   ausführen (`store.rs`, `INSERT INTO bank_transactions`).
3. Bei Erfolg: `notify_transaction_created(&tx)` broadcastet `"transaction_created"`.
4. **Automatisches Matching** — `try_auto_match(&pool, &tx)` wird aufgerufen:
   a. `store::get_open_invoices(&pool)` liest alle Rechnungen, deren N2-Status (aus der View
      `invoice_statuses_view`) aktuell `"Open"` ist (`store.rs::get_open_invoices` filtert das
      Ergebnis von `get_invoices_with_status`).
   b. Für jede offene Rechnung wird `problemraum::matching::match_transaction_to_invoice(tx, invoice)`
      aufgerufen (reine Domänenfunktion, keine DB-Abhängigkeit, siehe Spec 002):
      - Prüft zuerst Währungsgleichheit (sonst `MatchingError::CurrencyMismatch`, wird ignoriert —
        `if let Ok(Some(...))` überspringt Fehler-Ergebnisse).
      - Prüft, ob `reference_text` die `invoice_number` enthält.
      - Falls ja: baut eine `Allocation { transaction_id, invoice_id, allocated_amount = tx.amount }`.
   c. Beim ersten Treffer: `store::insert_allocation(&pool, &allocation)` schreibt
      `INSERT INTO allocations`, danach `notify_allocation_created(&allocation)` broadcastet
      `"allocation_created"`. Danach `break` — nur eine Zuordnung pro eingehender Zahlung (V1).
5. Handler antwortet mit `201 Created` (unabhängig davon, ob ein Match gefunden wurde).

**Warum ändert sich der Rechnungsstatus, ohne dass jemand ihn explizit setzt?** Der Status ist
*abgeleitet* (N2-Prinzip): Die SQL-View `invoice_statuses_view`
(`backend/migrations/20260623000001_core_n2_tables_and_views.sql`) summiert bei jeder Abfrage
`SUM(allocated_amount)` über alle `allocations` einer Rechnung und vergleicht sie live mit dem
Rechnungsbetrag (`Open` / `Paid` / `Overpaid`). Es gibt keine Spalte "status" in der Tabelle
`invoices` selbst.

## 5. Ablauf: Datei-Upload (`POST /api/v1/invoices/upload` bzw. `/transactions/upload`)

Datei: `api/invoices.rs::upload_invoices_handler` / `api/transactions.rs::upload_transactions_handler`.

1. `axum::extract::Multipart` liest das erste Formularfeld (erwartet: `file`) und dessen
   Rohbytes (`field.bytes().await`). Fehlt die Datei → `400 Bad Request`.
2. Die Bytes werden an die **Ingest-Schicht** übergeben (bewusst von HTTP entkoppelt, damit sie
   ohne Axum getestet werden kann):
   - `ingest::invoices_parser::parse_invoices_json(&bytes)` bzw.
     `ingest::transactions_parser::parse_transactions_json(&bytes)`.
   - Diese Funktionen deserialisieren ein JSON-Array in `RawInvoiceRecord`/`RawTransactionRecord`,
     übersetzen jeden Eintrag einzeln in den Domänen-Typ und sammeln Fehler statt abzubrechen
     (`ParsedInvoiceBatch { invoices, errors }`).
3. Für jeden erfolgreich geparsten Eintrag: derselbe Store-Aufruf wie bei den Einzel-Endpunkten
   (`store::insert_invoice` / `store::insert_transaction`), inkl. WS-Broadcast pro Eintrag und
   (bei Transaktionen) `try_auto_match` pro Eintrag.
4. Rückgabe: `UploadResult { inserted, skipped, errors }` als JSON mit Status `200 OK`.

## 6. Ablauf: Echtzeit-Updates (WebSocket, `ws.rs`)

1. `ws::create_ws_router()` registriert `GET /ws` → `ws_handler`, der die HTTP-Verbindung per
   `WebSocketUpgrade::on_upgrade(handle_socket)` in eine WebSocket-Verbindung umwandelt.
2. Es gibt **einen einzigen** globalen Broadcast-Kanal (`tokio::sync::broadcast::Sender<String>`,
   `lazy_static`, Kapazität 100 Nachrichten) — jede Server-Instanz teilt sich diesen In-Memory-Kanal.
3. `handle_socket(socket)` abonniert den Kanal (`TX.subscribe()`) und leitet **jede** Nachricht,
   die irgendwo im Backend per `ws::broadcast_event(json)` gesendet wird, an genau diesen einen
   Client weiter (`socket.send(Message::Text(msg))`). Jeder verbundene Client bekommt also alle
   Events, unabhängig davon, wer sie ausgelöst hat.
4. `ws::subscribe()` ist eine zusätzliche, nur intern/testseitig genutzte Funktion, um den Kanal
   ohne echte WebSocket-Verbindung zu abonnieren (siehe `tests/integration/test_ws_broadcast.rs`).
5. Wer ruft `broadcast_event` auf? Ausschließlich `api::events::notify_*` (siehe Abschnitt 3–5) —
   die Handler selbst kennen das Wire-Format nicht, sie rufen nur `notify_invoice_created(&invoice)`
   etc. auf. Das entkoppelt "was passiert ist" (Domänen-Ereignis) von "wie es über die Leitung
   aussieht" (JSON-Struktur in `api/events.rs`).

## 7. Ablauf: Rechnungen/Transaktionen lesen (`GET /api/v1/invoices`, `GET /api/v1/transactions`)

Datei: `api/queries.rs`.

1. `list_invoices_handler` ruft `store::get_invoices_with_status(&pool)` auf — das joint
   `invoices` mit der View `invoice_statuses_view` und mappt jede Zeile über
   `store::code_to_currency` zurück in `problemraum::models::Currency`.
2. Jedes `(Invoice, InvoiceStatus)`-Paar wird in ein `queries::InvoiceView` (reines Serde-DTO mit
   `String`-Feldern) übersetzt und als JSON-Array zurückgegeben. Diese Trennung
   (Domänen-Typ ohne Serde vs. API-DTO mit Serde) ist bewusst so gewählt, damit `problemraum/`
   frei von Infrastruktur-Abhängigkeiten bleibt (siehe `copilot-instructions.md`).
3. `list_transactions_handler` funktioniert analog über `store::get_transactions`.

## 8. Wo liegt welche Verantwortung? (Modulübersicht)

| Modul | Verantwortung | Kennt die DB? | Kennt HTTP/Serde? |
|---|---|---|---|
| `problemraum::models` | Domänen-Typen (`Invoice`, `BankTransaction`, `Currency`, `InvoiceStatus`) | Nein | Nein |
| `problemraum::matching` | Reine Matching-Logik (`match_transaction_to_invoice`) | Nein | Nein |
| `ingest::*_parser` | Rohe JSON-Bytes → Domänen-Typen (Batch, fehlertolerant) | Nein | Nur `serde_json` |
| `store` | SQL-Zugriff (Insert/Select), Currency-Code-Mapping | Ja | Nein |
| `api::invoices` / `api::transactions` | HTTP-Handler, Payload-DTOs, ruft `store` + `ingest` + `events` auf | Indirekt (via `store`) | Ja |
| `api::queries` | Read-Only-HTTP-Handler + Response-DTOs | Indirekt (via `store`) | Ja |
| `api::events` | Domänen-Ereignis → JSON-Event-Payload → `ws::broadcast_event` | Nein | Ja (Serde) |
| `ws` | WebSocket-Upgrade, Broadcast-Kanal | Nein | Ja (rohe Strings) |

## 9. Tests: welche Datei prüft welchen Ablauf?

| Test-Datei | Prüft |
|---|---|
| `tests/api_smoke/test_ingest.rs` | Abschnitt 3 (Invoice anlegen, Validierung, Liste danach) |
| `tests/api_smoke/test_transactions.rs` | Abschnitt 4 (Transaktion anlegen, Liste, Auto-Matching) |
| `tests/api_smoke/test_upload.rs` | Abschnitt 5 (Batch-Upload, teilweise ungültige Einträge) |
| `tests/integration/test_db_store.rs` | Abschnitt 7 + Store-Funktionen direkt (ohne HTTP) |
| `tests/integration/test_ws_broadcast.rs` | Abschnitt 6 (Broadcast bei Invoice-Erstellung) |
| `tests/integration/postgres_container_test.rs` | End-to-End Store-Layer gegen echten/temporären Postgres |
| `src/problemraum/models.rs` (in-file) | Domänen-Typen isoliert |
| `src/problemraum/matching.rs` (in-file) | Matching-Regeln isoliert (exakt, Teilzahlung, Überzahlung, Währungskonflikt) |
| `src/ingest/invoices_parser.rs` / `transactions_parser.rs` (in-file) | Parsing + Fehlertoleranz isoliert |
