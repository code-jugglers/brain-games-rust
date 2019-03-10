mod board;
mod bot;
mod player;

use board::{Board, BoardSpace, Position};
use bot::Bot;
use player::Player;
use std::collections::HashMap;
use std::io;

fn main() {
    let mut board = Board::empty();

    let x = Player::new(BoardSpace::X);
    let o = Bot::new(Player::new(BoardSpace::O), HashMap::new());

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

        x.make_move(
            &mut board,
            Position {
                x: user_position[0],
                y: user_position[1],
            },
        );

        o.make_move(&mut board);

        board.print();
    }
}
