use futures_util::StreamExt;
use tokio_tungstenite::connect_async;

#[tokio::main]
async fn main() {
    let url = url::Url::parse("ws://control.advil.me/ws").unwrap();
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (_, read) = ws_stream.split();

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
                    _ => {}
                }
            }
        }
    })
    .await;
}
