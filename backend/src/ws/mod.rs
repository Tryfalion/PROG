use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::sync::broadcast;

// Dies ist unser zentraler Hub-Kanal für Realtime Events
// In einer echten Architektur meist im Axum AppState (Extension) gekapselt.
lazy_static::lazy_static! {
    static ref TX: broadcast::Sender<String> = {
        let (tx, _rx) = broadcast::channel(100);
        tx
    };
}

pub fn create_ws_router() -> Router {
    Router::new().route("/ws", get(ws_handler))
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut rx = TX.subscribe();

    // Pushe alle internen State-Transitions raus an Clients
    while let Ok(msg) = rx.recv().await {
        if socket.send(Message::Text(msg)).await.is_err() {
            break; // Client disconnected
        }
    }
}

/// Exportierte Funktion, die Services nutzen, um Frontend-Updates anzustoßen
pub fn broadcast_event(json_payload: String) {
    let _ = TX.send(json_payload);
}
