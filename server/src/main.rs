use std::io;
use futures_util::SinkExt;
use rand::Rng;
use tokio::{self};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use futures_util::stream::StreamExt;

const BOARD_DIMENSION : usize = 10;

const NUM_BIG_SHIPS : usize = 1;
const BIG_SHIP_SIZE : usize = 4;

const NUM_MEDIUM_SHIPS : usize = 2;
const MEDIUM_SHIP_SIZE : usize = 2;

const NUM_SMALL_SHIPS : usize = 4;
const SMALL_SHIP_SIZE : usize = 1;

enum ORIENTATION {
    Vertical,
    Horizontal
}

enum BATTLESHIP {
    Big,
    Medium,
    Small
}
impl BATTLESHIP {
    fn get_ship_size(&self) -> usize {
        match self {
            BATTLESHIP::Big => {return BIG_SHIP_SIZE},
            BATTLESHIP::Medium => {return MEDIUM_SHIP_SIZE},
            BATTLESHIP::Small => {return SMALL_SHIP_SIZE}
        }
    }
}

struct Board {
    board : [[u8; BOARD_DIMENSION]; BOARD_DIMENSION],
    num_ships: usize,
    open_space_left: usize
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on: ws://127.0.0.1:8080");

    let mut player_count : usize = 0;

    while let Ok((stream, _)) = listener.accept().await {
        player_count += 1;
        tokio::spawn(handle_client(stream, player_count)); // Change this line
    }

    return Ok(());
}

async fn handle_client(stream: tokio::net::TcpStream, player_number: usize) {

    if let Ok(ws_handshake) = accept_async(stream).await {
        println!("New websocket connection!");
        let player_board = init_board();
        let board_message = send_board(&player_board, player_number);
        let (mut write, mut read) = ws_handshake.split();
        if write.send(board_message.into()).await.is_err() {
            println!("Message not sent due to internal error"); 
        }
        // listen for messages from the client
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    println!("Received a message from the Player {}: {}", player_number, msg);
                    let reply: String = reply_to_message_from_client(msg.to_string(), player_number).await;
                    if write.send(tokio_tungstenite::tungstenite::Message::Text(reply)).await.is_err() {
                        println!("Message not sent due to internal error");
                    }
                }
                Err(e) => {
                    println!("Error receiving message: {:?}", e);
                    break;
                }
            }
        }

    } else {
        println!("perchance....websocket got cooked");
    }
}

async fn reply_to_message_from_client(message : String, player_number: usize) -> String {
    let mut input = String::new();
    println!("Enter a message to send to the client: ");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input;
}

fn init_board() -> Board {
    let board : [[u8; BOARD_DIMENSION]; BOARD_DIMENSION] = [[0; BOARD_DIMENSION]; BOARD_DIMENSION];
    let num_ships = 0;
    let open_space_left = BOARD_DIMENSION * BOARD_DIMENSION;
    let mut board_struct = Board{ board: board, num_ships: num_ships, open_space_left: open_space_left};

    initialize_battleships(&mut board_struct, BATTLESHIP::Big, NUM_BIG_SHIPS);
    initialize_battleships(&mut board_struct, BATTLESHIP::Medium, NUM_MEDIUM_SHIPS);
    initialize_battleships(&mut board_struct, BATTLESHIP::Small, NUM_SMALL_SHIPS);

    return board_struct;
}

fn send_board(board : &Board, player_number: usize) -> String {

    let mut board_string = String::new();
    board_string.push_str(&format!("\nInitial battleship board for Player {}\n", player_number)); 
    for row in board.board {
        board_string.push_str("\n|");
        for value in row {
            if value == 0 {
                board_string.push_str("   |")
            } else {
                board_string.push_str(" * |")
            }
        }
        board_string.push_str("  |");
        board_string.push_str("\n\n")
    }
    board_string.push_str("Number of ships: ");
    board_string.push_str(&board.num_ships.to_string());
    board_string.push_str("\n");
    board_string.push_str("Open spaces left: ");
    board_string.push_str(&board.open_space_left.to_string());
    board_string.push_str("\n-----------------------------------\n\n");
    return board_string;
}

fn update_player_board(board : &mut Board, hit_location : (usize, usize)) {
    board.board[hit_location.0][hit_location.1] = 0;
    board.open_space_left += 1;
}

fn is_near_another_ship(board: &Board, row : usize, col : usize) -> bool {
    if row > 0 && board.board[row - 1][col] == 1 {
        return true;
    } else if row < BOARD_DIMENSION - 1 && board.board[row + 1][col] == 1 {
        return true;
    } else if col > 0 && board.board[row][col - 1] == 1 {
        return true;
    } else if col < BOARD_DIMENSION - 1 && board.board[row][col + 1] == 1 {
        return true;
    }
    return false;
}

fn valid_generated_ship(board: &Board, ship_size : usize, start_index: usize, row_or_col: usize, ship_orientation: &ORIENTATION) -> bool {
    match ship_orientation {
        ORIENTATION::Vertical => {
            for i in start_index..start_index + ship_size {
                if board.board[i][row_or_col] == 1 || is_near_another_ship(board, i, row_or_col) {
                    return false;
                }
            }
        },
        ORIENTATION::Horizontal => {
            for i in start_index..start_index + ship_size {
                if board.board[row_or_col][i] == 1 || is_near_another_ship(board, row_or_col, i) {
                    return false
                }
            }
        },
    };
    return true;
}

fn initialize_battleships(board: &mut Board, ship_type: BATTLESHIP, ship_count: usize) {
    let ship_size = ship_type.get_ship_size();
    board.num_ships += ship_count;
    board.open_space_left -= ship_size * ship_count;
    
    for _i in 0..ship_count {
        let ship_orientation: ORIENTATION =
            if rand::thread_rng().gen_range(1..=2) == 1 {
                ORIENTATION::Vertical
            } else {
                ORIENTATION::Horizontal
            };
        
        let mut random_row_or_col = rand::thread_rng().gen_range(0..BOARD_DIMENSION);
        let mut random_start_index = rand::thread_rng().gen_range(0..BOARD_DIMENSION - ship_size);

        while !valid_generated_ship(board, ship_size, random_start_index, random_row_or_col, &ship_orientation) {
            random_row_or_col = rand::thread_rng().gen_range(0..BOARD_DIMENSION);
            random_start_index = rand::thread_rng().gen_range(0..BOARD_DIMENSION - ship_size);
        }
        
        for i in random_start_index..random_start_index + ship_size {
            match ship_orientation {
                ORIENTATION::Vertical => {board.board[i as usize][random_row_or_col as usize] = 1},
                ORIENTATION::Horizontal => board.board[random_row_or_col as usize][i as usize] = 1,
            };
        }
    }
}