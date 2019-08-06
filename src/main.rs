//main.rs

pub mod chessboard;
use chessboard::Chessboard;

fn main() {
    let cboard = Chessboard::new();
    cboard.print();
}
