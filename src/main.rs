mod board;
mod bot;

use board::{Board, BoardSpace};
use bot::Bot;
use std::collections::HashMap;

fn main() {
    let mut board = Board::empty();

    // let player = BoardSpace::X;
    // let mut bot = Bot::new(BoardSpace::O, HashMap::new());

    // loop {
    //     let mut user_input = String::new();

    //     board.print();

    //     io::stdin()
    //         .read_line(&mut user_input)
    //         .expect("Failed to read line");

    //     let user_position: Vec<usize> = user_input
    //         .split(",")
    //         .map(|pos| pos.trim().parse::<usize>().unwrap())
    //         .collect();

    //     board.set(
    //         &player,
    //         Position {
    //             row: user_position[0],
    //             col: user_position[1],
    //         },
    //     );

    //     if board.determine_winner() != BoardSpace::Blank {
    //         bot.learn(false);

    //         println!("{} WINS!", board.determine_winner());

    //         break;
    //     }

    //     bot.make_move(&mut board);

    //     if board.determine_winner() != BoardSpace::Blank {
    //         bot.learn(true);

    //         println!("{} WINS!", board.determine_winner());

    //         break;
    //     }

    //     board.print();
    // }

    let mut bot1 = Bot::new(BoardSpace::X, HashMap::new());
    let mut bot2 = Bot::new(BoardSpace::O, HashMap::new());

    for _ in 0..=10 {
        loop {
            bot1.make_move(&mut board);

            if board.determine_winner() == BoardSpace::X {
                board.print();
                bot1.learn(true);
                bot2.learn(false);

                println!("===");
                println!("{} WINS!", BoardSpace::X);

                break;
            }

            bot2.make_move(&mut board);

            if board.determine_winner() == BoardSpace::O {
                board.print();
                bot1.learn(false);
                bot2.learn(true);

                println!("===");
                println!("{} WINS!", BoardSpace::O);

                break;
            }
        }
    }
}
