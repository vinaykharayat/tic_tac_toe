use std::{io, usize};
const PLAYER_X:char = 'X';
const PLAYER_O:char = 'O';
type Board = [[char;3];3];
fn main() {
    let mut current_board:Board = [[' '; 3]; 3];
    let mut current_player:char = PLAYER_X;
    loop { 
        
        let result = take_user_input(&mut current_board, current_player);
        if check_winner(current_player, &current_board) {
            println!("Player {} is the winner", current_player);
            break;
        }
        print_board(&current_board);
        if result {
            current_player = if current_player == PLAYER_X {
            PLAYER_O
            } else {
                PLAYER_X
            };
        }
        
    }   
}

fn check_winner(current_player: char, current_board: &Board) -> bool {
    let mut won = false;
    for row in 0..3 {
        if current_board[row][0] == current_player &&
        current_board[row][1] == current_player &&
        current_board[row][2] == current_player {
            won = true;
        }
    }

    for col in 0..3 {
        if current_board[0][col] == current_player &&
        current_board[1][col] == current_player &&
        current_board[2][col] == current_player {
            won = true;
        }
    }

    if current_board[0][0] == current_player &&
        current_board[1][1] == current_player &&
        current_board[2][2] == current_player {
            won = true;
    }

    if current_board[0][2] == current_player &&
        current_board[1][1] == current_player &&
        current_board[2][0] == current_player {
            won = true;
    }
    return won;
}

fn take_user_input(current_board: &mut Board, current_player: char) -> bool {
    println!("Your turn {}. Enter (row, col)", current_player);
    let mut user_input = String::new();
    let _ = io::stdin().read_line(&mut user_input);
    let coordinates:Vec<usize> = user_input
                                    .split_whitespace()
                                    .filter_map(|s| s.parse::<usize>().ok())
                                    .collect();
    if coordinates.len() == 2 
    && coordinates[0] <= 2
    && coordinates[1] <= 2
    && current_board[coordinates[0]][coordinates[1]] == ' ' 
    
    {
        current_board[coordinates[0]][coordinates[1]] = current_player;
        return true;
    } else {
        println!("Incorrect input");
        return false;
    }
}

fn print_board(current_board: &Board) {
    for x in current_board{
        for y in x {
            print!("{}", y);
        }
        println!("");
    }
}