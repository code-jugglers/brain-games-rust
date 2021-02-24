use crate::board::{Board, BoardSpaceState, Player};
use crate::bot::Bot;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum GameResult {
    XWin,
    OWin,
    Tie,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameResult::XWin => write!(f, "X"),
            GameResult::OWin => write!(f, "O"),
            GameResult::Tie => write!(f, "TIE"),
        }
    }
}

pub fn play(board: &mut Board, player_x: &mut Bot, player_o: &mut Bot) -> Option<GameResult> {
    let mut current_player: BoardSpaceState = BoardSpaceState::Player(Player::X);
    let mut winner: Option<BoardSpaceState> = None;
    let mut moves_available = true;

    while moves_available && winner == None {
        if current_player == BoardSpaceState::Player(Player::X) {
            let current_move = player_x.determine_move(board);
            let space_state = BoardSpaceState::Player(Player::X);

            board.set_by_index(current_move.unwrap(), space_state);

            current_player = BoardSpaceState::Player(Player::O);
        } else {
            let current_move = player_o.determine_move(board);
            let space_state = BoardSpaceState::Player(Player::O);

            board.set_by_index(current_move.unwrap(), space_state);

            current_player = BoardSpaceState::Player(Player::X);
        }

        winner = board.determine_winner();
        moves_available = board.moves_available();

        if let Some(res) = winner {
            match res {
                BoardSpaceState::Player(Player::X) => {
                    player_x.learn(board, true);
                    player_o.learn(board, false);

                    return Some(GameResult::XWin);
                }
                BoardSpaceState::Player(Player::O) => {
                    player_x.learn(&board, false);
                    player_o.learn(&board, true);

                    return Some(GameResult::OWin);
                }
                BoardSpaceState::Empty => {}
            }
        } else if !moves_available {
            return Some(GameResult::Tie);
        }
    }

    None
}
