# Spec 006 — API-Service-Schicht & WebSocket-Hook (Frontend)

## Kontext
Frontend hat bisher keine Verbindung zum Backend (nur Mock-Daten). `frontend/src/services/`
und `frontend/src/hooks/` sind leer. Backend bietet seit Spec 005: `GET/POST /api/v1/invoices`,
`GET/POST /api/v1/transactions`, Upload-Endpunkte, sowie WS-Events unter `/ws` (Spec 007 Backend).

## Ziel / Scope
1. `services/api.ts` — Zentrale `fetch`-Wrapper-Funktionen für alle Backend-Endpunkte
   (`getInvoices`, `getTransactions`, `createInvoice`, `createTransaction`, `uploadInvoicesFile`,
   `uploadTransactionsFile`). Basis-URL konfigurierbar über `VITE_API_BASE_URL` (Fallback
   `http://localhost:3000`).
2. `services/types.ts` — TypeScript-Typen, die die Backend-JSON-DTOs spiegeln (`InvoiceDto`,
   `TransactionDto`, `UploadResult`).
3. `hooks/useWebSocket.ts` — Verbindet sich mit `ws://<host>/ws`, parst eingehende JSON-Events
   (`invoice_created`, `transaction_created`, `allocation_created`) und ruft einen übergebenen
   Callback auf. Reconnect nach Verbindungsabbruch mit exponentiellem Backoff (max. 5 Versuche).
4. `hooks/useInvoicesAndTransactions.ts` — Lädt Rechnungen & Transaktionen initial per REST,
   abonniert danach `useWebSocket` und lädt bei jedem Event die betroffene Liste neu (einfacher
   "refetch on event" Ansatz statt komplexem Client-Side-Merging, bewusst einfach gehalten für
   die Nachvollziehbarkeit im Uni-Projekt).

## Verhalten & Regeln
- Netzwerkfehler beim initialen Laden zeigen eine Fehlermeldung in der UI, brechen die App aber
  nicht ab (leere Listen als Fallback).
- Der WebSocket-Hook darf die Komponente nicht crashen, wenn `/ws` nicht erreichbar ist
  (z. B. Backend noch nicht gestartet) — Fehler werden geloggt, kein Rethrow.

## Testpflichten
- Da das Frontend keine Testinfrastruktur hatte, wird `vitest` + `@testing-library/react`
  eingeführt (Spec 009 deckt die Einrichtung ab). Tests für diese Spec:
  - `services/api.test.ts`: `getInvoices` parst eine gemockte `fetch`-Antwort korrekt.
  - `hooks/useWebSocket.test.ts`: Hook ruft `onMessage` bei eingehender Nachricht auf (Mock-WebSocket).
