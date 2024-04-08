use std::io::{self, Write};

use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;
use futures_util::{stream::StreamExt, SinkExt};

const BOARD_DIMENSION : usize = 10;


struct ServerMessage {
    player_number : usize,
    move_id : String,
    msg : String
}

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
                    println!("\nReceived a message from the server\n {}", msg);
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

fn process_message(message : String) -> ServerMessage {
    let key_message = message.split(";").next().unwrap();
    let value_message = message.split(";").nth(1).unwrap();
    let player_number = key_message.split_whitespace().next().unwrap().parse::<usize>().unwrap();
    let move_id = key_message.split_whitespace().nth(1).unwrap();
    return ServerMessage{player_number: player_number, move_id: move_id.to_string(), msg: value_message.to_string()};
}

fn reply_to_server_message(message : String) -> String {
    let message = message.trim();
    let msg = process_message(message.to_string());

    let player_number = msg.player_number;
    let move_id = msg.move_id;
    let message = msg.msg;

    if move_id == "init" || move_id == "turn" {
        let (player_shot_row, player_shot_col) = get_player_input();
        return format!("{},{},{}", player_number, player_shot_row, player_shot_col);
    } else {
        let mut input = String::new();
        print!("Enter a message to send to the server: ");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        return input;
    }
}

fn get_player_input() -> (usize, usize) {
    loop {
        println!("Player, type i,j as the box to hit where i ≤ 10 and j ≤ 10");
        let mut player_input = String::new();
        
        io::stdout().flush().unwrap();

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