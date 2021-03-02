mod board;
mod bot;
mod play;

use board::{Board, BoardSpaceState, GameResult, Player};
use bot::Bot;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
            player_x: Bot::new(BoardSpaceState::Player(Player::X)),
            player_o: Bot::new(BoardSpaceState::Player(Player::O)),
        }
    }

    pub fn board(&self) -> String {
        self.board.key()
    }

    pub fn reset_board(&mut self) {
        self.board = Board::new();
    }

    pub fn make_move_x(&mut self, index: usize) -> Option<String> {
        self.board
            .set_by_index(index, BoardSpaceState::Player(Player::X));

        if let Some(res) = self.board.determine_winner() {
            return Some(res.to_string());
        }

        let bot_move = self.player_o.determine_move(&self.board);

        self.board
            .set_by_index(bot_move.unwrap(), BoardSpaceState::Player(Player::O));

        if let Some(res) = self.board.determine_winner() {
            return Some(res.to_string());
        }

        None
    }

    pub fn make_bot_move_x(&mut self) {
        let bot_move = self.player_x.determine_move(&self.board);

        self.board
            .set_by_index(bot_move.unwrap(), BoardSpaceState::Player(Player::X));
    }

    pub fn make_move_o(&mut self, index: usize) -> Option<String> {
        self.board
            .set_by_index(index, BoardSpaceState::Player(Player::O));

        if let Some(res) = self.board.determine_winner() {
            return Some(res.to_string());
        }

        let bot_move = self.player_x.determine_move(&self.board);

        self.board
            .set_by_index(bot_move.unwrap(), BoardSpaceState::Player(Player::X));

        if let Some(res) = self.board.determine_winner() {
            return Some(res.to_string());
        }

        None
    }

    pub fn train(&mut self, game_count: u32) -> String {
        let mut x_win = 0;
        let mut o_win = 0;
        let mut tie = 0;

        let mut dummy_o = Bot::new(BoardSpaceState::Player(Player::O));

        for _ in 1..=game_count / 2 {
            let result = play::play(&mut self.board, &mut self.player_x, &mut dummy_o);

            if let Some(res) = result {
                if res == GameResult::XWin {
                    x_win += 1;
                } else if res == GameResult::OWin {
                    o_win += 1;
                } else {
                    tie += 1;
                }
            }

            self.reset_board();
        }
        
        let mut dummy_x = Bot::new(BoardSpaceState::Player(Player::X));

        for _ in 1..=game_count / 2 {
            let result = play::play(&mut self.board, &mut dummy_x, &mut self.player_o);

            if let Some(res) = result {
                if res == GameResult::XWin {
                    x_win += 1;
                } else if res == GameResult::OWin {
                    o_win += 1;
                } else {
                    tie += 1;
                }
            }

            self.reset_board();
        }

        "X: ".to_string()
            + &x_win.to_string()
            + "\nO: "
            + &o_win.to_string()
            + "\nTIE: "
            + &tie.to_string()
    }
}
