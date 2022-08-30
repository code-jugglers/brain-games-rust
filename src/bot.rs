use rand::Rng;
use std::collections::HashMap;

use crate::board::{Board, GameResult, Player, Space};

pub type BotMemory = HashMap<u64, Vec<i32>>;

pub struct BotConfig {
    pub player: Player,
    pub winning_move_boost: Option<i32>,
    pub win_boost: Option<i32>,
    pub loose_boost: Option<i32>,
    pub tie_boost: Option<i32>,
}

pub struct Bot {
    pub memory: BotMemory,
    pub player: Player,
    pub winning_move_boost: i32,
    pub win_boost: i32,
    pub loose_boost: i32,
    pub tie_boost: i32,
}

impl Bot {
    pub fn new(config: BotConfig) -> Self {
        Self {
            memory: HashMap::new(),
            player: config.player,
            winning_move_boost: config.winning_move_boost.unwrap_or(1000),
            win_boost: config.win_boost.unwrap_or(3),
            loose_boost: config.loose_boost.unwrap_or(-1),
            tie_boost: config.tie_boost.unwrap_or(0),
        }
    }

    pub fn load_brain(&mut self, encode_brain: Vec<u8>) {
        self.memory = bincode::deserialize(&encode_brain).unwrap();
    }

    pub fn export_brain(&self) -> Vec<u8> {
        bincode::serialize(&self.memory).unwrap()
    }

    pub fn determine_move(&mut self, board: &Board) -> Option<usize> {
        let memory = self
            .memory
            .entry(board.key_as_u64())
            .or_insert(Bot::get_default_moves(&board));

        let total = memory.iter().fold(0, |a, b| a + b);

        let mut random = if total > 1 {
            rand::thread_rng().gen_range(0..=total)
        } else {
            1
        };

        for (index, current) in memory.iter().enumerate() {
            let val = *current;

            if val > 0 && random <= val {
                return Some(index);
            }

            // account for negative numbers in arrays
            random = if val >= 0 { random - val } else { random + val }
        }

        None
    }

    pub fn learn(&mut self, board: &Board, game_result: GameResult) {
        let max_moves = board.moves.len();

        let did_win = match game_result {
            GameResult::Winner(res) => self.player == res,
            GameResult::Tie => false,
        };

        for (i, m) in board.moves.iter().enumerate() {
            if m.space == Space::Player(self.player) {
                let game_state_entry = self.memory.entry(m.key).or_insert(vec![]);

                // this should be safe. If we panic here something went wrong as the bot was deciding moves
                game_state_entry[m.index] = if did_win {
                    // give boosts for winning
                    if i == max_moves - 1 {
                        // If winning move give larger boost
                        game_state_entry[m.index] + self.winning_move_boost
                    } else {
                        // all other moves get standard boost
                        game_state_entry[m.index] + self.win_boost
                    }
                } else if game_result == GameResult::Tie {
                    // add different boost if the game is a tie
                    game_state_entry[m.index] + self.tie_boost
                } else if i == max_moves - 2 {
                    // bot lost! if the last move made lead to loss 0 it out
                    0
                } else {
                    // standard move during a loss get loose boost applied
                    game_state_entry[m.index] + self.loose_boost
                };

                // if all values are 0 remove it and let the bot start over
                // This keeps the bot for "dying"
                if game_state_entry.iter().all(|&val| val <= 0) {
                    self.memory.remove(&m.key);
                }
            }
        }
    }

    pub fn get_default_moves(board: &Board) -> Vec<i32> {
        let mut spaces: Vec<i32> = board.spaces.clone().iter().map(|_| 0).collect();

        for available_space in board.get_available_spaces() {
            spaces[available_space] = 10;
        }

        spaces
    }
}
