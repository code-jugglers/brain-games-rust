mod board;
mod bot;
mod player;

use board::{Board, BoardSpace, Position};
use bot::Bot;
use player::Player;
use std::io;

fn main() {
    let mut board: Board = Board::empty();

    loop {
        let x = Player::new(BoardSpace::X);
        let o = Bot::new(Player::new(BoardSpace::O));

        let mut user_input = String::new();

        board.print();

        print!("Please Enter your move:");

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let user_position: Vec<&str> = user_input.split(",").map(|pos| pos.trim()).collect();

        x.make_move(
            &mut board,
            Position {
                x: user_position[0].parse::<usize>().unwrap(),
                y: user_position[1].parse::<usize>().unwrap(),
            },
        );
        o.make_move(&mut board);

        board.print();
    }
}
