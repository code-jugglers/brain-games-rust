mod board;
mod bot;

use board::{Board, BoardSpaceState};
use bot::Bot;

fn main() {
    let mut board = Board::new();
    let player_x = Bot::new();
    let player_o = Bot::new();

    let mut current_player: BoardSpaceState = BoardSpaceState::X;
    let mut winner: Option<BoardSpaceState> = None;
    let mut moves_available = true;

    while moves_available && winner == None {
        if current_player == BoardSpaceState::X {
            let current_move = player_x.determine_move(&board.spaces);

            board.set_by_index(current_move.unwrap(), BoardSpaceState::X);

            current_player = BoardSpaceState::O;
        } else {
            let current_move = player_o.determine_move(&board.spaces);

            board.set_by_index(current_move.unwrap(), BoardSpaceState::O);

            current_player = BoardSpaceState::X;
        }

        winner = board.determine_winner();
        moves_available = board.moves_available();

        if let Some(res) = winner {
            println!("Winner: {:?}", res);
            println!("{:?}", board.spaces);
        } else if !moves_available {
            println!("IT IS A TIE!");
            println!("{:?}", board.spaces);
        }
    }
}
