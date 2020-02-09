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
            "train" => train(),
            "play_x" => play_x(),
            "play_o" => play_o(),
            _ => println!("command {} not found", command)
        }
    } else {
        println!("no command found");
    }
}

fn play_x() {
    let mut board = Board::new();
    let mut game_result: GameResult = GameResult::Incomplete;
    let mut bots_turn = true;

    let mut bot = Bot::new(BoardSpace::X, "brain_x.json");

    while game_result == GameResult::Incomplete {
        if bots_turn {
            let m = bot
                .determine_move(board.key(), board.spaces)
                .unwrap();

            board.make_move(bot.space, m[0], m[1]);

            bots_turn = false;
        } else {
            let m = get_move_from_user_input();

            board.make_move(BoardSpace::O, m[0], m[1]);

            bots_turn = true;
        }

        println!("{}", board);

        game_result = board.determine_winner();
    }

    println!("THE WINNER IS {:?}", game_result);
}

fn play_o() {
    let mut board = Board::new();
    let mut game_result: GameResult = GameResult::Incomplete;
    let mut bots_turn = false;

    let mut bot = Bot::new(BoardSpace::O, "brain_o.json");

    println!("{}", board);

    while game_result == GameResult::Incomplete {
        if bots_turn {
            let m = bot
                .determine_move(board.key(), board.spaces)
                .unwrap();

            board.make_move(bot.space, m[0], m[1]);

            bots_turn = false;
        } else {
            let m = get_move_from_user_input();

            board.make_move(BoardSpace::X, m[0], m[1]);

            bots_turn = true;
        }

        println!("{}", board);

        game_result = board.determine_winner();
    }

    println!("THE WINNER IS {:?}", game_result);
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


fn train() {
    const ITERATIONS: u32 = 3000000;

    let mut player_1 = Bot::new(BoardSpace::X, "brain_x.json");
    let mut player_2 = Bot::new(BoardSpace::O, "brain_o.json");
    let mut x_wins = 0;
    let mut o_wins = 0;
    let mut ties = 0;

    for i in 0..ITERATIONS {
        let mut board = Board::new();

        let winner = play(&mut board, &mut player_1, &mut player_2);

        if winner == GameResult::X {
            player_1.learn(&board.moves, true);
            player_2.learn(&board.moves, false);

            x_wins = x_wins + 1;
        } else if winner == GameResult::O {
            player_1.learn(&board.moves, false);
            player_2.learn(&board.moves, true);

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
        } else if i == ITERATIONS - 1 {
            println!("=========== FINAL =========");
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
            let m = current_player
                .determine_move(board.key(), board.spaces)
                .unwrap();

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
