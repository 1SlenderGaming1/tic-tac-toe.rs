mod types;
mod util;
use colored::Colorize;
use std::io::{stdout, Write};
use types::*;
use util::*;

const CIRCLE: &str = "";
const X: &str = "";
const EMPTY: &str = " ";

fn main() {
    let mut board: Board = [[Cell::EMPTY; 3]; 3];
    let mut player = Cell::P1;
    let mut turns: u8 = 0;

    // draw before loop for error handling
    clear_terminal();
    draw_game(&board);

    loop {
        if let Some(end) = check_win(&board, &turns) {
            println!("{end}");
            break;
        }

        print!("Enter Direction:  ");
        let _ = stdout().flush();

        let direction = match input_to_direction() {
            Ok(t) => t,
            Err(e) => {
                println!("Input Error: {}", e);
                continue;
            }
        };

        // let direction get dropped
        if let Err(e) = change_cell(&mut board, &player, direction) {
            println!("{}: {}\n", "[ERROR]".red(), e);
            continue;
        }

        player = !player;
        turns += 1;

        clear_terminal();
        draw_game(&board);
    }

    // if let Some(end) = check_win(&board, &turns) {
    //     println!("\n\n{end}");
    // };
}
