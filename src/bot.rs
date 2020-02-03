use crate::board::{BoardSpaces, Move, MoveEntry};
use crate::board_space::BoardSpace;
use rand::seq::SliceRandom;
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
        let memory = self.memory.entry(board_key).or_insert(Vec::new());

        if memory.len() > 0 {
            let total = memory.iter().fold(0, |a, b| a + b.weight);
            let mut rng = rand::thread_rng();
            let mut random = rng.gen_range(0, total);

            for i in 0..total {
                let current = memory[i as usize];

                if current.weight == 0 {
                    continue;
                }

                if random <= current.weight {
                    return Some(current.position);
                }

                random = random - current.weight;
            }

            None
        } else {
            let available_moves = Bot::get_available_moves(board_spaces);

            for am in &available_moves {
                memory.push(am.clone());
            }

            match &available_moves.choose(&mut rand::thread_rng()) {
                Some(m) => Some(m.position.clone()),
                None => None,
            }
        }
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
            let game_state_entry = self.memory.entry(m.key.clone()).or_insert(Vec::new());

            let current_move = game_state_entry.iter_mut().find(|memory| {
                memory.position[0] == m.position[0] && memory.position[1] == m.position[1]
            });

            if let Some(m) = current_move {
                m.weight = if did_win {
                    m.weight + 6
                } else if m.weight > 0 {
                    m.weight - 1
                } else {
                    0
                }
            } else {
                game_state_entry.push(BotMemoryEntry {
                    position: m.position,
                    weight: 3,
                })
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
