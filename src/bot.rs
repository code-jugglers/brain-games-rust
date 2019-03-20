extern crate rand;

use board::{Board, BoardSpace, GameResult, HistoryEntry};
use bot::rand::Rng;
use std::collections::HashMap;
use std::mem::replace;

#[derive(Clone)]
pub struct MoveEntry {
    move_index: usize,
    weight: usize,
}

type Brain = HashMap<String, Vec<MoveEntry>>;

pub struct Bot {
    space: BoardSpace,
    brain: Brain,
}
impl Bot {
    pub fn new(space: BoardSpace) -> Bot {
        Bot {
            space,
            brain: HashMap::new(),
        }
    }

    pub fn make_move(&mut self, board: &mut Board) {
        let space: BoardSpace = self.space.clone();

        if let Some(value) = self.determine_move(board) {
            board.set_by_index(value, space);
        }
    }

    pub fn learn(&mut self, board: &Board) {
        let res = board.check_board();

        let won = (self.space == BoardSpace::X && res == GameResult::XWwin)
            || (self.space == BoardSpace::O && res == GameResult::OWin);

        let bot_moves = board
            .move_history
            .iter()
            .filter(|m| m.player == self.space)
            .collect::<Vec<&HistoryEntry>>();

        for item in bot_moves {
            let available_moves: &mut Vec<MoveEntry> =
                self.brain.entry(item.key.clone()).or_insert(
                    board
                        .get_available_spaces()
                        .iter()
                        .map(|space| MoveEntry {
                            move_index: *space,
                            weight: 3,
                        })
                        .collect(),
                );

            let current_move_index = available_moves
                .iter()
                .enumerate()
                .find_map(|(index, entry)| {
                    if entry.move_index == item.move_index {
                        return Some(index);
                    }

                    None
                })
                .unwrap();

            if won {
                let move_index = available_moves[current_move_index].move_index;
                let weight = available_moves[current_move_index].weight + 3;

                replace(
                    &mut available_moves[current_move_index],
                    MoveEntry { move_index, weight },
                );
            } else {
                let move_index = available_moves[current_move_index].move_index;
                let weight = available_moves[current_move_index].weight - 1;

                replace(
                    &mut available_moves[current_move_index],
                    MoveEntry {
                        move_index,
                        weight: if weight > 0 { weight } else { 3 },
                    },
                );
            }
        }
    }

    fn determine_move(&mut self, board: &Board) -> Option<usize> {
        let available_moves = self.brain.entry(board.key()).or_insert(
            board
                .get_available_spaces()
                .into_iter()
                .map(|move_index| MoveEntry {
                    move_index,
                    weight: 3,
                })
                .collect(),
        );

        if available_moves.len() > 0 {
            if let Some(m) = Bot::get_random_percentage(available_moves) {
                return Some(m.move_index);
            }
        }

        None
    }

    fn get_random_percentage(available_moves: &Vec<MoveEntry>) -> Option<MoveEntry> {
        let total: usize = available_moves.iter().fold(0, |acc, m| acc + m.weight);
        let mut random: usize = rand::thread_rng().gen_range(0, total);

        for current_move in available_moves {
            if current_move.weight == 0 {
                continue;
            }

            if random <= current_move.weight {
                return Some(current_move.clone());
            }

            random = random - current_move.weight;
        }

        None
    }
}
