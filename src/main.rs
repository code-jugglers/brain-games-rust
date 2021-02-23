mod board;
mod bot;

use board::{Board, BoardSpaceState, Player};
use bot::Bot;

fn main() {
    let mut board = Board::new();
    let mut player_x = Bot::new();
    let mut player_o = Bot::new();

    let mut current_player: BoardSpaceState = BoardSpaceState::Player(Player::X);
    let mut winner: Option<BoardSpaceState> = None;
    let mut moves_available = true;

    while moves_available && winner == None {
        if current_player == BoardSpaceState::Player(Player::X) {
            let current_move = player_x.determine_move(&board);

            board.set_by_index(current_move.unwrap(), BoardSpaceState::Player(Player::X));

            current_player = BoardSpaceState::Player(Player::O);
        } else {
            let current_move = player_o.determine_move(&board);

            board.set_by_index(current_move.unwrap(), BoardSpaceState::Player(Player::O));

            current_player = BoardSpaceState::Player(Player::X);
        }

        winner = board.determine_winner();
        moves_available = board.moves_available();

        if let Some(res) = winner {
            println!("Winner: {:?}", res);
            println!("{:?}", board.spaces);

            match res {
                BoardSpaceState::Player(Player::X) => {
                    player_x.learn(&board, true);
                    player_o.learn(&board, false);
                }
                BoardSpaceState::Player(Player::O) => {
                    player_x.learn(&board, false);
                    player_o.learn(&board, true);
                }
                BoardSpaceState::Empty => {}
            }
        } else if !moves_available {
            println!("IT IS A TIE!");
            println!("{:?}", board.spaces);
        }
    }
}
