use futures_util::{StreamExt, SinkExt};
use std::time::Duration;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::main]
async fn main() {
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    let url = url::Url::parse("ws://control.advil.me/ws").unwrap();

    loop {
        interval.tick().await;
        if let Ok(connection) = connect_async(&url).await {
            let ws_stream = connection.0;
            println!("WebSocket handshake has been successfully completed");

            let (mut write, read) = ws_stream.split();

            tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(60));
                loop {
                    interval.tick().await;
                    if let Err(e) = write.send(Message::Text("ping".to_string())).await {
                        println!("Error sending message: {}", e);
                        break;
                    }
                }
            });

            read.for_each(|data| async {
                if !data.is_err() {
                    if let Ok(message) = data.unwrap().into_text() {
                        match &message[..] {
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
                            "click" => {
                                println!("xdotool click 1");
                                tokio::process::Command::new("xdotool")
                                    .arg("click")
                                    .arg("1")
                                    .spawn()
                                    .expect("Failed to execute process");
                            }
                            _ => {}
                        }
                    }
                }
            })
            .await;
        } else {
            println!("WebSocket handshake has failed");
        }
    }
}
