# LedgerGate

Fortgeschrittene Programmierung — Reconciliation- und Rechnungssystem für Selbstständige und
kleine Teams. Rust/Axum-Backend (N2-Architektur, PostgreSQL, WebSockets) + React-Frontend.

## Schnellstart

```bash
# 1. Backend + Datenbank starten (Docker; wendet Migrationen automatisch an)
docker compose up -d postgres
cd backend && DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres" cargo run

# 2. Frontend (separates Terminal) — `corepack pnpm install` ist zwingend,
#    sonst meldet der Editor "Cannot find module 'react'" etc., weil node_modules/ dann fehlt.
cd frontend
corepack pnpm install
corepack pnpm run dev
```

Danach ist das Frontend unter `http://localhost:5173` erreichbar, das Backend unter
`http://localhost:3000`. Das Frontend spricht das Backend über `VITE_API_BASE_URL`
(Default `http://localhost:3000`) an — REST-Aufrufe und die WebSocket-Verbindung (`/ws`) nutzen
dieselbe Basis-Adresse (siehe `frontend/src/services/api.ts`). CORS ist im Backend für alle
Origins geöffnet, damit der Vite-Dev-Server (Port 5173) ohne weitere Konfiguration auf Port 3000
zugreifen kann.

Komplette Testsuite (Backend gegen echten Postgres-Container):

```bash
./backend/run-tests-in-docker.sh
```

## Dokumentation

- [copilot-instructions.md](copilot-instructions.md) — Architektur, Tech-Stack, Spec-Driven
  Development Workflow, Coding-Standards.
- [docs/USER_MANUAL.md](docs/USER_MANUAL.md) — Nutzerhandbuch (Drag & Drop Import, Reconciliation
  Board, KPIs).
- [docs/ADMIN_SETUP.md](docs/ADMIN_SETUP.md) — Installation, Konfiguration, Betrieb, Tests.
- [docs/DEVELOPER_GUIDE.md](docs/DEVELOPER_GUIDE.md) — Spec-Nummerierung, Modul-Granularität,
  Testinfrastruktur.
- [docs/BACKEND_ARCHITECTURE.md](docs/BACKEND_ARCHITECTURE.md) — Erklärt jeden Backend-Ablauf
  Schritt für Schritt (welche Datei ruft welche Funktion auf, bis hin zur DB/zum WebSocket).
- [docs/FRONTEND_ARCHITECTURE.md](docs/FRONTEND_ARCHITECTURE.md) — Erklärt jeden
  Frontend-Ablauf Schritt für Schritt (Rendering, API-Aufrufe, WebSocket, Datei-Upload).
- `backend/specs/` und `frontend/specs/` — einzelne Feature-Spezifikationen.

## Projektstruktur

```
backend/    Rust/Axum Backend (Problemraum, Store, API, WebSockets, Ingest)
frontend/   React/Vite Frontend (Dashboard, API-Service, Hooks, Komponenten)
docs/       Nutzer-, Administrator- und Entwickler-Dokumentation
ai/         Pflicht-Prompt-Log für alle LLM-Interaktionen
test_data/  Beispiel-JSON-Dateien für den Datei-Upload
```
