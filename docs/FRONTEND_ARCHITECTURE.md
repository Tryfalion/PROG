# LedgerGate — Frontend-Dokumentationshandbuch

Dieses Dokument erklärt **Datei für Datei und Funktion für Funktion**, wie das Frontend
aufgebaut ist und wie Daten vom Backend bis auf den Bildschirm gelangen. Ergänzt
`copilot-instructions.md` und `docs/DEVELOPER_GUIDE.md`.

## 1. Einstiegspunkt (`index.html` → `main.tsx` → `App.tsx`)

1. `index.html` lädt `src/main.tsx` als ES-Modul.
2. `main.tsx` ruft `ReactDOM.createRoot(...).render(<App />)` auf — React übernimmt ab hier das
   `<div id="root">`.
3. `App.tsx` rendert die Navigationsleiste (`.dashboard-nav`) und `react-router-dom`-Routen:
   - `/` → `pages/Statistics.tsx`
   - `/live` → `pages/LiveProcess.tsx`
   - `/settings` → `pages/Settings.tsx`

Keine dieser Seiten bekommt Daten als Props von `App.tsx` — jede Seite lädt ihre Daten selbst
über den Hook aus Abschnitt 3.

## 2. Verbindung zum Backend (`services/api.ts`, `services/types.ts`)

`services/types.ts` definiert TypeScript-Typen, die **exakt** die JSON-Antworten des Backends
spiegeln (`InvoiceDto`, `TransactionDto`, `UploadResult`, `RealtimeEvent`) — siehe
`docs/BACKEND_ARCHITECTURE.md` Abschnitt 7 für die Rust-Gegenstücke (`InvoiceView`,
`TransactionView`).

`services/api.ts` ist die **einzige** Datei, die `fetch()` aufruft:

- `API_BASE_URL` wird aus `import.meta.env.VITE_API_BASE_URL` gelesen, Fallback
  `http://localhost:3000` (siehe `docs/ADMIN_SETUP.md` für Konfiguration).
- `getWebSocketUrl()` leitet aus derselben Basis-URL die WebSocket-Adresse ab
  (`http` → `ws`, `+ "/ws"`).
- `requestJson<T>(path, init)` ist der interne Low-Level-Wrapper: ruft `fetch`, wirft bei
  `!response.ok`, parst sonst `response.json()`.
- `getInvoices()` / `getTransactions()` → `GET /api/v1/invoices` bzw. `/api/v1/transactions`.
- `createInvoice()` / `createTransaction()` → `POST` auf dieselben Pfade (aktuell nicht an ein
  UI-Formular angebunden, aber einsatzbereit für zukünftige manuelle Eingabemasken).
- `uploadFile(kind, file)` → baut ein `FormData` mit Feld `"file"`, POSTet an
  `/api/v1/invoices/upload` bzw. `/api/v1/transactions/upload` (abhängig von `kind`).

`services/index.ts` re-exportiert alles aus `api.ts`/`types.ts` als ein Barrel-Modul (Import-Komfort).

## 3. Daten laden + live aktuell halten (`hooks/useInvoicesAndTransactions.ts`)

Dieser Hook wird von **jeder** Seite genutzt, die Rechnungs-/Zahlungsdaten anzeigt
(`pages/Statistics.tsx`, `pages/LiveProcess.tsx`). Ablauf:

1. Beim ersten Rendern (`useEffect`) ruft er `reloadInvoices()` und `reloadTransactions()` auf —
   das sind `useCallback`-gewrappte Funktionen, die `services/api.ts::getInvoices()` bzw.
   `getTransactions()` aufrufen und das Ergebnis in `useState` ablegen (`invoices`, `transactions`).
2. Schlägt ein Request fehl, wird die Fehlermeldung in `error` gespeichert (die Seite zeigt sie an,
   die App crasht nicht — leere Listen bleiben als Fallback).
