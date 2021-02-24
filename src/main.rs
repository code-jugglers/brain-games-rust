mod board;
mod bot;
mod play;

use board::{Board, BoardSpaceState, Player};
use bot::Bot;

fn main() {
    let mut board = Board::new();
    let mut player_x = Bot::new(BoardSpaceState::Player(Player::X));
    let mut player_o = Bot::new(BoardSpaceState::Player(Player::O));

    let mut x_win = 0;
    let mut o_win = 0;
    let mut tie = 0;

    for _ in 1..=3000000 {
        let res = play::play(&mut board, &mut player_x, &mut player_o);

        if let Some(res) = res {
            if res == play::GameResult::XWin {
                x_win += 1;
            } else if res == play::GameResult::OWin {
                o_win += 1;
            } else {
                tie += 1;
            }
        }

        board = Board::new();
    }

    println!("X: {:?}", x_win);
    println!("O: {:?}", o_win);
    println!("TIE: {:?}", tie);
}
