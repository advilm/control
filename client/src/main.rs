use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::{process::Command, time::sleep};
use tokio_tungstenite::{connect_async_tls_with_config, tungstenite::Message};

#[tokio::main]
async fn main() {
    let duration = Duration::from_secs(5);
    let url = url::Url::parse("wss://control.advil.me/ws").unwrap();

    loop {
        if let Ok(connection) = connect_async_tls_with_config(&url, None, None).await {
            println!("WebSocket handshake has been successfully completed");

            let (mut write, read) = connection.0.split();

            tokio::spawn(async move {
                let duration = Duration::from_secs(45);
                loop {
                    sleep(duration).await;
                    if let Err(e) = write.send(Message::Text("ping".to_string())).await {
                        println!("Error sending message: {}", e);
                        break;
                    }
                }
            });

            read.for_each(|data| async {
                if data.is_err() {
                    return;
                }

                if let Ok(message) = data.unwrap().into_text() {
                    match &message[..] {
                        "up" => {
                            println!("xdotool mousemove_relative 0 -100");
                            Command::new("xdotool")
                                .args(["mousemove_relative", "0", "-100"])
                                .spawn()
                                .expect("Failed to execute process");
                        }
                        "down" => {
                            println!("xdotool mousemove_relative 0 100");
                            Command::new("xdotool")
                                .args(["mousemove_relative", "0", "100"])
                                .spawn()
                                .expect("Failed to execute process");
                        }
                        "left" => {
                            println!("xdotool mousemove_relative -- -100 0");
                            Command::new("xdotool")
                                .args(["mousemove_relative", "--", "-100", "0"])
                                .spawn()
                                .expect("Failed to execute process");
                        }
                        "right" => {
                            println!("xdotool mousemove_relative 100 0");
                            Command::new("xdotool")
                                .args(["mousemove_relative", "100", "0"])
                                .spawn()
                                .expect("Failed to execute process");
                        }
                        "click" => {
                            println!("xdotool click 1");
                            Command::new("xdotool")
                                .args(["click", "1"])
                                .spawn()
                                .expect("Failed to execute process");
                        }
                        _ => {}
                    }
                }
            })
            .await;
        } else {
            println!("WebSocket handshake has failed");
        }

        sleep(duration).await;
    }
}
