use std::env;

use futures_util::{future, pin_mut, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[tokio::main]
async fn main() {
    let url = url::Url::parse("ws://127.0.0.1:3000/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (_, read) = ws_stream.split();

    read.for_each(|message| async {
        let data = message.unwrap().into_text().unwrap();
        match &data[..] {
            "up" => {
                println!("xdotool key Up");
                tokio::process::Command::new("xdotool")
                    .arg("mousemove_relative")
                    .arg("0")
                    .arg("-100")
                    .spawn()
                    .expect("Failed to execute process");
            }
            "down" => {
                println!("xdotool key Down");
                tokio::process::Command::new("xdotool")
                    .arg("mousemove_relative")
                    .arg("0")
                    .arg("100")
                    .spawn()
                    .expect("Failed to execute process");
            }
            "left" => {
                println!("xdotool key Left");
                tokio::process::Command::new("xdotool")
                    .arg("mousemove_relative")
                    .arg("--")
                    .arg("-100")
                    .arg("0")
                    .spawn()
                    .expect("Failed to execute process");
            }
            "right" => {
                println!("xdotool key Right");
                tokio::process::Command::new("xdotool")
                    .arg("mousemove_relative")
                    .arg("100")
                    .arg("0")
                    .spawn()
                    .expect("Failed to execute process");
            }
            _ => println!("{}", data),
        }
    }).await;
}
