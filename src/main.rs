mod board;
mod bot;
mod play;

use board::{Board, GameResult, Player};
use bot::{Bot, BotConfig};
use std::fs;

fn main() {
    let mut board = Board::new();

    let mut player_x = Bot::new(BotConfig {
        player: Player::X,
        winning_move_boost: None,
        win_boost: None,
        loose_boost: None,
        tie_boost: None,
    });

    let mut player_o = Bot::new(BotConfig {
        player: Player::O,
        winning_move_boost: None,
        win_boost: None,
        loose_boost: None,
        tie_boost: None,
    });

    let mut x_win = 0;
    let mut o_win = 0;
    let mut tie = 0;

    for count in 1..=500000 {
        match play::play(&mut board, &mut player_x, &mut player_o) {
            Some(GameResult::Winner(Player::X)) => x_win += 1,
            Some(GameResult::Winner(Player::O)) => o_win += 1,
            Some(GameResult::Tie) => tie += 1,
            None => {}
        }

        if count % 10000 == 0 {
            println!("{}", board);
            println!("=======");
            println!("X: {:?}", x_win);
            println!("O: {:?}", o_win);
            println!("TIE: {:?}", tie);
        }

        board = Board::new();
    }

    fs::write("www/bot_x_brain.bin", player_x.export_brain()).unwrap();
    fs::write("www/bot_o_brain.bin", player_o.export_brain()).unwrap();
}
