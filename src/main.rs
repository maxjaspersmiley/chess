//main.rs

pub mod chessboard;
use chessboard::Chessboard;
use crate::chessboard::chessmen::Coord;

fn main() {
    let mut i = 0;
    let mut cboard = Chessboard::new();
    /*
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 6, col: 4}, Coord{row: 5, col: 4});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 1, col: 4}, Coord{row: 3, col: 4});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 6, col: 5}, Coord{row: 4, col: 5});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 3, col: 4}, Coord{row: 4, col: 5});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 5, col: 4}, Coord{row: 4, col: 5});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 4, col: 5}, Coord{row: 1, col: 1});
    cboard.print();
    println!("{}", i);
    i += 1;
    */
    
    cboard.print();
    println!("{}", i);
    i += 1;


    cboard.player_move(Coord{row: 6, col: 4}, Coord{row: 5, col: 4});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 6, col: 7}, Coord{row: 4, col: 7});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 1, col: 6}, Coord{row: 3, col: 6});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 4, col: 7}, Coord{row: 3, col: 6});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 3, col: 6}, Coord{row: 2, col: 6});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 7, col: 5}, Coord{row: 2, col: 0});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 7, col: 7}, Coord{row: 1, col: 7});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 7, col: 3}, Coord{row: 3, col: 7});
    cboard.print();
    println!("{}", i);
    i += 1;
    cboard.player_move(Coord{row: 3, col: 7}, Coord{row: 3, col: 0});
    cboard.print();
    println!("{}", i);

}
