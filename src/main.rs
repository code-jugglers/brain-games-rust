mod board;
mod player;

use board::{Board, BoardSpace, Position};
use player::Player;

fn main() {
    let mut board: Board = Board::empty();

    let x = Player::new(BoardSpace::X);
    let o = Player::new(BoardSpace::O);

    x.make_move(&mut board, Position { x: 0, y: 0 });
    o.make_move(&mut board, Position { x: 0, y: 1 });

    board.print();
}
