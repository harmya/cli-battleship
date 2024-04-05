use std::io;

use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use futures_util::{stream::StreamExt, SinkExt};


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The URL of the WebSocket server.
    let url = Url::parse("ws://127.0.0.1:8080")?;
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Got server");

    let (mut write, mut read) = ws_stream.split();
    
    

    loop {

        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    println!("Received a message from the server: {}", msg);
                }
                Err(e) => {
                    println!("Error receiving message: {:?}", e);
                    break;
                }
            }
        }

        let mut input_string = String::new();
        io::stdin()
        .read_line(&mut input_string)
        .expect("perchance....no line?");
        
        write.send(Message::Text(input_string.into())).await?;

        println!("\nSending message to server.....\n\n");

        let mut received_message = String::new();

        

        if received_message.trim() == "byebye" {
            break;
        }

    }
    

    return Ok(());
}