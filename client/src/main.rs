use tokio::net::TcpStream;
use tokio_tungstenite::connect_async;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The URL of the WebSocket server.
    let url = Url::parse("ws://127.0.0.1:8080")?;
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Got client");

    return Ok(());
}