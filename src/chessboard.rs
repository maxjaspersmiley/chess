pub mod chessmen;
use colored::Colorize;
use crate::chessboard::chessmen::*;
use crate::chessboard::chessmen::Chessman;
use crate::chessboard::chessmen::Coord;
use crate::chessboard::chessmen::Color::*;
use crate::chessboard::chessmen::Piece::*;

pub struct Chessboard {
    board: [[Option<Chessman>; 8]; 8] 
}

impl Chessboard {
    pub fn new() -> Chessboard {
        let mut board = [[None; 8]; 8];
        for col in 0..8 {
            board[1][col] = Some(Chessman{color: Black, piece: Pawn});
            board[6][col] = Some(Chessman{color: White, piece: Pawn});
        }

        board[0][0] = Some(Chessman{color: Black, piece: Rook});
        board[0][7] = Some(Chessman{color: Black, piece: Rook});
        board[7][0] = Some(Chessman{color: White, piece: Rook});
        board[7][7] = Some(Chessman{color: White, piece: Rook});

        board[0][1] = Some(Chessman{color: Black, piece: Knight}); 
        board[0][6] = Some(Chessman{color: Black, piece: Knight}); 
        board[7][1] = Some(Chessman{color: White, piece: Knight}); 
        board[7][6] = Some(Chessman{color: White, piece: Knight}); 

        board[0][2] = Some(Chessman{color: Black, piece: Bishop});
        board[0][5] = Some(Chessman{color: Black, piece: Bishop});
        board[7][2] = Some(Chessman{color: White, piece: Bishop});
        board[7][5] = Some(Chessman{color: White, piece: Bishop});

        board[0][3] = Some(Chessman{color: Black, piece: Queen});
        board[0][4] = Some(Chessman{color: Black, piece: King});
        
        board[7][3] = Some(Chessman{color: White, piece: Queen});
        board[7][4] = Some(Chessman{color: White, piece: King});

        Chessboard{board}
    }

    fn check_path(& self, start: Coord, end: Coord) -> bool {
        let diff_row = start.row as i32 - end.row as i32; //positive is moving up.
        let diff_col = end.col as i32 - start.col as i32; //positive is to the right.
        


    pub fn player_move(&mut self, start: Coord, end: Coord) -> Option<bool> {
        //check the piece at start location. if empty, failure
        match self.board[start.row][start.col] {
            None    => None,
            Some(c) => {
                //ask piece to decide if its rules support a move to end location. if not, failure
                match move_chessman(c, self.board[end.row][end.col], start, end){
                    false => None,
                    true => {
                        //check the contents of the destination square. if an ally, failure.
                        match self.board[end.row][end.col] {
                            None    => None,
                            Some(p) => {
                                if p.color == c.color {
                                    None
                                }
                                else {
                                    //check that the path to the destination square is not
                                    //obstructed except if piece is knight or king



                                    /*
                                    if p.piece == King {
                                        Some(true)
                                    }
                                    else {
                                        self.board[end.row][end.col] = self.board[start.row][start.col];
                                        self.board[start.row][start.col] = None;
                                        Some(false)
                                    }
                                    */
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    







    pub fn print(&self) {
        for row in 0..8 {
            if row == 0{
                for _ in 0..49 {
                    print!("{}", "-".black().on_white());
                }
            }
            else {
                for i in 0..49 {
                    if i % 6 == 0 {
                        print!("{}", "|".black().on_white());
                    }
                    else {
                        print!("{}", "-".black().on_white());
                    }
                }
            }
            println!("");
            for col in 0..8 {
                print!("{}{}{}", "|  ".black().on_white(), chessmen::print(self.board[row][col]), "  ".black().on_white());
                if col == 7 {
                    println!("{}", "|".black().on_white())
                }
            }
        }
        for _ in 0..49 {
            print!("{}", "-".black().on_white());
        }
        println!("");
    }
}

