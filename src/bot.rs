use crate::board::{BoardSpaceState, BoardState};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct BotMemoryEntry {
    position: u8,
    weight: u32,
}

pub type BotMemory = HashMap<String, Vec<BotMemoryEntry>>;

pub struct Bot {
    memory: BotMemory,
}

impl Bot {
    pub fn new() -> Bot {
        Bot {
            memory: HashMap::new(),
        }
    }

    pub fn determine_move(&self, board_state: &BoardState) -> Option<usize> {
        for i in 0..9 {
            if board_state[i] == BoardSpaceState::Empty {
                return Some(i);
            }
        }

        None
    }

    fn learn(&self, board: &BoardState, did_win: bool) {}
}
