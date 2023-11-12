use crate::{CIRCLE, EMPTY, X};
use colored::Colorize;
use core::ops::Not;
use std::fmt::{Display, Formatter, Result as FmtResult};

pub type Board = [[Cell; 3]; 3];

#[derive(Copy, Clone, PartialEq)]
pub enum Cell {
    EMPTY,
    P1,
    P2,
}

pub enum GameEnd {
    TIE,
    P1,
    P2,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                Cell::EMPTY => EMPTY.into(),
                Cell::P1 => X.bright_blue(),
                Cell::P2 => CIRCLE.bright_red(),
            }
        )
    }
}

impl Not for Cell {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Self::P1 => Self::P2,
            Self::P2 => Self::P1,
            Self::EMPTY => Self::EMPTY,
        }
    }
}

impl From<Cell> for GameEnd {
    fn from(value: Cell) -> GameEnd {
        match value {
            Cell::P1 => GameEnd::P1,
            Cell::P2 => GameEnd::P2,
            _ => panic!("Can only convert P1 and P2"),
        }
    }
}

impl Display for GameEnd {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(
            f,
            "{}",
            match self {
                GameEnd::P1 => "Player 1 Wins!",
                GameEnd::P2 => "Player 2 Wins!",
                GameEnd::TIE => "It's a tie!",
            }
            .bold()
            .bright_magenta()
        )
    }
}
