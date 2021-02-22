mod board;
mod bot;

use board::{Board, BoardSpaceState};
use bot::Bot;
use std::fmt;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

impl fmt::Display for BoardSpaceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardSpaceState::X => write!(f, "X"),
            BoardSpaceState::O => write!(f, "O"),
            BoardSpaceState::Empty => write!(f, "-"),
        }
    }
}

#[wasm_bindgen]
pub struct Game {
    board: Board,
    player_x: Bot,
    player_o: Bot,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            player_x: Bot::new(),
            player_o: Bot::new(),
        }
    }

    pub fn board(&self) -> String {
        let mut val = String::new();

        for (i, space) in self.board.spaces.iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                val += &String::from("\n")
            }

            val += &space.to_string();
            val += &String::from(" ");
        }

        val
    }

    pub fn train() {}

    pub fn play(&mut self) -> Option<String> {
        let mut current_player: BoardSpaceState = BoardSpaceState::X;
        let mut winner: Option<BoardSpaceState> = None;
        let mut moves_available = true;

        while moves_available && winner == None {
            if current_player == BoardSpaceState::X {
                let current_move = self.player_x.determine_move(&self.board.spaces);

                self.board
                    .set_by_index(current_move.unwrap(), BoardSpaceState::X);

                current_player = BoardSpaceState::O;
            } else {
                let current_move = self.player_o.determine_move(&self.board.spaces);

                self.board
                    .set_by_index(current_move.unwrap(), BoardSpaceState::O);

                current_player = BoardSpaceState::X;
            }

            winner = self.board.determine_winner();
            moves_available = self.board.moves_available();

            if let Some(res) = winner {
                match res {
                    BoardSpaceState::O => return Some(String::from("O Wins!")),
                    BoardSpaceState::X => return Some(String::from("X Wins!")),
                    BoardSpaceState::Empty => {}
                }
            } else if !moves_available {
                return Some(String::from("TIE!"));
            }
        }

        None
    }
}
