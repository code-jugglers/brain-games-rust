use crate::board::{Board, BoardSpaceState};
use rand::Rng;
use std::collections::HashMap;

pub type BotMemory = HashMap<String, Vec<u32>>;

pub struct Bot {
    pub memory: BotMemory,
    pub player: BoardSpaceState,
}

impl Bot {
    pub fn new(player: BoardSpaceState) -> Bot {
        Bot {
            memory: HashMap::new(),
            player,
        }
    }

    #[allow(dead_code)]
    pub fn load_brain(&mut self, encode_brain: Vec<u8>) {
        self.memory = bincode::deserialize(&encode_brain).unwrap();
    }

    pub fn export_brain(&self) -> Vec<u8> {
        bincode::serialize(&self.memory).unwrap()
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
        let max_moves = board.moves.len();

        for (i, m) in board.moves.iter().enumerate() {
            if m.space == self.player {
                let key = m.key.clone();
                let game_state_entry = self.memory.entry(key).or_insert(vec![]);

                // this should be safe. If we panic here something went wrong as the bot was deciding moves
                game_state_entry[m.index] = if did_win {
                    // If winning move give extreme boost
                    if i == max_moves - 1 {
                        game_state_entry[m.index] + 1000
                    } else {
                        game_state_entry[m.index] + 3
                    }
                } else if game_state_entry[m.index] > 0 {
                    // if the bot has lost the game 0 out the last move it made since it failed to prevent a loss
                    if i == max_moves - 2 {
                        0
                    } else {
                        game_state_entry[m.index] - 1
                    }
                } else {
                    0
                };

                let all_0 = game_state_entry.iter().all(|&val| val <= 0);

                // if all values are 0 remove it and let the bot start over
                if all_0 {
                    self.memory.remove(&m.key);
                }
            }
        }
    }

    pub fn get_default_moves(board: &Board) -> Vec<u32> {
        let mut spaces = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

        for available_space in board.get_available_spaces() {
            spaces[available_space] = 10
        }

        spaces
    }
}
