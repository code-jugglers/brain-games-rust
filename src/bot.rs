extern crate rand;

use board::{Board, BoardSpace};
use bot::rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Move {
    index: usize,
    weight: usize,
}

pub type Brain = HashMap<String, HashMap<usize, usize>>;

#[derive(Debug, Clone)]
pub struct Bot {
    pub space: BoardSpace,
    pub brain: Brain,
    pub current_game_history: HashMap<String, usize>,
}
impl Bot {
    pub fn new(space: BoardSpace, brain: Brain) -> Bot {
        Bot {
            space,
            brain,
            current_game_history: HashMap::new(),
        }
    }

    pub fn make_move(&mut self, board: &mut Board) {
        let value: Option<usize> = self.determine_move(board);

        match value {
            Some(move_index) => {
                board.set_by_index(&self.space, move_index);

                self.current_game_history.insert(board.key(), move_index);
            }
            None => println!("Bot could not find an acceptable move"),
        }
    }

    pub fn learn(&mut self, won: bool) {
        for (board_key, index) in &self.current_game_history {
            let entries = self
                .brain
                .entry(board_key.clone())
                .or_insert(HashMap::new());

            let entry = entries.entry(index.clone()).or_insert(0);

            if won {
                *entry += 3;
            } else {
                if *entry > 3 {
                    *entry -= 1;
                }
            }
        }
    }

    fn determine_move(&mut self, board: &Board) -> Option<usize> {
        let current_available_moves = self.get_available_moves(board);

        let available_moves = self
            .brain
            .entry(board.key())
            .or_insert(current_available_moves);

        Bot::pick_random_percentage(&available_moves)
    }

    fn get_available_moves(&self, board: &Board) -> HashMap<usize, usize> {
        let mut available_spaces = HashMap::new();

        for (index, space) in board.spaces.iter().enumerate() {
            if *space == BoardSpace::Blank {
                available_spaces.insert(index, 3);
            }
        }

        available_spaces
    }

    fn pick_random_percentage(moves: &HashMap<usize, usize>) -> Option<usize> {
        let total = {
            let mut value = 0;

            for (_, weight) in moves {
                value = value + weight;
            }

            value
        };

        let mut random = rand::thread_rng().gen_range(0, total);

        for (current_move, weight) in moves {
            if weight > &0 {
                if random <= *weight {
                    return Some(*current_move);
                }

                random = random - weight;
            }
        }

        None
    }
}
