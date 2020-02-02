use crate::board_space::BoardSpace;
use std::fmt::{Display, Formatter, Result};

pub type BoardSpaces = [[BoardSpace; 3]; 3];
pub type Move = [usize; 2];

#[derive(Debug)]
pub struct MoveEntry {
    pub key: String,
    pub space: BoardSpace,
    pub position: Move,
}

#[derive(Debug)]
pub struct Board {
    pub spaces: BoardSpaces,
    pub moves: Vec<MoveEntry>,
}
impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}
impl Board {
    pub fn new() -> Board {
        Board {
            spaces: [
                [BoardSpace::Empty, BoardSpace::Empty, BoardSpace::Empty],
                [BoardSpace::Empty, BoardSpace::Empty, BoardSpace::Empty],
                [BoardSpace::Empty, BoardSpace::Empty, BoardSpace::Empty],
            ],
            moves: vec![],
        }
    }

    fn key(&self) -> String {
        let mut result: String = String::new();

        for col in &self.spaces {
            for cell in col {
                result = result + cell.to_string();
            }
        }

        result
    }

    pub fn make_move(&mut self, space: BoardSpace, col: usize, row: usize) -> bool {
        if self.spaces[col][row] == BoardSpace::Empty {
            self.moves.push(MoveEntry {
                space,
                key: self.key(),
                position: [col, row],
            });

            self.spaces[col][row] = space;

            true
        } else {
            false
        }
    }

    pub fn determine_winner(&self) -> BoardSpace {
        // check columns
        for col in 0..=2 {
            let start = self.spaces[col][0];

            if start != BoardSpace::Empty
                && start == self.spaces[col][1]
                && start == self.spaces[col][2]
            {
                return self.spaces[col][0];
            }
        }

        // check rows
        for row in 0..=2 {
            let start = self.spaces[0][row];

            if start != BoardSpace::Empty
                && start == self.spaces[1][row]
                && start == self.spaces[2][row]
            {
                return self.spaces[0][row];
            }
        }

        // Check Diagonals Left -> Right
        if self.spaces[0][0] == self.spaces[1][1] && self.spaces[0][0] == self.spaces[2][2] {
            if self.spaces[0][0] != BoardSpace::Empty {
                return self.spaces[0][0];
            }
        }

        // Checl Diagonals Right -> Left
        if self.spaces[0][2] == self.spaces[1][1] && self.spaces[0][2] == self.spaces[2][0] {
            if self.spaces[0][2] != BoardSpace::Empty {
                return self.spaces[0][2];
            }
        }

        BoardSpace::Empty
    }

