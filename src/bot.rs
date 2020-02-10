use crate::board::{Board, Move};
use crate::board_space::BoardSpace;

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BotMemoryEntry {
    position: Move,
    weight: u32,
}

pub type BotMemory = HashMap<String, Vec<BotMemoryEntry>>;

#[derive(Debug)]
pub struct Bot {
    pub space: BoardSpace,
    pub memory: BotMemory,
    pub file_path: &'static str,
}
impl Bot {
    pub fn new(space: BoardSpace, file_path: &'static str) -> Bot {
        let memory = if Path::new(file_path).exists() {
            let data: BotMemory = serde_json::from_reader(File::open(file_path).unwrap()).unwrap();

            data
        } else {
            HashMap::new()
        };

        Bot {
            space,
            memory,
            file_path,
        }
    }

    pub fn determine_move(&mut self, board: &Board) -> Option<Move> {
        let memory = self
            .memory
            .entry(board.key())
            .or_insert(Bot::get_default_moves(&board));

        let total = memory.iter().fold(0, |a, b| a + b.weight);
        let mut rng = rand::thread_rng();
        let mut random = if total > 1 {
            rng.gen_range(1, total)
        } else {
            1
        };

        for current in memory {
            if current.weight > 0 && random <= current.weight {
                return Some(current.position);
            }

            random = random - current.weight;
        }

        None
    }

    pub fn get_default_moves(board: &Board) -> Vec<BotMemoryEntry> {
        board
            .get_available_spaces()
            .into_iter()
            .map(|position| BotMemoryEntry {
                position,
                weight: 3,
            })
            .collect()
    }

    pub fn learn(&mut self, board: &Board, did_win: bool) {
        for (i, m) in board.moves.iter().enumerate() {
            if m.space == self.space {
                let game_state_entry = self.memory.entry(m.key.clone()).or_insert(Vec::new());

                // Safe to unwrap.
                // If an error is thrown here there is something wrong with the move selection
                let current_move = game_state_entry
                    .iter_mut()
                    .find(|entry| {
                        entry.position[0] == m.position[0] && entry.position[1] == m.position[1]
                    })
                    .unwrap();

                current_move.weight = if did_win {
                    if i == board.moves.len() - 1 {
                        // if this is the last move aka the winning move jack the weight way up
                        current_move.weight + 100
                    } else {
                        current_move.weight + 3
                    }
                } else {
                    current_move.weight - 1
                };

                // If every entry is 0 reset them all
                if game_state_entry
                    .iter()
                    .fold(true, |val, entry| entry.weight == 0 && val)
                {
                    for entry in game_state_entry {
                        entry.weight = 3;
                    }
                }
            }
        }
    }

    pub fn save_brain_to_file(&self) {
        serde_json::to_writer(File::create(self.file_path).unwrap(), &self.memory).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn should_return_a_valid_move() {
        let board = Board::new();
        let mut bot = Bot::new(BoardSpace::X, "bot_x.json");

        let move_found: bool = match bot.determine_move(&board) {
            Some(_) => true,
            None => false,
        };

        assert_eq!(move_found, true);
    }

    #[test]
    fn should_increase_weight_for_moves_used_during_win() {
        let mut board = Board::new();
        board.make_move(BoardSpace::X, 0, 0);
        board.make_move(BoardSpace::X, 1, 1);
        board.make_move(BoardSpace::X, 2, 2);

        let mut bot = Bot::new(BoardSpace::X, "bot_x.json");

        bot.learn(&board, true);

        println!("{:?}", bot.memory);
    }
}
