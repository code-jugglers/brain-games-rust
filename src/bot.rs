use crate::board::{BoardSpaces, Move, MoveEntry};
use crate::board_space::BoardSpace;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct BotMemoryEntry {
    position: Move,
    weight: u32,
}

pub type BotMemory = HashMap<String, Vec<BotMemoryEntry>>;

#[derive(Debug)]
pub struct Bot {
    pub space: BoardSpace,
    pub memory: BotMemory,
}
impl Bot {
    pub fn new(space: BoardSpace) -> Bot {
        Bot {
            space,
            memory: HashMap::new(),
        }
    }

    pub fn determine_move(&mut self, board_key: String, board_spaces: BoardSpaces) -> Option<Move> {
        let memory = self
            .memory
            .entry(board_key)
            .or_insert(Bot::get_available_moves(board_spaces));

        let total = memory.iter().fold(0, |a, b| a + b.weight);
        let mut rng = rand::thread_rng();
        let mut random = rng.gen_range(1, total);

        for current in memory {
            if random <= current.weight {
                return Some(current.position);
            } else {
                random = random - current.weight;
            }
        }

        None
    }

    pub fn get_available_moves(board_spaces: BoardSpaces) -> Vec<BotMemoryEntry> {
        let mut available_moves = Vec::new();

        for (col_index, col) in board_spaces.iter().enumerate() {
            for (cell_index, cell) in col.iter().enumerate() {
                if cell == &BoardSpace::Empty {
                    available_moves.push(BotMemoryEntry {
                        position: [col_index, cell_index],
                        weight: 3,
                    });
                }
            }
        }

        available_moves
    }

    pub fn learn(&mut self, moves: &Vec<MoveEntry>, did_win: bool) {
        for m in moves {
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
                    current_move.weight + 3
                } else {
                    current_move.weight - 1
                };

                if current_move.weight > 0 {
                    current_move.weight = 3;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;

    #[test]
    fn should_return_a_valid_move() {
        let board = Board::new();
        let mut bot = Bot::new(BoardSpace::X);

        let move_found: bool = match bot.determine_move(String::from(""), board.spaces) {
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

        let mut bot = Bot::new(BoardSpace::X);

        bot.learn(&board.moves, true);

        println!("{:?}", bot.memory);
    }
}
