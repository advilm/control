use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension, Query,
    },
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use futures::{
    lock::Mutex,
    sink::SinkExt,
    stream::{SplitSink, StreamExt},
};
use std::collections::HashMap;
use std::sync::Arc;
type WebSockets = Arc<Mutex<HashMap<String, SplitSink<WebSocket, Message>>>>;

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

    axum::Server::bind(&"0.0.0.0:9000".parse().unwrap())
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
    println!("New connection");

    let uuid = uuid::Uuid::new_v4().to_simple().to_string();
    let (sender, mut receiver) = socket.split();
    websockets.lock().await.insert(uuid.clone(), sender);

    tokio::spawn(async move {
        while let Some(msg) = receiver.next().await {
            if msg.is_ok() {
                if let Some(socket) = websockets.lock().await.get_mut(&uuid) {
                    if let Err(e) = socket.send(Message::Text("pong".to_string())).await {
                        println!("Error sending message: {}", e);
                        break;
                    }
                }
            } else {
                println!("Error receiving message: {:?}", msg.err().unwrap());
                break;
            }
        }
        websockets.lock().await.remove(&uuid);
    });
}

async fn handle_run(query: Query<Parameters>, Extension(websockets): Extension<WebSockets>) {
    println!("Running command: {}", query.command);
    for (uuid, ws) in websockets.lock().await.iter_mut() {
        if ws.send(Message::Text(query.command.clone())).await.is_err() {
            websockets.lock().await.remove(uuid);
            println!("Connection closed");
        }
    }
}
