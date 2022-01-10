mod board;
mod bot;
mod train;

use board::{Board, GameResult, Player, Space};
use bot::{Bot, BotConfig};
use clap::Parser;
use std::fs;
use std::io;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, default_value = "train")]
    mode: String,
}

fn main() -> Result<(), ()> {
    let args = Args::parse();

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

    if let Ok(bin) = fs::read("www/bot_x_brain.bin") {
        player_x.load_brain(bin);
    }

    if let Ok(bin) = fs::read("www/bot_o_brain.bin") {
        player_o.load_brain(bin);
    }

    if args.mode == "train" {
        train(&mut player_x, &mut player_o)
    } else if args.mode == "play_as_x" {
        play(Player::X, &mut player_o)
    } else if args.mode == "play_as_o" {
        play(Player::O, &mut player_x)
    } else {
        Err(())
    }
}

fn train(player_x: &mut Bot, player_o: &mut Bot) -> Result<(), ()> {
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

    Ok(())
}

fn play(player: Player, bot: &mut Bot) -> Result<(), ()> {
    let mut board = Board::new();
    let mut current_player: Player = Player::X;

    println!("{}", board);

    loop {
        if current_player == player {
            let input = parse_user_input();

            board.set(input[0], input[1], Space::Player(player));

            println!("{}", board);
        } else {
            let m = bot.determine_move(&board).unwrap();

            board.set_by_index(m, Space::Player(bot.player));

            println!("{}", board);
        }

        if let Some(res) = board.determine_winner() {
            println!("Game Over! Result: {}", res);

            return Ok(());
        }

        // Toggle current player
        current_player = if current_player == Player::X {
            Player::O
        } else {
            Player::X
        };
    }
}

fn parse_user_input() -> Vec<usize> {
    let mut buf = String::new();

    io::stdin()
        .read_line(&mut buf)
        .expect("could not read from stdin");

    buf.trim_matches('\n')
        .split(&['-', '.', ':', ',', '/', ' '][..])
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}
