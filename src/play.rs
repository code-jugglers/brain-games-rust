use crate::board::{Board, BoardSpaceState, GameResult, Player};
use crate::bot::Bot;

pub fn play(board: &mut Board, player_x: &mut Bot, player_o: &mut Bot) -> Option<GameResult> {
    let mut current_player: Player = Player::X;
    let mut winner: Option<GameResult> = None;
    let mut moves_available = true;

    while moves_available && winner == None {
        if current_player == Player::X {
            let current_move = player_x.determine_move(board);
            let space_state = BoardSpaceState::Player(Player::X);

            board.set_by_index(current_move.unwrap(), space_state);

            current_player = Player::O;
        } else {
            let current_move = player_o.determine_move(board);
            let space_state = BoardSpaceState::Player(Player::O);

            board.set_by_index(current_move.unwrap(), space_state);

            current_player = Player::X;
        }

        winner = board.determine_winner();
        moves_available = board.moves_available();

        if let Some(res) = winner {
            player_x.learn(&board, res);
            player_o.learn(&board, res);
        }
    }

    winner
}
