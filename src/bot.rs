use crate::board::{Board, BoardSpaceState, GameResult};
use rand::Rng;
use std::collections::HashMap;

pub type BotMemory = HashMap<u32, Vec<i32>>;

pub struct BotConfig {
    pub player: BoardSpaceState,
    pub winning_move_boost: Option<i32>,
    pub win_boost: Option<i32>,
    pub loose_boost: Option<i32>,
    pub tie_boost: Option<i32>,
}

pub struct Bot {
    pub memory: BotMemory,
    pub player: BoardSpaceState,
    pub winning_move_boost: i32,
    pub win_boost: i32,
    pub loose_boost: i32,
    pub tie_boost: i32,
}

impl Bot {
    pub fn new(config: BotConfig) -> Bot {
        Bot {
            memory: HashMap::new(),
            player: config.player,
            winning_move_boost: config.winning_move_boost.unwrap_or(1000),
            win_boost: config.win_boost.unwrap_or(3),
            loose_boost: config.loose_boost.unwrap_or(-1),
            tie_boost: config.tie_boost.unwrap_or(0),
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
            .entry(board.key_as_u32())
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

    pub fn learn(&mut self, board: &Board, game_result: GameResult) {
        let max_moves = board.moves.len();

        let did_win = match game_result {
            GameResult::Winner(res) => self.player == BoardSpaceState::Player(res),
            GameResult::Tie => false,
        };

        for (i, m) in board.moves.iter().enumerate() {
            if m.space == self.player {
                let key = m.key.clone();
                let game_state_entry = self.memory.entry(key).or_insert(vec![]);

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
                } else if game_state_entry[m.index] > 0 {
                    // if the bot has lost the game 0 out the last move it made since it failed to prevent a loss
                    if i == max_moves - 2 {
                        0
                    } else {
                        game_state_entry[m.index] + self.loose_boost
                    }
                } else {
                    0
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
        let mut spaces = vec![0, 0, 0, 0, 0, 0, 0, 0, 0];

        for available_space in board.get_available_spaces() {
            spaces[available_space] = 10
        }

        spaces
    }
}
