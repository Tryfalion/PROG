# LedgerGate — Administrator-Setup-Handbuch

Dieses Handbuch beschreibt, wie LedgerGate installiert, konfiguriert und betrieben wird.

## 1. Voraussetzungen

- Docker & Docker Compose (empfohlener Weg für Backend + Datenbank + Tests).
- Node.js ≥ 18 und npm (für das Frontend).
- Optional für lokale Backend-Entwicklung ohne Docker: Rust-Toolchain ≥ 1.75 (`rustup`).

## 2. Backend + Datenbank starten (Docker)

```bash
# Startet Postgres und führt die komplette Testsuite gegen eine echte DB aus mit git
./backend/run-tests-in-docker.sh
```

Für den produktiven/lokalen Betrieb (Server bleibt erreichbar, keine Tests):

```bash
docker compose up -d postgres
cd backend
DATABASE_URL="postgres://postgres:postgres@localhost:5432/postgres" cargo run
```

Der Server lauscht danach auf `http://localhost:3000`. Beim Start werden Datenbank-Migrationen
automatisch angewendet (`sqlx::migrate!`).

### Umgebungsvariablen (Backend)

| Variable | Beschreibung | Default |
|---|---|---|
| `DATABASE_URL` | Postgres-Verbindungsstring | `postgres://postgres:postgres@localhost:5432/postgres` |
| `RUST_LOG` | Log-Level für `tracing_subscriber` (z. B. `info`, `debug`) | (keiner) |

## 3. Frontend starten

```bash
cd frontend
npm install
npm run dev
```

Der Dev-Server läuft standardmäßig unter `http://localhost:5173` und erwartet das Backend
unter `http://localhost:3000`.

### Umgebungsvariablen (Frontend)

| Variable | Beschreibung | Default |
|---|---|---|
| `VITE_API_BASE_URL` | Basis-URL des Backends (REST + WebSocket-Ableitung) | `http://localhost:3000` |

Beispiel für eine abweichende Backend-Adresse:

```bash
VITE_API_BASE_URL=http://ledgergate-backend:3000 npm run build
```

## 4. Tests ausführen

Backend (gegen echten Postgres via Docker Compose):

```bash
./backend/run-tests-in-docker.sh
```

Backend (lokal, mit `testcontainers`-Fallback, benötigt trotzdem Docker):

```bash
cd backend
cargo test --release -- --nocapture
```

Frontend:

```bash
cd frontend
npm test
```

## 5. Produktions-Build

Frontend:

```bash
cd frontend
npm run build   # erzeugt frontend/dist, per beliebigem Static-Server ausliefern
```

Backend:

```bash
cd backend
cargo build --release   # Binary unter target/release/ledger_gate
```

## 6. Bekannte Betriebs-Hinweise

- CORS ist im Backend für alle Origins geöffnet (`tower-http::cors::Any`) — für den
  Uni-Prototyp bewusst offen gehalten; für einen echten Produktivbetrieb sollte dies auf die
  tatsächliche Frontend-Domain eingeschränkt werden.
- Der WebSocket-Broadcast-Kanal ist ein einzelner In-Memory-`tokio::sync::broadcast`-Kanal
  (kein Multi-Instanz-Fanout). Für einen horizontal skalierten Betrieb (mehrere Backend-Replicas)
  müsste dies durch z. B. Postgres `LISTEN/NOTIFY` oder Redis Pub/Sub ersetzt werden.
- Migrationen liegen unter `backend/migrations/` und werden beim Start automatisch angewendet;
  neue Migrationen müssen linear (aufsteigend nummeriert) über `sqlx-cli` hinzugefügt werden.
