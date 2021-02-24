mod board;
mod bot;
mod play;

use board::{Board, BoardSpaceState, Player};
use bot::Bot;

fn main() {
    let mut board = Board::new();
    let mut player_x = Bot::new(BoardSpaceState::Player(Player::X));
    let mut player_o = Bot::new(BoardSpaceState::Player(Player::O));

    let result = play::play(&mut board, &mut player_x, &mut player_o);

    println!("Winner: {:?}", result);
    println!("{:?}", board.spaces);
}
