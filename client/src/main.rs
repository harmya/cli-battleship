use std::io;

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use futures_util::{stream::StreamExt, SinkExt};

const BOARD_DIMENSION : usize = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("ws://127.0.0.1:8080")?;
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Got server");

    let (mut write, mut read) = ws_stream.split();

    loop {
        let mut message_from_server = String::new();

        if let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    message_from_server = msg.to_string();
                    println!("Received a message from the server: {}", msg);
                }
                Err(e) => {
                    println!("Error receiving message: {:?}", e);
                    break;
                }
            }
        }
        let message_to_server = reply_to_server_message(message_from_server);
        write.send(Message::Text(message_to_server)).await.expect("Failed to send message");
    }
    
    println!("closed?");
    
    return Ok(());
}

fn reply_to_server_message(message : String) -> String {
    //get the first word of the message
    let message = message.trim();
    let message = message.split_whitespace().next().unwrap();
    println!("Message: {}", message);
    if message == "Initial" {
        let (player_shot_row, player_shot_col) = get_player_input();
        return format!("{},{}", player_shot_row, player_shot_col);
    } else {
        return String::from("Goodbye");
    }
}

fn get_player_input() -> (usize, usize) {
    loop {
        println!("Player, type i,j as the box to hit where i ≤ 10 and j ≤ 10");
        let mut player_input = String::new();
        io::stdin()
        .read_line(&mut player_input)
        .expect("perchance....no line?");
    
        let player_input : Vec<&str> = player_input.split(',').collect();

        if player_input.len() != 2 {
            println!("ERROR: perchance....input a number?");
            continue;
        }
        
        let player_shot_row  = match player_input[0].trim()
            .parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("ERROR: Perchance....check your number? \n");
                    continue;
                }
            };
        
        let player_shot_col = match player_input[1].trim()
            .parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("ERROR: Perchance....check your number?");
                    continue;
                }
            };

        if player_shot_row >= BOARD_DIMENSION || player_shot_col >= BOARD_DIMENSION{
            println!("Input a number less than {}", BOARD_DIMENSION);
            continue;
        }

        return (player_shot_row, player_shot_col);
    }
}