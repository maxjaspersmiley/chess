use crate::chessboard::chessmen::Piece::*;
use crate::chessboard::chessmen::Color::*;
use colored::Colorize;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Coord {
    pub fn new(r: usize, c: usize) -> Coord {
        Coord{row: r, col: c}
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            White => write!(f, "White"),
            Black => write!(f, "Black"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Chessman {
    pub color: Color,
    pub piece: Piece,
}

pub fn print(c: Option<Chessman>) -> colored::ColoredString {
    match c {
        Some(m) => {
            let color = match m.color {
                White => "blue",
                Black => "red", 
            };
            match m.piece {
                Pawn    => "p".color(color).on_white().bold(),
                Rook    => "R".color(color).on_white().bold(),
                Knight  => "K".color(color).on_white().bold(),
                Bishop  => "B".color(color).on_white().bold(),
                Queen   => "Q".color(color).on_white().bold(),
                King    => "&".color(color).on_white().bold(),
            }
        }
        None         => " ".on_white(),
    }
}

pub fn move_chessman(c: Chessman, t: Option<Chessman>, start: Coord, end: Coord) -> bool {
    match c.piece {
        Pawn    => {
            //is this a move that this piece can theoretically make? 

            //check conditions of movement (just pawn i think?)

            match start.col as i32 - end.col as i32 {
                0 => {
                    if c.color == White 
                        && (start.row as i32 - end.row as i32 == 1 
                        || (start.row as i32 - end.row as i32 == 2 && start.row == 6)) {
                            true
                    }
                    else if c.color == Black 
                        && (end.row as i32 - start.row as i32 == 1
                        || (end.row as i32 - start.row as i32 == 2 && start.row == 1)){
                            true
                    }
                    else {
                        false
                    }
                }
                -1 | 1 => {
                    if c.color == White && start.row as i32 - end.row as i32 == 1 {
                        match t {
                            Some(_) => true,
                            None    => false,
                        }
                    }
                    else if c.color == Black && end.row as i32 - start.row as i32 == 1 {
                        match t {
                            Some(_) => true,
                            None    => false,
                        }
                    }
                    else {
                        false
                    }
                }
                _ => false,
            }
        }
        Rook    => {
            if !(start == end) 
                && (start.row == end.row || start.col == end.col) {
                    true
                }
            else {
                false
            }
        }
        Knight  => {
            let a = start.col as i32 - end.col as i32;
            match start.row as i32 - end.row as i32 {
                -2 => {
                    if a == 1 || a == -1 {true}
                    else {false}
                }
                -1 => {
                    if a == 2 || a == -2 {true}
                    else {false}
                }
                1  => {
                    if a == 2 || a == -2 {true}
                    else {false}
                }
                2  => {
                    if a == 1 || a == -1 {true}
                    else {false}
                }
                _  => false,
            }
        }

        Bishop  => {
            if start == end {
                false
            }
            else {
                let diff = start.row as i32- start.col as i32;
                let sum  = start.row + start.col;
                if end.row as i32- end.col as i32 == diff || end.row + end.col == sum {
                    true
                }
                else {
                    false
                }
            }
        }

        Queen   => {
            if start == end { 
                false
            }
            else {
                let diff = start.row as i32 - start.col as i32;
                let sum  = start.row + start.col;
                if end.row as i32 - end.col as i32 == diff || end.row + end.col == sum {
                    true
                }
                else if start.row == end.row || start.col == end.col {
                    true
                }
                else {
                    false
                }
            }
        }
        King    => {
            if start == end { 
                false
            }
            else {
                let dist_y = start.row as i32 - end.row as i32;
                let dist_x = start.col as i32 - end.col as i32;
                if dist_y < -1 || dist_y > 1 || dist_x < -1 || dist_y > 1 {
                    false
                }
                else {
                    true
                }
            }
        }
    }
}


