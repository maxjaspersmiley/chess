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


    println!("First white pawn forward.");
    cboard.player_move(Coord{row: 6, col: 4}, Coord{row: 5, col: 4});
    cboard.print();
    println!("{}", i);
    i += 1;
    println!("Second white pawn forward.");
    cboard.player_move(Coord{row: 6, col: 7}, Coord{row: 4, col: 7});
    cboard.print();
    println!("{}", i);
    i += 1;
    println!("First black pawn forward.");
    cboard.player_move(Coord{row: 1, col: 6}, Coord{row: 3, col: 6});
    cboard.print();
    println!("{}", i);
    i += 1;
    println!("White pawn takes black pawn.");
    cboard.player_move(Coord{row: 4, col: 7}, Coord{row: 3, col: 6});
    cboard.print();
    println!("{}", i);
    i += 1;
    println!("White pawn moves forward.");
    cboard.player_move(Coord{row: 3, col: 6}, Coord{row: 2, col: 6});
    cboard.print();
    println!("{}", i);
    i += 1;
    println!("Bishop left.");
    cboard.player_move(Coord{row: 7, col: 5}, Coord{row: 2, col: 0});
    cboard.print();
    println!("{}", i);
    i += 1;
    println!("Rook up.");
    cboard.player_move(Coord{row: 7, col: 7}, Coord{row: 1, col: 7});
    cboard.print();
    println!("{}", i);
    i += 1;
    println!("Queen right.");
    cboard.player_move(Coord{row: 7, col: 3}, Coord{row: 3, col: 7});
    cboard.print();
    println!("{}", i);
    i += 1;
    println!("Queen left.");
    cboard.player_move(Coord{row: 3, col: 7}, Coord{row: 3, col: 0});
    cboard.print();
    println!("{}", i);

    println!("King up.");
    cboard.player_move(Coord{row: 7, col: 4}, Coord{row: 6, col: 4});
    cboard.print();
    println!("{}", i);
    println!("King right (should fail).");
    cboard.player_move(Coord{row: 6, col: 4}, Coord{row: 6, col: 5});
    cboard.print();
    println!("{}", i);

    println!("Knight up/left.");
    cboard.player_move(Coord{row: 7, col: 6}, Coord{row: 5, col: 5});
    cboard.print();
    println!("{}", i);
    println!("Knight down/left.");
    cboard.player_move(Coord{row:  5, col: 5}, Coord{row: 7, col: 4});
    cboard.print();
    println!("{}", i);
    println!("Knight left/up (should fail).");
    cboard.player_move(Coord{row:  7, col: 4}, Coord{row: 6, col: 2});
    cboard.print();
    println!("{}", i);
}
