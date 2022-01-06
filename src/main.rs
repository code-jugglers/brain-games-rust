mod board;
mod bot;
mod train;

use board::{Board, GameResult, Player};
use bot::{Bot, BotConfig};
use std::fs;

fn main() {
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

    // if let Ok(bin) = fs::read("www/bot_x_brain.bin") {
    //     player_x.load_brain(bin);
    // }

    // if let Ok(bin) = fs::read("www/bot_o_brain.bin") {
    //     player_o.load_brain(bin);
    // }

    train(&mut player_x, &mut player_o);
}

fn train(player_x: &mut Bot, player_o: &mut Bot) {
    let mut board = Board::new();

    let mut x_win = 0;
    let mut o_win = 0;
    let mut tie = 0;

    for count in 1..=1_000_000 {
        match train::play(&mut board, player_x, player_o) {
            Ok(GameResult::Winner(Player::X)) => x_win += 1,
            Ok(GameResult::Winner(Player::O)) => o_win += 1,
            Ok(GameResult::Tie) => tie += 1,
            Err(player) => println!(
                "{:?} was unable to find an available move on board:\n {}",
                player, board
            ),
        }

        if count % 10_000 == 0 {
            println!("{}", board);
            println!("============");
            println!("X: {}", x_win);
            println!("O: {}", o_win);
            println!("TIE: {}", tie);
        }

        board = Board::new();
    }

    fs::write("www/bot_x_brain.bin", player_x.export_brain()).expect("Could not save X brain");
    fs::write("www/bot_o_brain.bin", player_o.export_brain()).expect("Could not save O brain");
}
