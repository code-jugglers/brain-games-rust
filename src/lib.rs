mod board;
mod bot;
mod train;

use wasm_bindgen::prelude::*;

use crate::board::{Board, GameResult, Player, Space};
use crate::bot::{Bot, BotConfig};

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
    #[wasm_bindgen(constructor)]
    pub fn new(
        winning_move_boost: Option<i32>,
        win_boost: Option<i32>,
        loose_boost: Option<i32>,
        tie_boost: Option<i32>,
    ) -> Self {
        Self {
            board: Board::new(3, 3),
            player_x: Bot::new(BotConfig {
                player: Player::X,
                winning_move_boost: winning_move_boost,
                win_boost: win_boost,
                loose_boost: loose_boost,
                tie_boost: tie_boost,
            }),
            player_o: Bot::new(BotConfig {
                player: Player::O,
                winning_move_boost: winning_move_boost,
                win_boost: win_boost,
                loose_boost: loose_boost,
                tie_boost: tie_boost,
            }),
        }
    }

    pub fn load_x_brain(&mut self, brain: Vec<u8>) {
        self.player_x.load_brain(brain);
    }

    pub fn load_o_brain(&mut self, brain: Vec<u8>) {
        self.player_o.load_brain(brain);
    }

    pub fn export_x_brain(&mut self) -> Vec<u8> {
        self.player_x.export_brain()
    }

    pub fn export_o_brain(&mut self) -> Vec<u8> {
        self.player_o.export_brain()
    }

    pub fn board(&self) -> String {
        self.board.key_as_string()
    }

    pub fn reset_board(&mut self) {
        self.board = Board::new(3, 3);
    }

    pub fn make_move_x(&mut self, index: usize) -> Option<String> {
        self.board
            .set_by_index(index, Space::Player(Player::X))
            .unwrap();

        if let Some(res) = self.board.determine_winner() {
            return Some(res.to_string());
        }

        let bot_move = self.player_o.determine_move(&self.board);

        self.board
            .set_by_index(bot_move.unwrap(), Space::Player(Player::O))
            .unwrap();

        if let Some(res) = self.board.determine_winner() {
            return Some(res.to_string());
        }

        None
    }

    pub fn make_bot_move_x(&mut self) {
        let bot_move = self.player_x.determine_move(&self.board);

        self.board
            .set_by_index(bot_move.unwrap(), Space::Player(Player::X))
            .unwrap();
    }

    pub fn make_move_o(&mut self, index: usize) -> Option<String> {
        self.board
            .set_by_index(index, Space::Player(Player::O))
            .unwrap();

        if let Some(res) = self.board.determine_winner() {
            return Some(res.to_string());
        }

        let bot_move = self.player_x.determine_move(&self.board);

        self.board
            .set_by_index(bot_move.unwrap(), Space::Player(Player::X))
            .unwrap();

        if let Some(res) = self.board.determine_winner() {
            return Some(res.to_string());
        }

        None
    }

    pub fn train(&mut self, game_count: u32) -> String {
        let mut x_win = 0;
        let mut o_win = 0;
        let mut tie = 0;

        for _ in 1..=game_count {
            match train::play(&mut self.board, &mut self.player_x, &mut self.player_o) {
                Ok(GameResult::Winner(Player::X)) => x_win += 1,
                Ok(GameResult::Winner(Player::O)) => o_win += 1,
                Ok(GameResult::Tie) => tie += 1,
                Err(player) => {
                    println!(
                        "{:?} was unable to find an available move on board:\n {}",
                        player, self.board
                    );
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
