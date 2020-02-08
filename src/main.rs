mod board;
mod board_space;
mod bot;

use board::{Board, GameResult};
use board_space::BoardSpace;
use bot::Bot;

fn main() {
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
