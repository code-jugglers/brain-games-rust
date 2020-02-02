use crate::board::{BoardSpaces, Move, MoveEntry};
use crate::board_space::BoardSpace;
use rand::seq::SliceRandom;
use std::collections::HashMap;

pub type BotMemory = HashMap<String, HashMap<Move, u32>>;

#[derive(Debug)]
pub struct Bot {
    pub id: String,
    pub space: BoardSpace,
    pub memory: BotMemory,
}
impl Bot {
    pub fn new(id: String, space: BoardSpace) -> Bot {
        Bot {
            id,
            space,
            memory: HashMap::new(),
        }
    }

    pub fn determine_move(&self, board_spaces: BoardSpaces) -> Option<Move> {
        let available_moves = self.get_available_moves(board_spaces);

        match available_moves.choose(&mut rand::thread_rng()) {
            Some(m) => Some(m.clone()),
            None => None,
        }
    }

    pub fn get_available_moves(&self, board_spaces: BoardSpaces) -> Vec<Move> {
        let mut available_moves = vec![];

        for (col_index, col) in board_spaces.iter().enumerate() {
            for (cell_index, cell) in col.iter().enumerate() {
                if cell == &BoardSpace::Empty {
                    available_moves.push([col_index, cell_index]);
                }
            }
        }

        available_moves
    }

    pub fn learn(&mut self, moves: &Vec<MoveEntry>, did_win: bool) {
        for m in moves {
            let move_entry = self.memory.entry(m.key.clone()).or_insert(HashMap::new());
            let current_move = move_entry.entry(m.position).or_insert(0);

            *current_move = if did_win {
                *current_move + 6
            } else if *current_move > 0 {
                *current_move - 1
            } else {
                0
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
        let bot = Bot::new(String::from("0"), BoardSpace::X);

        let move_found: bool = match bot.determine_move(board.spaces) {
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

        let mut bot = Bot::new(String::from("0"), BoardSpace::X);

        bot.learn(&board.moves, true);

        println!("{:?}", bot.memory);
    }
}
