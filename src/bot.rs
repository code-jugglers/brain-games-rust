use crate::board::Board;
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

        for current in memory {
            let val = current.clone();

            if val > 0 && random <= val {
                return Some(val as usize);
            }

            random -= val;
        }

        None
    }

    pub fn learn(&mut self, board: &Board, did_win: bool) {
        for m in &board.moves {
            let key = m.key.clone();
            let game_state_entry = self.memory.entry(key).or_insert(vec![]);
            let current_move = game_state_entry[m.index];

            game_state_entry[m.index] = if did_win {
                current_move + 3
            } else {
                current_move - 1
            };
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
