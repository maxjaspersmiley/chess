//main.rs

pub mod chessboard;
use chessboard::Chessboard;
use crate::chessboard::chessmen::Coord;
use crate::chessboard::chessmen::Color::*;
use crate::chessboard::chessmen::Color;

fn move_loop(mut cboard: Chessboard) {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n ");
    println!("*************************************");
    println!("*************************************");
    println!("********                     ********");
    println!("********  Welcome to Chess!  ********");
    println!("********                     ********");
    println!("*************************************");
    println!("*************************************");
    println!("\nA move looks like this:");
    println!("White player, enter your move:");
    println!("White player, enter your move: e2 e4");
    println!("Good luck, and enjoy!\n\n");

    let mut active_player: Color = White;

    loop {
        match active_player {
            White => cboard.print_white(),
            Black => cboard.print_black(),
        }
        let r_str = format!("{} player, enter your move: ", active_player);
        let reply = rprompt::prompt_reply_stdout(&r_str);
        match reply {
            Ok(a)   => {
                let reply: Vec<char> = a.chars().collect();
                println!("len: {}", reply.len());
                let fr = reply[1].to_digit(10);
                let mut from_row = match fr {
                    Some(b) => {
                        if b > 0 && b <= 8 {
                            b
                        }
                        else {
                            println!("Invalid input 1. Try again.\n");
                            continue;
                        }
                    }
                    None    => {
                        println!("Invalid input 2. Try again.\n");
                        continue;
                    }
                };

                let tr   = reply[4].to_digit(10);
                let mut to_row = match tr {
                    Some(b) => {
                        if b > 1 && b <= 8 {
                            b
                        }
                        else {
                            println!("Invalid input 3. Try again.\n");
                            continue;
                        }
                    }
                    None    => {
                        println!("Invalid input 4. Try again.\n");
                        continue;
                    }
                };

                let from_col = match reply[0] {
                    'a' => 0,
                    'b' => 1,
                    'c' => 2,
                    'd' => 3,
                    'e' => 4,
                    'f' => 5,
                    'g' => 6,
                    'h' => 7,
                    _   => {
                        println!("Invalid input 5. Try again.\n");
                        continue;
                    }
                };
                let to_col = match reply[3] {
                    'a' => 0,
                    'b' => 1,
                    'c' => 2,
                    'd' => 3,
                    'e' => 4,
                    'f' => 5,
                    'g' => 6,
                    'h' => 7,
                    _   => {
                        println!("Invalid input 6. Try again.\n");
                        continue;
                    }
                };
                from_row = 8 - from_row;
                to_row   = 8 - to_row;
                println!("r:{} c:{} -> r:{} c:{}", from_row, from_col, to_row, to_col);
                if !cboard.check_ownership(Coord::new(from_row as usize, from_col as usize), active_player){
                    println!("You can't move the other player's pieces! Try again.");
                    continue;
                }
                match cboard.player_move(Coord::new(from_row as usize, from_col as usize), Coord::new(to_row as usize, to_col as usize)) {
                    None => {
                        println!("Invalid input 7. Try again.\n");
                        continue;
                    }
                    Some(_) => {
                        println!("{}{} to {}{} confirmed.\n", reply[0], reply[1], reply[3], reply[4]);
                        active_player = match active_player {
                            White => Black,
                            Black => White,
                        };
                    }
                }
            }
            Err(_)  => {
                println!("Invalid input 8. Try again.\n");
                continue;
            }
        }
    }
}

fn main() {
    let cboard = Chessboard::new();
    move_loop(cboard);
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
    cboard.print();
    println!("{}", i);
    i += 1;
//////////
    println!("First white pawn forward.");
    cboard.player_move(Coord{row: 6, col: 4}, Coord{row: 5, col: 4});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("Second white pawn forward.");
    cboard.player_move(Coord{row: 6, col: 7}, Coord{row: 4, col: 7});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("First black pawn forward.");
    cboard.player_move(Coord{row: 1, col: 6}, Coord{row: 3, col: 6});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("White pawn takes black pawn.");
    cboard.player_move(Coord{row: 4, col: 7}, Coord{row: 3, col: 6});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("White pawn moves forward.");
    cboard.player_move(Coord{row: 3, col: 6}, Coord{row: 2, col: 6});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("Bishop left.");
    cboard.player_move(Coord{row: 7, col: 5}, Coord{row: 2, col: 0});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("Rook up.");
    cboard.player_move(Coord{row: 7, col: 7}, Coord{row: 1, col: 7});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("Queen right.");
    cboard.player_move(Coord{row: 7, col: 3}, Coord{row: 3, col: 7});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("Queen left.");
    cboard.player_move(Coord{row: 3, col: 7}, Coord{row: 3, col: 0});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("King up.");
    cboard.player_move(Coord{row: 7, col: 4}, Coord{row: 6, col: 4});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("King right (should fail).");
    cboard.player_move(Coord{row: 6, col: 4}, Coord{row: 6, col: 5});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("Knight up/left.");
    cboard.player_move(Coord{row: 7, col: 6}, Coord{row: 5, col: 5});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("Knight down/left.");
    cboard.player_move(Coord{row:  5, col: 5}, Coord{row: 7, col: 4});
    cboard.print();
    println!("{}", i);
    println!("");
    println!("");
    i += 1;
    println!("Knight left/up (should fail).");
    cboard.player_move(Coord{row:  7, col: 4}, Coord{row: 6, col: 2});
    cboard.print();
    println!("{}", i);
    */
}
