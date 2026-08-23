# LedgerGate — Entwickler-Leitfaden (Spec-Driven Development)

Dieses Dokument ergänzt `copilot-instructions.md` um eine kompakte Übersicht für Entwickler:innen.

## Architektur-Kurzüberblick

```
Problemraum (DDD)  →  Rust-Typen  →  Store/Persistenz (N2, Postgres)  →  API/Realtime (Axum + WS)
```

- **Backend:** `backend/src/problemraum/` (Domänenlogik), `backend/src/store.rs` (DB-Zugriff),
  `backend/src/ingest/` (Datei-Parsing für Uploads), `backend/src/api/` (HTTP-Handler),
  `backend/src/ws.rs` (Realtime-Broadcast).
- **Frontend:** `frontend/src/services/` (API-Client), `frontend/src/hooks/` (Datenladen +
  WebSocket), `frontend/src/components/` (wiederverwendbare UI-Bausteine),
  `frontend/src/pages/` (Routen-Ebene).

## Spec-Nummerierung

| Bereich | Specs |
|---|---|
| Backend | 001 Domänenmodelle · 002 Matching · 003 DB-Schema · 004 Ingest-API (Invoices) · 005 Transaktions-Ingest/Upload/Queries · 006 Store-Layer · 007 WebSockets |
| Frontend | 005 Invoice-List-UI · 006 API-Service & WS-Hook · 007 Datei-Upload (Drag & Drop) · 008 Dashboard-Grundlayout · 009 Reconciliation-Board-Redesign |

Neue Features erhalten die nächste freie Nummer im jeweiligen `specs/`-Ordner (Backend und
Frontend werden getrennt nummeriert, da sie unabhängige Liefergegenstände sind).

## Granularitäts-Regel für Code & Specs

Ein Verzeichnis (Modul-Ordner) wird nur dann als eigener Ordner geführt, wenn es **mindestens
drei** inhaltlich zusammengehörige Dateien enthält (z. B. `mod.rs`/`index.ts` + mind. zwei
Implementierungsdateien). Enthält ein Bereich weniger, wird er auf eine einzelne Datei "eine
Ebene höher" zusammengefasst (Beispiel: `backend/src/store/mod.rs` → `backend/src/store.rs`,
da der Store nur eine Datei umfasste). Dieselbe Regel gilt für Spec-Verzeichnisse.

## Workflow für neue Features

1. Spec in `backend/specs/` bzw. `frontend/specs/` schreiben (Kontext, Ziel, Verhalten & Regeln,
   Testpflichten).
2. Tests aus der Spec ableiten (Unit + Integration/Smoke).
3. Freigabe einholen.
4. Implementieren, bis Tests grün sind.
5. Commit im Conventional-Commits-Format, Prompt-Log in `ai/` ergänzen.

## Testinfrastruktur

- Backend: Cargo registriert Testdateien in Unterordnern **nicht automatisch** — jede Datei unter
  `backend/tests/**/*.rs` muss als `[[test]]`-Eintrag in `backend/Cargo.toml` stehen.
- Backend-DB-Tests teilen sich in Docker-Compose-Läufen eine Postgres-Instanz; deshalb läuft
  `cargo test` dort mit `--test-threads=1` (siehe `backend/Dockerfile`).
- Frontend: `vitest` + `@testing-library/react` + `jsdom` (`frontend/vite.config.ts`,
  `frontend/tests/setup.ts`).
