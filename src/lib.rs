mod board;
mod bot;
mod play;

use board::{Board, BoardSpaceState, Player};
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

    pub fn make_move_x(&mut self, index: usize) {
        self.board
            .set_by_index(index, BoardSpaceState::Player(Player::X));

        let bot_move = self.player_o.determine_move(&self.board);

        if let Some(m) = bot_move {
            self.board
                .set_by_index(m, BoardSpaceState::Player(Player::O));
        }
    }

    pub fn make_move_o(&mut self, index: usize) {
        self.board
            .set_by_index(index, BoardSpaceState::Player(Player::O));

        let bot_move = self.player_x.determine_move(&self.board);

        if let Some(m) = bot_move {
            self.board
                .set_by_index(m, BoardSpaceState::Player(Player::X));
        }
    }

    pub fn train(&mut self, game_count: u32) -> String {
        let mut x_win = 0;
        let mut o_win = 0;
        let mut tie = 0;

        for _ in 1..=game_count {
            let res = self.play();

            if let Some(res) = res {
                if res == "X" {
                    x_win += 1;
                } else if res == "O" {
                    o_win += 1;
                } else {
                    tie += 1;
                }
            }
        }

        self.reset_board();

        "X: ".to_string()
            + &x_win.to_string()
            + "\nO: "
            + &o_win.to_string()
            + "\nTIE: "
            + &tie.to_string()
    }

    pub fn play(&mut self) -> Option<String> {
        self.reset_board();

        let result = play::play(&mut self.board, &mut self.player_x, &mut self.player_o);

        if let Some(res) = result {
            return Some(res.to_string());
        }

        None
    }
}
