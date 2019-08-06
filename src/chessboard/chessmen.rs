#[derive(Copy, Clone, Debug)]
pub enum Chessman {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

pub struct Coord {
    row: u8,
    col: u8,
}

pub fn print(c: Option<Chessman>) -> String {
    match c {
        Some(Chessman::Pawn)    => "p".to_string(),
        Some(Chessman::Rook)    => "R".to_string(),
        Some(Chessman::Knight)  => "K".to_string(),
        Some(Chessman::Bishop)  => "B".to_string(),
        Some(Chessman::Queen)   => "Q".to_string(),
        Some(Chessman::King)    => "&".to_string(),
        None                    => " ".to_string(),
    }
}

pub fn move_chessman(c: Chessman, t: Option<Chessman>, start: Coord, end: Coord) -> bool {
    match c {
        Chessman::Pawn      => {
            match start.row as i32 -end.row as i32 {
                0   => match 
                -1  => ,
                1   => ,
        }
        Chessman::Rook      => {

        }
        Chessman::Knight    => {

        }
        Chessman::Bishop    => {

        }
        Chessman::Queen     => {

        }
        Chessman::King      => {

        }
}


