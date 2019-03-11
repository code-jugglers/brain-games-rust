mod board;
mod bot;

use board::{Board, BoardSpace, Position};
use bot::Bot;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut board = Board::empty();

    let player = BoardSpace::X;
    let mut bot = Bot::new(BoardSpace::O, HashMap::new());

    loop {
        let mut user_input = String::new();

        board.print();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_position: Vec<usize> = user_input
            .split(",")
            .map(|pos| pos.trim().parse::<usize>().unwrap())
            .collect();

        board.set(
            &player,
            Position {
                row: user_position[0],
                col: user_position[1],
            },
        );

        if board.determine_winner() != BoardSpace::Blank {
            bot.learn(false);

            println!("{} WINS!", board.determine_winner());

            break;
        }

        bot.make_move(&mut board);

        if board.determine_winner() != BoardSpace::Blank {
            bot.learn(true);

            println!("{} WINS!", board.determine_winner());

            break;
        }

        board.print();
    }
}
