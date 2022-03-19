use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, Query,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use futures::lock::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
type WebSockets = Arc<Mutex<HashMap<String, WebSocket>>>;

#[derive(serde_derive::Deserialize)]
struct Parameters {
    command: String,
}

#[tokio::main]
async fn main() {
    // create vector of shared websockets
    let ws_connections: WebSockets = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/ws", get(upgrade_ws))
        .route("/run", post(handle_run))
        .layer(Extension(ws_connections));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn upgrade_ws(
    ws: WebSocketUpgrade,
    Extension(websockets): Extension<WebSockets>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, websockets))
}

async fn handle_socket(socket: WebSocket, websockets: WebSockets) {
    let uuid = uuid::Uuid::new_v4().to_simple().to_string();
    websockets.lock().await.insert(uuid.clone(), socket);
    println!("New connection");

    // while let Some(msg) = websockets.lock().await.get_mut(&uuid).unwrap().recv().await {
    //     if msg.is_err() {
    //         println!("Connection closed");
    //         websockets.lock().await.remove(&uuid);
    //         return;
    //     }
    // }
}

async fn handle_run(query: Query<Parameters>, Extension(websockets): Extension<WebSockets>) {
    for (uuid, ws) in websockets.lock().await.iter_mut() {
        if ws.send(Message::Text(query.command.clone())).await.is_err() {
            websockets.lock().await.remove(uuid);
            println!("Connection closed");
        }
    }
}
