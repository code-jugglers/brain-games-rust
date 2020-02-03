mod board;
mod board_space;
mod bot;

use board::Board;
use board_space::BoardSpace;
use bot::Bot;

fn main() {
    let mut player_1 = Bot::new(BoardSpace::X);
    let mut player_2 = Bot::new(BoardSpace::O);
    let mut x_wins = 0;
    let mut y_wins = 0;
    let mut ties = 0;

    for _ in 0..1000000 {
        let mut board = Board::new();

        let winner = play(&mut board, &mut player_1, &mut player_2);

        if winner == BoardSpace::X {
            player_1.learn(&board.moves, true);
            player_2.learn(&board.moves, false);

            x_wins = x_wins + 1;
        } else if winner == BoardSpace::O {
            player_1.learn(&board.moves, false);
            player_2.learn(&board.moves, true);

            y_wins = y_wins + 1;
        } else {
            ties = ties + 1;
        }

        println!("==========");
        println!("X WINS: {}", x_wins);
        println!("O WINS: {}", y_wins);
        println!("Ties  : {}", ties);
    }

    fn play(board: &mut Board, player_1: &mut Bot, player_2: &mut Bot) -> BoardSpace {
        let mut current_player: &mut Bot = player_1;
        let mut current_player_id: String = String::from("0");
        let mut continue_game: bool = true;
        let mut game_result: BoardSpace = BoardSpace::Empty;

        while continue_game {
            game_result = board.determine_winner();

            if game_result != BoardSpace::Empty {
                continue_game = false;
            } else if let Some(m) = current_player.determine_move(board.key(), board.spaces) {
                if board.make_move(current_player.space, m[0], m[1]) {
                    if current_player_id == String::from("0") {
                        current_player = player_2;
                        current_player_id = String::from("1");
                    } else {
                        current_player = player_1;
                        current_player_id = String::from("0");
                    }
                }
            } else {
                continue_game = false;
            }
        }

        game_result
    }
}
