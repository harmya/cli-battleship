use std::{fs::read, io};
use rand::Rng;


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

fn main() {
    let mut board_player_1 = init_board();
    let mut board_player_2 = init_board();
    
    show_board(&board_player_1);
    show_board(&board_player_2);

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

fn show_board(board : &Board) {
    for row in board.board {
        print!("|");
        for value in row {
            if value == 0 {
                print!(" |")
            } else {
                print!("*|")
            }
        }
        print!(" |");
        println!();
    }
    println!("{}", board.num_ships);
    println!("{}", board.open_space_left);
}

fn get_player_input(player_number : u8) -> (usize, usize) {
    loop {
        println!("Player {player_number}, type i,j as the box to hit where i ≤ 10 and j ≤ 10");
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


