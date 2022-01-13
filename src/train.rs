use crate::board::{Board, GameResult, Player, Space};
use crate::bot::Bot;

pub fn play(
    board: &mut Board,
    player_x: &mut Bot,
    player_o: &mut Bot,
) -> Result<GameResult, Player> {
    let mut current_player: Player = Player::X;

    loop {
        if current_player == Player::X {
            if let Some(current_move) = player_x.determine_move(board) {
                board
                    .set_by_index(current_move, Space::Player(current_player))
                    .unwrap();

                current_player = Player::O;
            } else {
                return Err(Player::X);
            }
        } else {
            if let Some(current_move) = player_o.determine_move(board) {
                board
                    .set_by_index(current_move, Space::Player(current_player))
                    .unwrap();

                current_player = Player::X;
            } else {
                return Err(Player::O);
            }
        }

        if let Some(res) = board.determine_winner() {
            player_x.learn(&board, res);
            player_o.learn(&board, res);

            return Ok(res);
        }
    }
}
