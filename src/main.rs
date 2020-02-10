mod board;
mod board_space;
mod bot;

use board::{Board, GameResult, Move};
use board_space::BoardSpace;
use bot::Bot;
use std::env;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let command = args[1].as_str();

        match command {
            "train" => train(3000000, "brain_x.json", "brain_o.json"),
            "play_x" => play_bot(BoardSpace::X, "brain_x.json"),
            "play_o" => play_bot(BoardSpace::O, "brain_o.json"),
            _ => println!("command {} not found", command),
        }
    } else {
        println!("no command found");
    }
}

fn play_bot(bot_space: BoardSpace, brain_path: &'static str) {
    let mut board = Board::new();
    let mut game_result: GameResult = GameResult::Incomplete;
    let mut bots_turn = bot_space == BoardSpace::X;

    let player_space = if bot_space == BoardSpace::X {
        BoardSpace::O
    } else {
        BoardSpace::X
    };

    let mut bot = Bot::new(bot_space, brain_path);

    println!("Game Start");
    println!("{}", board);

    while game_result == GameResult::Incomplete {
        if bots_turn {
            let m = bot.determine_move(&board).unwrap();

            board.make_move(bot.space, m[0], m[1]);

            println!("Bot makes move at {} {}", m[0], m[1]);

            bots_turn = false;
        } else {
            let m = get_move_from_user_input();

            board.make_move(player_space, m[0], m[1]);

            bots_turn = true;
        }

        println!("{}", board);

        game_result = board.determine_winner();
    }

    if bot_space == BoardSpace::X {
        bot.learn(&board, game_result == GameResult::X);
    } else {
        bot.learn(&board, game_result == GameResult::O);
    }

    bot.save_brain_to_file();

    match game_result {
        GameResult::X => println!("X WINS!"),
        GameResult::O => println!("O WINS!"),
        GameResult::Tie => println!("IT IS A TIE!"),
        _ => {}
    }
}

fn get_move_from_user_input() -> Move {
    println!("Enter your move:");

    let mut m: String = String::new();
    io::stdin().read_line(&mut m).expect("No move entered");

    let mut parsed_move = m.split_whitespace().map(|m| m.parse::<usize>().unwrap());
    let row = parsed_move.next().unwrap();
    let col = parsed_move.next().unwrap();

    [col, row]
}

fn train(iterations: u32, brain_1: &'static str, brain_2: &'static str) {
    let mut player_1 = Bot::new(BoardSpace::X, brain_1);
    let mut player_2 = Bot::new(BoardSpace::O, brain_2);
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut ties = 0;

    for i in 0..iterations {
        let mut board = Board::new();

        let winner = play(&mut board, &mut player_1, &mut player_2);

        if winner == GameResult::X {
            player_1.learn(&board, true);
            player_2.learn(&board, false);

            x_wins = x_wins + 1;
        } else if winner == GameResult::O {
            player_1.learn(&board, false);
            player_2.learn(&board, true);

            o_wins = o_wins + 1;
        } else {
            ties = ties + 1;
        }

        if i % 100000 == 0 {
            println!("=========== GAME {} =========", i + 1);
            println!("{}", board);
            println!("X WINS: {}", x_wins);
            println!("O WINS: {}", o_wins);
            println!("Ties  : {}", ties);
        } else if i == iterations - 1 {
            println!("=========== ROUND COMPLETE =========");
            println!("{}", board);
            println!("X WINS: {}", x_wins);
            println!("O WINS: {}", o_wins);
            println!("Ties  : {}", ties);
        }
    }

    player_1.save_brain_to_file();
    player_2.save_brain_to_file();

    fn play(board: &mut Board, player_1: &mut Bot, player_2: &mut Bot) -> GameResult {
        let mut current_player: &mut Bot = player_1;
        let mut current_player_id: &str = "0";
        let mut game_result: GameResult = GameResult::Incomplete;

        while game_result == GameResult::Incomplete {
            let m = current_player.determine_move(&board).unwrap();

            board.make_move(current_player.space, m[0], m[1]);

            if current_player_id == "0" {
                current_player = player_2;
                current_player_id = "1";
            } else {
                current_player = player_1;
                current_player_id = "0";
            }

            game_result = board.determine_winner();
        }

        game_result
    }
}
