pub mod chessmen;
use crate::chessboard::chessmen::Chessman;
use crate::chessboard::chessmen::Chessman::*;
use chessboard::chessmen::Coord;

pub struct Chessboard {
    board: [[Option<Chessman>; 8]; 8] 
}

impl Chessboard {
    pub fn new() -> Chessboard {
        let mut board = [[None; 8]; 8];
        for col in 0..8 {
            board[1][col] = Some(Pawn);
            board[6][col] = Some(Pawn);
        }

        board[0][0] = Some(Rook);
        board[0][7] = Some(Rook);
        board[7][0] = Some(Rook);
        board[7][7] = Some(Rook);

        board[0][1] = Some(Knight); 
        board[0][6] = Some(Knight);
        board[7][1] = Some(Knight);
        board[7][6] = Some(Knight);

        board[0][2] = Some(Bishop);
        board[0][5] = Some(Bishop);
        board[7][2] = Some(Bishop);
        board[7][5] = Some(Bishop);

        board[0][3] = Some(Queen);
        board[0][4] = Some(King);
        
        board[7][3] = Some(Queen);
        board[7][4] = Some(King);

        Chessboard{board}
    }

    pub fn player_move(&self, start: Coord, end: Coord) -> bool{
        match self.board[start.row][start.col] {
            None    => false,
            Some(c) => {
                match move_chessman(c, self.board[end.row][end.col], start, end){
                    false => false,
                    true => {
                        self.board[end.row][end.col] = self.board[start.row][start.col];
                        self.board[start.row][start.col] = None;
                        true
                    }
                }
            }
        }
    }























    pub fn print(&self) {
        for row in 0..8 {
            if row == 0{
                for _ in 0..49 {
                    print!("-");
                }
            }
            else {
                for i in 0..49 {
                    if i % 6 == 0 {
                        print!("|");
                    }
                    else {
                        print!("-");
                    }
                }
            }
            println!("");
            for col in 0..8 {
                print!("|  {}  ", chessmen::print(self.board[row][col]));
                if col == 7 {
                    println!("|")
                }
            }
        }
        for _ in 0..49 {
            print!("-");
        }
        println!("");
    }
}

