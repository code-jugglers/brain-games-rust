use board::{Board, BoardSpace};
use player::Player;

#[derive(Debug, Clone)]
pub struct Bot {
    pub player: Player,
}
impl Bot {
    pub fn new(player: Player) -> Bot {
        Bot { player }
    }

    fn determin_move(board: &Board) -> Option<usize> {
        let mut index: Option<usize> = None;

        for (i, space) in board.spaces.iter().enumerate() {
            if *space == BoardSpace::Blank {
                index = Some(i);
                break;
            }
        }

        index
    }

    pub fn make_move(self, board: &mut Board) {
        let index = Bot::determin_move(board);

        if let Some(_) = index {
            board.set_by_index(self.player.player_space, index.unwrap());
        }
    }
}