3. Er registriert außerdem `useWebSocket(callback)` (siehe Abschnitt 4). Der Callback entscheidet:
   - `event.type === 'transaction_created'` → nur `reloadTransactions()`.
   - alles andere (`invoice_created`, `allocation_created`) → `reloadInvoices()`
     (eine Allocation ändert den *Status* einer Rechnung, nicht die Transaktionsliste).
4. Rückgabewert `{ invoices, transactions, error, reloadInvoices, reloadTransactions }` wird von
   den Seiten direkt zum Rendern verwendet ("refetch on event" — bewusst simpel gehalten statt
   client-seitigem Merging einzelner Events, siehe Spec 006).

## 4. WebSocket-Verbindung (`hooks/useWebSocket.ts`)

1. Baut bei jedem Mount eine neue `WebSocket`-Verbindung zu `getWebSocketUrl()` auf
   (`ws://<host>/ws`, siehe `docs/BACKEND_ARCHITECTURE.md` Abschnitt 6 für die Server-Seite).
2. `socket.onmessage` parst `event.data` als JSON (`RealtimeEvent`) und ruft den zuletzt
   übergebenen Callback auf (`onEventRef.current`, ein `useRef`, damit die Verbindung nicht bei
   jedem Render neu aufgebaut wird, nur weil sich der Callback-Ausdruck ändert).
3. `socket.onclose` löst — solange der Effekt nicht durch Unmount aufgeräumt wurde — einen
   Reconnect mit exponentiellem Backoff aus (`1s, 2s, 4s, 8s, 10s (gekappt)`), maximal 5 Versuche.
4. Beim Unmount (`useEffect`-Cleanup) wird `closedByCleanup = true` gesetzt, ein evtl. laufender
   Reconnect-Timer gecancelt und der Socket geschlossen — verhindert Reconnect-Versuche auf
   bereits verlassenen Seiten.
5. Schlägt der `new WebSocket(...)`-Aufruf selbst fehl (z. B. ungültige URL), wird das geloggt,
   nicht geworfen — die UI bleibt nutzbar, auch ganz ohne Realtime-Updates.

## 5. Rechnungen & Zahlungen hochladen (Drag & Drop)

Beteiligte Dateien: `pages/LiveProcess.tsx` → `components/FileDropModal.tsx` →
`services/api.ts::uploadFile` → Backend (`docs/BACKEND_ARCHITECTURE.md` Abschnitt 5).

1. In `LiveProcess.tsx` setzt ein Klick auf **"+ Add Invoice"** den lokalen State
   `modalKind = 'invoice'`, **"+ Add Payment"** setzt `modalKind = 'payment'`. Ist `modalKind`
   nicht `null`, wird `<FileDropModal kind={modalKind} .../>` gerendert.
2. `FileDropModal` reagiert auf zwei Eingabewege:
   - **Drag & Drop**: `onDrop` liest `event.dataTransfer.files`; bei genau einer Datei wird
     `handleFile(file)` aufgerufen, bei mehreren erscheint ein Hinweistext (kein Upload).
   - **Klick**: das (unsichtbare) `<input type="file">` öffnet den nativen Datei-Dialog;
     `onFileInputChange` ruft ebenfalls `handleFile(file)` auf.
3. `handleFile(file)`:
   - Prüft die Dateiendung (`.json`), sonst Fehlermeldung, kein Netzwerk-Aufruf.
   - Setzt `isUploading = true` (deaktiviert die Drop-Zone visuell/funktional).
   - Ruft `services/api.ts::uploadFile(kind, file)` auf (baut `FormData`, POST an den
     passenden `.../upload`-Endpunkt).
   - Bei Erfolg: `result` (State) wird gesetzt → die Zusammenfassung (`inserted`/`skipped`/
     `errors`) wird angezeigt, `onUploaded(result)` wird an die aufrufende Seite gemeldet.
   - Bei Fehler: Fehlermeldung wird angezeigt, `isUploading` zurückgesetzt.
