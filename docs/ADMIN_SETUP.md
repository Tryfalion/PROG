# LedgerGate - Administrator-Setup-Handbuch

Dieses Handbuch erklaert den kompletten Start von LedgerGate aus einer ZIP-Datei. Die Anleitung
ist fuer Windows mit PowerShell geschrieben. In Linux/macOS kann `corepack pnpm` durch `pnpm`
ersetzt werden.

## 1. Was wird installiert?

LedgerGate besteht aus drei Teilen:

- **PostgreSQL** speichert Rechnungen und Transaktionen. Die Datenbank laeuft in Docker.
- **Backend** stellt API und WebSocket bereit. Es laeuft lokal mit Rust auf Port 3000.
- **Frontend** ist die Weboberflaeche. Es laeuft lokal mit Vite auf Port 5173.

## 2. Voraussetzungen installieren

Installiere vor dem Start:

1. **Docker Desktop fuer Windows** von <https://www.docker.com/products/docker-desktop/>.
   Docker Desktop muss geoeffnet sein und den Status "Running" anzeigen.
2. **Node.js LTS** von <https://nodejs.org/>. Corepack (fuer pnpm) ist enthalten.
3. **Rustup** von <https://rustup.rs/>. Die Standardinstallation reicht aus.

Pruefe danach in PowerShell, ob die Programme gefunden werden:

```powershell
docker --version
docker compose version
node --version
corepack --version
cargo --version
```

Wenn ein Befehl nicht gefunden wird, starte PowerShell nach der Installation neu. Docker-Befehle
funktionieren erst, wenn Docker Desktop laeuft.

## 3. ZIP-Datei vorbereiten

1. Entpacke die ZIP-Datei vollstaendig, zum Beispiel nach
   `C:\Users\<DeinName>\Documents\LedgerGate-PROG`.
2. Oeffne diesen entpackten Ordner in VS Code (**Datei > Ordner oeffnen**).
3. Oeffne in VS Code ein Terminal (**Terminal > Neues Terminal**).

Alle folgenden Befehle werden im Projekt-Hauptordner ausgefuehrt, also dort, wo
`docker-compose.yml`, `backend` und `frontend` liegen. Falls das Terminal in einem anderen Ordner
startet, wechsle zuerst dorthin:

```powershell
cd "C:\Users\<DeinName>\Documents\LedgerGate-PROG"
```

## 4. Anwendung starten (Schnellstart)

Fuer den normalen Betrieb benoetigst du drei Terminals. Lass Terminal 1 bis 3 geoeffnet, solange du
LedgerGate verwendest.

### Terminal 1: Datenbank

```powershell
docker compose up -d postgres
```

Beim ersten Start laedt Docker zunaechst das PostgreSQL-Image herunter. Das kann einige Minuten
dauern.

### Terminal 2: Backend

Oeffne ein zweites Terminal im Projekt-Hauptordner:

```powershell
cd backend
cargo run
```

Beim ersten Start laedt Rust die Abhaengigkeiten herunter und kompiliert das Backend. Die Meldung
`listening on 0.0.0.0:3000` bedeutet, dass das Backend bereit ist. Die Datenbanktabellen werden
beim Start automatisch angelegt.

### Terminal 3: Frontend

Oeffne ein drittes Terminal im Projekt-Hauptordner:

```powershell
cd frontend
corepack pnpm install
corepack pnpm run dev
```

`corepack pnpm install` ist nur beim ersten Mal oder nach Aenderungen an `package.json` oder
`pnpm-lock.yaml` notwendig. Oeffne anschliessend <http://localhost:5173> im Browser. Das Frontend
verbindet sich standardmaessig mit dem Backend unter <http://localhost:3000>.

## 5. Anwendung beenden

- Druecke in Terminal 2 und Terminal 3 jeweils `Ctrl+C`.
- Stoppe danach die Datenbank im Projekt-Hauptordner:

```powershell
docker compose stop postgres
```

Beim naechsten Start genuegt wieder `docker compose up -d postgres`; die Daten bleiben erhalten.
Zum vollstaendigen Zuruecksetzen der Datenbank einschliesslich aller Daten verwende bewusst:

```powershell
docker compose down -v
```

## 6. Tests ausfuehren

### Backend- und Integrationstests mit Docker

Die Variante funktioniert direkt aus PowerShell und benoetigt Docker Desktop:

```powershell
docker compose up --build --abort-on-container-exit --exit-code-from tester
docker compose down -v
```

Der erste Befehl baut den Test-Container, startet PostgreSQL und fuehrt die Backend-Tests aus. Der
zweite Befehl raeumt die Test-Container und die Testdatenbank auf. Der Testlauf ist erfolgreich,
wenn der erste Befehl mit Exit-Code 0 endet.

Alternativ kann `backend/run-tests-in-docker.sh` in Git Bash oder WSL ausgefuehrt werden:

```bash
./backend/run-tests-in-docker.sh
```

### Frontend-Tests

```powershell
cd frontend
corepack pnpm test
corepack pnpm run build
corepack pnpm run lint
```

## 7. Konfiguration

### Backend

| Variable | Beschreibung | Standardwert |
|---|---|---|
| `DATABASE_URL` | PostgreSQL-Verbindungsstring | `postgres://postgres:postgres@localhost:5432/postgres` |
| `RUST_LOG` | Log-Level, zum Beispiel `info` oder `debug` | nicht gesetzt |

Fuer den lokalen Start ist keine Konfiguration noetig. Das Backend verwendet den Standardwert
automatisch. Eine alternative Datenbank kann in PowerShell gesetzt werden:

```powershell
$env:DATABASE_URL = "postgres://postgres:postgres@localhost:5432/postgres"
cargo run
```

### Frontend

| Variable | Beschreibung | Standardwert |
|---|---|---|
| `VITE_API_BASE_URL` | Basis-URL fuer REST und WebSocket | `http://localhost:3000` |

```powershell
$env:VITE_API_BASE_URL = "http://localhost:3000"
corepack pnpm run dev
```

## 8. Produktions-Build

Der Build ist fuer die lokale Entwicklung nicht erforderlich. Fuer einen Release-Build:

```powershell
cd frontend
corepack pnpm run build
```

Das fertige Frontend liegt danach in `frontend/dist`. Das Backend wird so gebaut:

```powershell
cd backend
cargo build --release
```

## 9. Fehlerbehebung

### Docker kann nicht verbunden werden

Docker Desktop ist nicht gestartet oder noch nicht bereit. Oeffne Docker Desktop, warte auf den
Status "Running" und wiederhole den Befehl.

### Ein Port ist bereits belegt

Beende das Programm, das Port 5432, 3000 oder 5173 verwendet, oder stoppe alte LedgerGate-
Prozesse. Den Status der Compose-Services kannst du so pruefen:

```powershell
docker compose ps
```

### Das Backend meldet einen Datenbankfehler

Stelle sicher, dass `docker compose up -d postgres` ohne Fehler ausgefuehrt wurde. Standardmaessig
werden Benutzer `postgres`, Passwort `postgres` und Datenbank `postgres` verwendet.

### Das Frontend zeigt keine Daten

Pruefe, dass das Backend noch laeuft und <http://localhost:3000> erreichbar ist. Die API-Routen
beginnen mit `/api/v1/`; die WebSocket-Verbindung verwendet `/ws`.

## 10. Hinweise fuer den Betrieb

- CORS ist fuer den Uni-Prototyp fuer alle Origins geoeffnet. Fuer einen echten Produktivbetrieb
  sollte dies auf die tatsaechliche Frontend-Domain eingeschraenkt werden.
- Der WebSocket-Broadcast laeuft nur im Speicher einer Backend-Instanz. Fuer mehrere Instanzen
  waere beispielsweise Redis Pub/Sub oder PostgreSQL `LISTEN/NOTIFY` erforderlich.
- Migrationen liegen unter `backend/migrations/` und werden beim Backend-Start automatisch
  angewendet. Neue Migrationen muessen aufsteigend nummeriert werden.
