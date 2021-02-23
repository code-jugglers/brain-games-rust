use crate::board::{Board, BoardSpaceState};
use rand::Rng;
use std::collections::HashMap;

pub type BotMemory = HashMap<String, Vec<u32>>;

pub struct Bot {
    memory: BotMemory,
}

impl Bot {
    pub fn new() -> Bot {
        Bot {
            memory: HashMap::new(),
        }
    }

    pub fn determine_move(&mut self, board: &Board) -> Option<usize> {
        let memory = self
            .memory
            .entry(board.key())
            .or_insert(Bot::get_default_moves(&board));

        let total = memory.iter().fold(0, |a, b| a + b);

        let mut rng = rand::thread_rng();

        let mut random = if total > 1 {
            rng.gen_range(1, total)
        } else {
            1
        };

        for (index, current) in memory.iter().enumerate() {
            if *current > 0 && random <= *current {
                return Some(index);
            }

            random = random - *current;
        }

        None
    }

    pub fn learn(&mut self, board: &Board, did_win: bool) {
        for m in &board.moves {
            let key = m.key.clone();
            let game_state_entry = self.memory.entry(key).or_insert(vec![]);

            // this should be safe. If we panic here something went wrong as the bot was deciding moves
            game_state_entry[m.index] = if did_win {
                game_state_entry[m.index] + 3
            } else {
                game_state_entry[m.index] - 1
            };

            let all_0 = game_state_entry.iter().all(|&val| val <= 0);

            // if all values are 0 remove it and let the bot start over
            if all_0 {
                self.memory.remove(&m.key);
            }
        }
    }

    pub fn get_default_moves(board: &Board) -> Vec<u32> {
        let mut spaces = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

        for available_space in board.get_available_spaces() {
            spaces[available_space] = 3
        }

        spaces
    }
}