4. **Wichtig:** Die Listen in `LiveProcess.tsx` werden nach einem Upload **nicht** manuell neu
   geladen. Stattdessen sendet das Backend pro gespeichertem Datensatz ein WebSocket-Event
   (Abschnitt 4), das über `useInvoicesAndTransactions` automatisch einen Refetch auslöst —
   Upload und Live-Aktualisierung sind damit vollständig entkoppelt.

## 6. Reconciliation-Dashboard (`pages/LiveProcess.tsx`)

1. Holt `{ invoices, transactions, error }` aus `useInvoicesAndTransactions()`.
2. Berechnet die vier KPI-Werte direkt aus den geladenen Arrays (keine eigene Backend-Route
   dafür — bewusst clientseitig, da die Datenmengen im Uni-Projekt-Kontext klein bleiben):
   - `totalInvoiced` = Summe `invoice.amount`.
   - `totalDeposits` = Summe `transaction.amount`.
   - `matchedRate` = Anteil der Rechnungen mit Status `Paid`/`Overpaid`.
   - `openDiscrepancies` = Anzahl Rechnungen mit Status `!== 'Paid'`.
3. Rendert drei Spalten:
   - **Invoices Ledger**: gefilterte Liste (`invoiceSearch`-State, `useMemo`), + "Add Invoice".
   - **Payments Ledger**: ungefilterte Transaktionsliste, + "Add Payment".
   - **Reconciliation Board**: pro Rechnung ein Status-Badge, übersetzt via
     `reconciliationLabel(status)` (`Paid` → "Perfect Match", `Overpaid` → "Overpayment",
     `Open` → "Open / Unpaid").
4. `KpiCard` (`components/KpiCard.tsx`) ist eine reine Präsentations-Komponente ohne eigene
   Logik — bekommt Label/Wert/Sublabel/Akzent als Props.

## 7. Wo liegt welche Verantwortung? (Modulübersicht)

| Modul | Verantwortung | Ruft Backend auf? | Hat eigenen State? |
|---|---|---|---|
| `services/types.ts` | TS-Spiegel der Backend-DTOs | Nein | Nein |
| `services/api.ts` | Alle `fetch`-Aufrufe, WS-URL-Ableitung | Ja | Nein |
| `hooks/useWebSocket.ts` | WS-Verbindung, Reconnect | Ja (WS) | Intern (Ref) |
| `hooks/useInvoicesAndTransactions.ts` | Laden + Live-Aktualisierung der Listen | Ja (REST + WS) | Ja (`invoices`, `transactions`, `error`) |
| `components/FileDropModal.tsx` | Drag&Drop/Datei-Upload-UI | Ja (Upload) | Ja (Drag/Upload/Fehler-Status) |
| `components/KpiCard.tsx` | Kachel-Darstellung | Nein | Nein |
| `components/InvoiceList.tsx` | Wiederverwendbare Tabellen-Darstellung einer Invoice-Liste | Nein | Nein |
| `pages/*.tsx` | Seiten-Layout, kombiniert Hooks + Komponenten | Indirekt (via Hooks) | Seiten-lokaler UI-State (Suche, Modal) |

## 8. Tests: welche Datei prüft welchen Ablauf?

| Test-Datei | Prüft |
|---|---|
| `tests/services/api.test.ts` | Abschnitt 2 (`getInvoices` parst Antwort, wirft bei Fehlerstatus) |
| `tests/hooks/useWebSocket.test.tsx` | Abschnitt 4 (Event löst Callback aus, per Mock-`WebSocket`) |
| `tests/components/FileDropModal.test.tsx` | Abschnitt 5 (ungültige Dateiendung, erfolgreicher Upload) |
| `tests/components/KpiCard.test.tsx` | Abschnitt 6 (reines Rendering von Label/Wert/Sublabel) |
