mod board;
mod board_space;
mod bot;

use board::Board;
use board_space::BoardSpace;
use bot::Bot;

fn main() {
    let mut player_1 = Bot::new(String::from("0"), BoardSpace::X);
    let mut player_2 = Bot::new(String::from("1"), BoardSpace::O);

    for _ in 0..1000 {
        let mut board = Board::new();

        let winner = play(&mut board, &player_1, &player_2);

        if winner == BoardSpace::X {
            player_1.learn(&board.moves, true);
            player_2.learn(&board.moves, false);
        } else if winner == BoardSpace::O {
            player_1.learn(&board.moves, false);
            player_2.learn(&board.moves, true);
        }

        println!("{}", board);
        println!("The Winnder Is: {} \n", board.determine_winner());
    }

    fn play(board: &mut Board, player_1: &Bot, player_2: &Bot) -> BoardSpace {
        let mut current_player: &Bot = player_1;
        let mut game_result: BoardSpace = BoardSpace::Empty;

        while game_result == BoardSpace::Empty {
            game_result = board.determine_winner();

            if let Some(m) = current_player.determine_move(board.spaces) {
                if board.make_move(current_player.space, m[0], m[1]) {
                    current_player = if current_player.id == player_1.id {
                        player_2
                    } else {
                        player_1
                    }
                }
            }
        }

        game_result
    }
}
