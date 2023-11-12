use crate::types::*;
use colored::Colorize;
use std::io::stdin;

pub fn check_win(board: &Board, turns: &u8) -> Option<GameEnd> {
    // don't know why this works and not lesser than
    // if board.iter().filter(|x| x.contains(&Cell::EMPTY)).count() > 5 {
    //     return None;
    // }

    for player in [Cell::P1, Cell::P2] {
        let arr = [player; 3];

        // horizontal || vertical
        for i in 0..3 as usize {
            if (arr == board[i]) || (arr == [board[0][i], board[1][i], board[2][i]]) {
                return Some(GameEnd::from(player));
            }
        }

        // diagonals
        if (arr == [board[0][0], board[1][1], board[2][2]]) // NW -> SE
        || (arr == [board[2][2], board[1][1], board[0][0]]) // SE -> NW
        || (arr == [board[0][2], board[1][1], board[2][0]]) // SW -> NE
        || (arr == [board[2][0], board[1][1], board[0][2]])
        // NE -> SW
        // ^ Rust formatter >:[
        {
            return Some(GameEnd::from(player));
        }
    }

    if *turns == 9 {
        return Some(GameEnd::TIE);
    }

    None
}

pub fn input_to_direction<'a>() -> Result<[u8; 2], &'a str> {
    let reader = stdin();
    let mut input: String = String::new();

    reader.read_line(&mut input).unwrap(); // NOTE: shit better not panic bro

    let mut _input = input.to_lowercase().trim().chars().collect::<Vec<char>>();
    _input.sort();

    if _input.len() != 2 {
        return Err("Format is [A-C][1-3]; e.g. A3, B1, and C2");
    }

    Ok([
        match _input[0] {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            _ => return Err("Invalid row number."),
        },
        match _input[1] {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            _ => return Err("Invalid column character."),
        },
    ])
}

pub fn change_cell<'a>(board: &mut Board, cell: &Cell, point: [u8; 2]) -> Result<(), &'a str> {
    let x = point[0] as usize;
    let y = point[1] as usize;

    if x >= 3 && y >= 3 {
        return Err("Value too large");
    }

    if let Cell::EMPTY = board[y as usize][x as usize] {
        board[y][x] = *cell;
        return Ok(());
    }

    Err("Cell already in use")
}

/// in case I want to add a replay feature
// pub fn reset_board(board: &mut Board) {
//     board.iter_mut().for_each(|i| *i = [Cell::EMPTY; 3]);
// }

pub fn draw_game(board: &Board) {
    let columns = ['A', 'B', 'C'];

    println!("{}", "    1    2   3".to_string().bold().bright_green());
    println!("   ┏━━━┳━━━┳━━━┓");

    for i in 0..2 {
        let row = board[i];
        let col = columns[i].to_string().bold().bright_green();

        println!("{col}  ┃ {} ┃ {} ┃ {} ┃ ", row[0], row[1], row[2],);
        println!("   ┣━━━╋━━━╋━━━┫");
    }

    let row = board[2];
    let col = columns[2].to_string().bold().bright_green();

    println!("{col}  ┃ {} ┃ {} ┃ {} ┃ ", row[0], row[1], row[2],);
    println!("   ┗━━━┻━━━┻━━━┛");
}

/// same effect as as `std::process::Command::new("clear").status().unwrap();`
pub fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