    pub fn to_string(&self) -> String {
        let mut result: String = String::new();

        result = result + "\n";

        for col in &self.spaces {
            for cell in col {
                result = result + cell.to_string() + "   ";
            }

            result = result + "\n\n";
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_3x3() {
        let board = Board::new();

        assert_eq!(board.spaces.len(), 3);
        assert_eq!(board.spaces[0].len(), 3);
        assert_eq!(board.spaces[1].len(), 3);
        assert_eq!(board.spaces[2].len(), 3);
    }

    #[test]
    fn should_place_x_at_0_0() {
        let mut board = Board::new();

        assert_eq!(board.spaces[0][0], BoardSpace::Empty);

        board.make_move(BoardSpace::X, 0, 0);

        assert_eq!(board.spaces[0][0], BoardSpace::X);
    }

    #[test]
    fn should_place_o_at_0_0() {
        let mut board = Board::new();

        assert_eq!(board.spaces[0][0], BoardSpace::Empty);

        board.make_move(BoardSpace::O, 0, 0);

        assert_eq!(board.spaces[0][0], BoardSpace::O);
    }

    #[test]
    fn should_return_true_if_space_is_empty() {
        let mut board = Board::new();

        assert_eq!(board.make_move(BoardSpace::X, 0, 0), true);
    }

    #[test]
    fn should_return_false_if_space_is_not_empty() {
        let mut board = Board::new();

        board.make_move(BoardSpace::X, 0, 0);

        assert_eq!(board.make_move(BoardSpace::X, 0, 0), false);
    }

    #[test]
    fn should_return_x_as_winner_for_col_1() {
        let mut board = Board::new();

        board.make_move(BoardSpace::X, 0, 0);
        board.make_move(BoardSpace::X, 0, 1);
        board.make_move(BoardSpace::X, 0, 2);

        assert_eq!(board.determine_winner(), BoardSpace::X);
    }

    #[test]
    fn should_return_x_as_winner_for_col_2() {
        let mut board = Board::new();

        board.make_move(BoardSpace::X, 1, 0);
        board.make_move(BoardSpace::X, 1, 1);
        board.make_move(BoardSpace::X, 1, 2);

        assert_eq!(board.determine_winner(), BoardSpace::X);
    }

    #[test]
    fn should_return_x_as_winner_for_col_3() {
        let mut board = Board::new();

        board.make_move(BoardSpace::X, 2, 0);
        board.make_move(BoardSpace::X, 2, 1);
        board.make_move(BoardSpace::X, 2, 2);

        assert_eq!(board.determine_winner(), BoardSpace::X);
    }

    #[test]
    fn should_return_x_as_winner_for_row_1() {
        let mut board = Board::new();

        board.make_move(BoardSpace::X, 0, 0);
        board.make_move(BoardSpace::X, 1, 0);
        board.make_move(BoardSpace::X, 2, 0);

        assert_eq!(board.determine_winner(), BoardSpace::X);
    }

    #[test]
    fn should_return_x_as_winner_for_row_2() {
        let mut board = Board::new();

        board.make_move(BoardSpace::X, 0, 1);
        board.make_move(BoardSpace::X, 1, 1);
        board.make_move(BoardSpace::X, 2, 1);

        assert_eq!(board.determine_winner(), BoardSpace::X);
    }

    #[test]
    fn should_return_o_as_winner_for_row_3() {
        let mut board = Board::new();

        board.make_move(BoardSpace::O, 0, 2);
        board.make_move(BoardSpace::O, 1, 2);
        board.make_move(BoardSpace::O, 2, 2);

        assert_eq!(board.determine_winner(), BoardSpace::O);
    }

    #[test]
    fn should_return_o_as_winner_for_diag_left_right() {
        let mut board = Board::new();

        board.make_move(BoardSpace::O, 0, 0);
        board.make_move(BoardSpace::O, 1, 1);
        board.make_move(BoardSpace::O, 2, 2);

        assert_eq!(board.determine_winner(), BoardSpace::O);
    }

    #[test]
    fn should_return_o_as_winner_for_diag_right_left() {
        let mut board = Board::new();

        board.make_move(BoardSpace::O, 0, 2);
        board.make_move(BoardSpace::O, 1, 1);
        board.make_move(BoardSpace::O, 2, 0);

        assert_eq!(board.determine_winner(), BoardSpace::O);
    }

    #[test]
    fn should_keep_track_of_moves() {
        let mut board = Board::new();

        board.make_move(BoardSpace::X, 0, 2);
        board.make_move(BoardSpace::O, 1, 1);
        board.make_move(BoardSpace::X, 2, 0);

        assert_eq!(board.moves[0].key, String::from("---------"));
        assert_eq!(board.moves[0].space, BoardSpace::X);
        assert_eq!(board.moves[0].position[0], 0);
        assert_eq!(board.moves[0].position[1], 2);

        assert_eq!(board.moves[1].key, String::from("--X------"));
        assert_eq!(board.moves[1].space, BoardSpace::O);
        assert_eq!(board.moves[1].position[0], 1);
        assert_eq!(board.moves[1].position[1], 1);

        assert_eq!(board.moves[2].key, String::from("--X-O----"));
        assert_eq!(board.moves[2].space, BoardSpace::X);
        assert_eq!(board.moves[2].position[0], 2);
        assert_eq!(board.moves[2].position[1], 0);
    }
}
