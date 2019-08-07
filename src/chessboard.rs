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
    
    pub fn check_ownership(&self, loc: Coord, color: Color) -> bool {
        match self.board[loc.row][loc.col]{
            None    => false,
            Some(a) => a.color == color,
        }
    }


    fn check_attack(&self, c: Chessman, loc: Coord) -> Vec<Coord> {
        let mut attacks = Vec::new();
        let row = loc.row as i8;
        let col = loc.col as i8;
        match c.piece {
            Pawn    => {
                for i in -1..=1 {
                    let a_col = col + i;
                    if a_col >= 0 && a_col < 8 {
                        let dir = match c.color {
                            White => -1,
                            Black =>  1,
                        };
                        if i != 0 {
                            match self.board[(row + dir) as usize][a_col as usize] {
                                Some(p) => {
                                    if p.color == c.color {
                                        continue;
                                    }
                                    else {
                                        attacks.push(Coord::new((row + dir) as usize, a_col as usize));
                                    }
                                }
                                None    => (),
                            }
                        }
                        else {
                            match self.board[(row + dir) as usize][a_col as usize] {
                                Some(_) => (),
                                None    => attacks.push(Coord::new((row + dir) as usize, a_col as usize)),
                            }
                            let start_row = match c.color {
                                White => 6,
                                Black => 1,
                            };
                            if row == start_row {
                                match self.board[(row + 2 * dir) as usize][a_col as usize] {
                                    Some(_) => (),
                                    None    => attacks.push(Coord::new((row + 2 * dir) as usize, a_col as usize)),
                                }
                            }
                        }
                    }
                }
            }

            Rook    => {
                //look to our left (starting with closest square)
                for l in (0..loc.col).rev() {
                    match self.board[loc.row][l] {
                        Some(p) => {
                            //we found an ally
                            if p.color == c.color {
                                break;
                            }
                            //we found an enemy
                            else {
                                attacks.push(Coord::new(loc.row, l));
                                break;
                            }
                        }
                        None    => attacks.push(Coord::new(loc.row, l)),
                    }
                }
                //llook to our right (starting with closest square)
                for r in (loc.col + 1)..8 {
                    match self.board[loc.row][r] {
                        Some(p) => {
                            //we found an ally
                            if p.color == c.color {
                                break;
                            }
                            else {
                                attacks.push(Coord::new(loc.row, r));
                                break;
                            }
                        }
                        None    => attacks.push(Coord::new(loc.row, r)),
                    }
                }
                //look above us (starting with closest square)
                for u in (0..loc.row).rev() {
                    match self.board[u][loc.col] {
                        Some(p) => {
                            //we found an ally
                            if p.color == c.color {
                                break;
                            }
                            else {
                                attacks.push(Coord::new(u, loc.col));
                                break;
                            }
                        }
                        None    => attacks.push(Coord::new(u, loc.col)),
                    }
                }
                //look below us (starting with closest square)
                for d in (loc.row + 1)..8 {
                    match self.board[d][loc.col] {
                        Some(p) => {
                            if p.color == c.color {
                                break;
                            }
                            else {
                                attacks.push(Coord::new(d, loc.col));
                                break;
                            }
                        }
                        None    => attacks.push(Coord::new(d, loc.col)),
                    }
                }
            }

            Bishop  => {
                let mut ul = true;
                let mut ur = true;
                let mut dl = true;
                let mut dr = true;
                for i in 1..8 {
                    if row - i >= 0 && col - i >= 0 && ul {
                        match self.board[(row - i) as usize][(col - i) as usize] {
                            Some(p) => {
                                if p.color == c.color {ul = false;}
                                else {
                                    attacks.push(Coord::new((row - i) as usize, (col - i) as usize));
                                }
                            }
                            None    => attacks.push(Coord::new((row - i) as usize, (col - i) as usize))
                        }
                    }
                    if row - i >= 0 && col + i < 8 && ur {
                        match self.board[(row - i) as usize][(col + i) as usize] {
                            Some(p) => {
                                if p.color == c.color {ur = false;}
                                else {
                                    attacks.push(Coord::new((row - i) as usize, (col + i) as usize));
                                    ur = false;
                                }
                            }
                            None    => attacks.push(Coord::new((row - i) as usize, (col + i) as usize)),
                        }
                    }
                    if row + i < 8 && col - i >= 0 && dl {
                        match self.board[(row + i) as usize][(col - i) as usize] {
                            Some(p) => {
                                if p.color == c.color {dl = false;}
                                else {
                                    attacks.push(Coord::new((row + i) as usize, (col - i) as usize));
                                    dl = false;
                                }
                            }
                            None    => attacks.push(Coord::new((row + i) as usize, (col - i) as usize)),
                        }
                    }
                    if row + i < 8 && col + i < 8 && dr {
                        match self.board[(row + i) as usize][(col + i) as usize] {
                            Some(p) => {
                                if p.color == c.color {dr = false;}
                                else {
                                    attacks.push(Coord::new((row + i) as usize, (col + i) as usize));
                                    dr = false;
                                }
                            }
                            None    => attacks.push(Coord::new((row + i) as usize, (col + i) as usize)),
                        }
                    }
                }
            }

            Knight  => {
                for i in -2..=2 {
                    if i == 0 { continue; }
                    let j;
                    if i == -2 || i == 2 {
                        j = 1;
                    }
                    else {
                        j = 2;
                    }
                    if row + i >= 0 && row + i < 8 && col + j < 8{
                        match self.board[(row + i) as usize][(col + j) as usize] {
                            Some(p) => {
                                if p.color == c.color {
                                    continue;
                                }
                                else {
                                    attacks.push(Coord::new((row + i) as usize, (col + j) as usize));
                                }
                            }
                            None    => attacks.push(Coord::new((row + i) as usize, (col + j) as usize)),
                        }
                    }
                    if row + i >= 0 && row + i < 8 && col - j >= 0{
                        match self.board[(row + i) as usize][(col - j) as usize] {
                            Some(p) => {
                                if p.color == c.color {
                                    continue;
                                }
                                else {
                                    attacks.push(Coord::new((row + i) as usize, (col - j) as usize));
                                }
                            }
                            None    => attacks.push(Coord::new((row + i) as usize, (col - j) as usize)),
                        }
                    }
                }

            }
            Queen   => {
                //look to our left (starting with closest square)
                for l in (0..loc.col).rev() {
                    match self.board[loc.row][l] {
                        Some(p) => {
                            //we found an ally
                            if p.color == c.color {
                                break;
                            }
                            //we found an enemy
                            else {
                                attacks.push(Coord::new(loc.row, l));
                                break;
                            }
                        }
                        None    => attacks.push(Coord::new(loc.row, l)),
                    }
                }
                //llook to our right (starting with closest square)
                for r in (loc.col + 1)..8 {
                    match self.board[loc.row][r] {
                        Some(p) => {
                            //we found an ally
                            if p.color == c.color {
                                break;
                            }
                            else {
                                attacks.push(Coord::new(loc.row, r));
                                break;
                            }
                        }
                        None    => attacks.push(Coord::new(loc.row, r)),
                    }
                }
                //look above us (starting with closest square)
                for u in (0..loc.row).rev() {
                    match self.board[u][loc.col] {
                        Some(p) => {
                            //we found an ally
                            if p.color == c.color {
                                break;
                            }
                            else {
                                attacks.push(Coord::new(u, loc.col));
                                break;
                            }
                        }
                        None    => attacks.push(Coord::new(u, loc.col)),
                    }
                }
                //look below us (starting with closest square)
                for d in (loc.row + 1)..8 {
                    match self.board[d][loc.col] {
                        Some(p) => {
                            if p.color == c.color {
                                break;
                            }
                            else {
                                attacks.push(Coord::new(d, loc.col));
                                break;
                            }
                        }
                        None    => attacks.push(Coord::new(d, loc.col)),
                    }
                }
                let mut ul = true;
                let mut ur = true;
                let mut dl = true;
                let mut dr = true;
                for i in 1..8 {
                    if row - i >= 0 && col - i >= 0 && ul {
                        match self.board[(row - i) as usize][(col - i) as usize] {
                            Some(p) => {
                                if p.color == c.color {ul = false;}
                                else {
                                    attacks.push(Coord::new((row - i) as usize, (col - i) as usize));
                                }
                            }
                            None    => attacks.push(Coord::new((row - i) as usize, (col - i) as usize))
                        }
                    }
                    if row - i >= 0 && col + i < 8 && ur {
                        match self.board[(row - i) as usize][(col + i) as usize] {
                            Some(p) => {
                                if p.color == c.color {ur = false;}
                                else {
                                    attacks.push(Coord::new((row - i) as usize, (col + i) as usize));
                                    ur = false;
                                }
                            }
                            None    => attacks.push(Coord::new((row - i) as usize, (col + i) as usize)),
                        }
                    }
                    if row + i < 8 && col - i >= 0 && dl {
                        match self.board[(row + i) as usize][(col - i) as usize] {
                            Some(p) => {
                                if p.color == c.color {dl = false;}
                                else {
                                    attacks.push(Coord::new((row + i) as usize, (col - i) as usize));
                                    dl = false;
                                }
                            }
                            None    => attacks.push(Coord::new((row + i) as usize, (col - i) as usize)),
                        }
                    }
                    if row + i < 8 && col + i < 8 && dr {
                        match self.board[(row + i) as usize][(col + i) as usize] {
                            Some(p) => {
                                if p.color == c.color {dr = false;}
                                else {
                                    attacks.push(Coord::new((row + i) as usize, (col + i) as usize));
                                    dr = false;
                                }
                            }
                            None    => attacks.push(Coord::new((row + i) as usize, (col + i) as usize)),
                        }
                    }
                }
            }
            King    => {
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 { continue; }
                        let mr = row + i;
                        let mc = col + j;
                        if mr >= 0 && mr < 8 && mc >= 0 && mc < 8 {
                            match self.board[mr as usize][mc as usize] {
                                Some(p) => {
                                    if p.color == c.color {()}
                                    else {
                                        attacks.push(Coord::new(mr as usize, mc as usize));
                                    }
                                }
                                None    => attacks.push(Coord::new(mr as usize, mc as usize)),
                            }
                        }
                    }
                }
            }
        }
        attacks
    }


    pub fn player_move(&mut self, start: Coord, end: Coord) -> Option<bool> {
        //check the piece at start location. if empty, failure
        match self.board[start.row][start.col] {
            None    => None,
            Some(c) => {
                //ask piece to decide if its rules support a move to end location. if not, failure
                match move_chessman(c, self.board[end.row][end.col], start, end){
                    false => None,
                    true => {
                        let attacks: Vec<Coord> = self.check_attack(c, start);
                        println!("attacks: {:?}", attacks);
                        if !attacks.contains(&end) {
                            None
                        }
                        else {
                            self.board[end.row][end.col] = self.board[start.row][start.col];
                            self.board[start.row][start.col] = None;
                            Some(false)
                        }
                    }
                }
            }
        }
    }

    pub fn print(&self) {
        for row in 0..8 {
            if row == 0{
                for i in 0..49 {
                    if (i - 3) % 6 == 0 {
                        print!("{}", (((((i - 3) / 6) + 65) as u8) as char).to_string().black().on_white());
                    }
                    else{
                        print!("{}", "-".black().on_white());
                    }
                    if i == 48 {
                        print!("{}", "   ".black().on_white());
                    }
                }
            }
            else {
                for i in 0..49 {
                    if i % 6 == 0 {
                        print!("{}", "|".black().on_white());
                        if i == 48 {
                            print!("{}", "   ".black().on_white());
                        }
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
                    println!("{}{}{}", "| ".black().on_white(), (8 - row).to_string().black().on_white(), " ".black().on_white());
                }
            }
        }
        for i in 0..49 {
            print!("{}", "-".black().on_white());
            if i == 48 {
                print!("{}", "   ".black().on_white());
            }
        }
        println!("");
    }
}

