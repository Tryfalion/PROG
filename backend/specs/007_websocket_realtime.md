# Spezifikation: WebSockets (Realtime)

## 1. Übersicht
Sobald ein Event auftritt (z. B. neue Banktransaktion via Ingest importiert), muss das Frontend sofort informiert werden. Dies entspricht dem Architektur-Prinzip (Live Streams).

## 2. Architekturvorgaben
- Backend: Nutzt `axum::extract::ws::WebSocketUpgrade` und `tokio::sync::broadcast` als zentralen Event-Hub.
- Payload: JSON, spezifiziert Events wie `InvoiceAdded`, `TransactionAdded`, `AllocationCreated`.
- Frontend: Hängt sich an `ws://localhost:3000/ws` und verarbeitet einkommende JSON-Events im React Context.

## 3. Testing Obligations
(Test wird initial als Dummy via Message send simuliert, vollständiger WS Lifecycle Test erfordert im Uni-Projekt meist E2E Tests im Browser). Backend Unit Test checkt das Channel-Setup.