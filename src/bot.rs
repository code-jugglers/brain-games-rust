use board::{Board, BoardSpace};
use player::Player;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Bot {
    pub player: Player,
    pub brain: HashMap<String, usize>,
}
impl Bot {
    pub fn new(player: Player, brain: HashMap<String, usize>) -> Bot {
        Bot { player, brain }
    }

    pub fn make_move(&self, board: &mut Board) {
        let index = Bot::determine_move(board);

        if let Some(_) = index {
            board.set_by_index(&self.player.player_space, index.unwrap());
        }
    }

    pub fn learn(&mut self, board: &Board) {
        let key = board.key();

        let entry = self.brain.entry(key.clone()).or_insert(0);

        *entry += 3;
    }

    fn determine_move(board: &Board) -> Option<usize> {
        let mut index: Option<usize> = None;

        for (i, space) in board.spaces.iter().enumerate() {
            if space.clone() == BoardSpace::Blank {
                index = Some(i);
                break;
            }
        }

        index
    }
}
