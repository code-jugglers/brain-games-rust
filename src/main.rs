mod board;
mod bot;
mod train;

use clap::Parser;
use std::fs::{read, write};
use std::io;

use crate::board::{Board, GameResult, Player, Space};
use crate::bot::{Bot, BotConfig};

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, default_value = "train")]
    mode: String,
    #[clap(short, long, default_value = "1000000")]
    game_count: u32,
    #[clap(short, long, default_value = "3")]
    rows: usize,
    #[clap(short, long, default_value = "3")]
    cols: usize,
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

    if let Ok(bin) = read(format!("www/bot_{}x{}_x_brain.bin", args.rows, args.cols)) {
        player_x.load_brain(bin);
    }

    if let Ok(bin) = read(format!("www/bot_{}x{}_o_brain.bin", args.rows, args.cols)) {
        player_o.load_brain(bin);
    }

    if args.mode == "train" {
        train(&mut player_x, &mut player_o, &args)
    } else if args.mode == "play_as_x" {
        play(Player::X, &mut player_o, &args)
    } else if args.mode == "play_as_o" {
        play(Player::O, &mut player_x, &args)
    } else {
        Err(())
    }
}

fn train(player_x: &mut Bot, player_o: &mut Bot, args: &Args) -> Result<(), ()> {
    let mut board = Board::new(args.rows, args.cols);

    let mut x_win = 0;
    let mut o_win = 0;
    let mut tie = 0;

    for count in 1..=args.game_count {
        let game_res = train::play(&mut board, player_x, player_o);

        match game_res {
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
            println!("Result: {:?}", game_res);
            println!("X: {}", x_win);
            println!("O: {}", o_win);
            println!("TIE: {}", tie);
        }

        board = Board::new(args.rows, args.cols);
    }

    write(
        format!("www/bot_{}x{}_x_brain.bin", args.rows, args.cols),
        player_x.export_brain(),
    )
    .expect("Could not save X brain");

    write(
        format!("www/bot_{}x{}_o_brain.bin", args.rows, args.cols),
        player_o.export_brain(),
    )
    .expect("Could not save O brain");

    Ok(())
}

fn play(player: Player, bot: &mut Bot, args: &Args) -> Result<(), ()> {
    let mut board = Board::new(args.rows, args.cols);
    let mut current_player: Player = Player::X;

    println!("{}", board);

    loop {
        if current_player == player {
            let mut complete = false;

            while !complete {
                let input = parse_user_input();

                if board.set(input[0], input[1], Space::Player(player)).is_ok() {
                    println!("{}", board);

                    complete = true;
                } else {
                    println!("Move is invalid, Try again");
                }
            }
        } else {
            let m = bot.determine_move(&board).unwrap();

            // This should be safe since the bot should not be able to make an invalid move
            board.set_by_index(m, Space::Player(bot.player)).unwrap();

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
