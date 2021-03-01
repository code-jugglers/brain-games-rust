use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BoardSpaceState {
    Empty,
    Player(Player),
}

impl fmt::Display for BoardSpaceState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardSpaceState::Player(Player::X) => write!(f, "X"),
            BoardSpaceState::Player(Player::O) => write!(f, "O"),
            BoardSpaceState::Empty => write!(f, "-"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum GameResult {
    XWin,
    OWin,
    Tie,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameResult::XWin => write!(f, "X"),
            GameResult::OWin => write!(f, "O"),
            GameResult::Tie => write!(f, "TIE"),
        }
    }
}

pub type BoardState = [BoardSpaceState; 9];

#[derive(Debug, PartialEq)]
pub struct Move {
    pub key: String,
    pub space: BoardSpaceState,
    pub index: usize,
}

pub struct Board {
    pub spaces: BoardState,
    pub moves: Vec<Move>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            spaces: [
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
            ],
            moves: vec![],
        }
    }

    pub fn key(&self) -> String {
        let mut result = String::new();

        for space in &self.spaces {
            result = result + &space.to_string();
        }

        result
    }

    pub fn get_available_spaces(&self) -> Vec<usize> {
        let mut available_moves = Vec::new();

        for (index, space) in self.spaces.iter().enumerate() {
            if space == &BoardSpaceState::Empty {
                available_moves.push(index);
            }
        }

        available_moves
    }

    pub fn set_by_index(&mut self, index: usize, space: BoardSpaceState) {
        self.moves.push(Move {
            index,
            key: self.key(),
            space,
        });

        self.spaces[index] = space;
    }

    #[allow(dead_code)]
    pub fn set(&mut self, row: usize, col: usize, space: BoardSpaceState) {
        let index = self.get_index(row, col);

        self.set_by_index(index, space);
    }

    pub fn moves_available(&self) -> bool {
        self.spaces
            .iter()
            .any(|&space| space == BoardSpaceState::Empty)
    }

    pub fn determine_winner(&self) -> Option<GameResult> {
        // check rows
        for row in 0..3 {
            let check = self.check_spaces(
                self.get_index(row, 0),
                self.get_index(row, 1),
                self.get_index(row, 2),
            );

            if let Some(result) = check {
                return Some(result);
            }
        }

        // check cols
        for col in 0..3 {
            let check = self.check_spaces(
                self.get_index(0, col),
                self.get_index(1, col),
                self.get_index(2, col),
            );

            if let Some(result) = check {
                return Some(result);
            }
        }

        // check diag L -> R
        if let Some(result) = self.check_spaces(0, 4, 8) {
            return Some(result);
        }

        // check diag R -> L
        if let Some(result) = self.check_spaces(2, 4, 6) {
            return Some(result);
        }

        let moves_available = self.moves_available();

        if !moves_available {
            return Some(GameResult::Tie);
        }

        None
    }

    fn check_spaces(&self, i_1: usize, i_2: usize, i_3: usize) -> Option<GameResult> {
        let space_1 = self.spaces[i_1];
        let space_2 = self.spaces[i_2];
        let space_3 = self.spaces[i_3];

        if space_1 == space_2 && space_2 == space_3 {
            if space_1 == BoardSpaceState::Player(Player::X) {
                return Some(GameResult::XWin);
            } else if space_1 == BoardSpaceState::Player(Player::O) {
                return Some(GameResult::OWin);
            }
        }

        None
    }

    fn get_index(&self, row: usize, col: usize) -> usize {
        row * 3 + col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_set_by_index() {
        let mut board = Board::new();

        board.set_by_index(0, BoardSpaceState::Player(Player::X));

        assert_eq!(
            board.spaces,
            [
                BoardSpaceState::Player(Player::X),
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
            ]
        )
    }

    #[test]
    fn should_set_by_row_col() {
        let mut board = Board::new();

        board.set(0, 0, BoardSpaceState::Player(Player::X));
        board.set(1, 1, BoardSpaceState::Player(Player::X));
        board.set(2, 2, BoardSpaceState::Player(Player::X));

        assert_eq!(
            board.spaces,
            [
                BoardSpaceState::Player(Player::X),
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Player(Player::X),
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Player(Player::X),
            ]
        )
    }

    #[test]
    fn should_track_past_moves() {
        let mut board = Board::new();

        board.set(0, 0, BoardSpaceState::Player(Player::X));
        board.set(1, 1, BoardSpaceState::Player(Player::O));
        board.set(2, 2, BoardSpaceState::Player(Player::X));

        assert_eq!(
            board.moves,
            [
                Move {
                    index: 0,
                    key: "---------".to_string(),
                    space: BoardSpaceState::Player(Player::X)
                },
                Move {
                    index: 4,
                    key: "X--------".to_string(),
                    space: BoardSpaceState::Player(Player::O)
                },
                Move {
                    index: 8,
                    key: "X---O----".to_string(),
                    space: BoardSpaceState::Player(Player::X)
                }
            ]
        )
    }

    #[test]
    fn should_check_if_moves_available_1() {
        let board = Board::new();

        assert_eq!(board.moves_available(), true);

        let mut board = Board::new();

        for space in 0..9 {
            board.set_by_index(space, BoardSpaceState::Player(Player::X));
        }

        assert_eq!(board.moves_available(), false);
    }

    #[test]
    fn should_check_if_moves_available_2() {
        let mut board = Board::new();

        board.set_by_index(0, BoardSpaceState::Player(Player::X));
        board.set_by_index(1, BoardSpaceState::Player(Player::O));

        assert_eq!(board.moves_available(), true);
    }

    #[test]
    fn should_determine_row_winner_1() {
        let mut board = Board::new();
        let row = 0;

        board.set(row, 0, BoardSpaceState::Player(Player::X));
        board.set(row, 1, BoardSpaceState::Player(Player::X));
        board.set(row, 2, BoardSpaceState::Player(Player::X));

        assert_eq!(
            board.determine_winner(),
            Some(BoardSpaceState::Player(Player::X))
        );
    }

    #[test]
    fn should_determine_row_winner_2() {
        let mut board = Board::new();
        let row = 1;

        board.set(row, 0, BoardSpaceState::Player(Player::X));
        board.set(row, 1, BoardSpaceState::Player(Player::X));
        board.set(row, 2, BoardSpaceState::Player(Player::X));

        assert_eq!(
            board.determine_winner(),
            Some(BoardSpaceState::Player(Player::X))
        );
    }

    #[test]
    fn should_determine_row_winner_3() {
        let mut board = Board::new();
        let row = 2;

        board.set(row, 0, BoardSpaceState::Player(Player::X));
        board.set(row, 1, BoardSpaceState::Player(Player::X));
        board.set(row, 2, BoardSpaceState::Player(Player::X));

        assert_eq!(
            board.determine_winner(),
            Some(BoardSpaceState::Player(Player::X))
        );
    }

    #[test]
    fn should_determine_col_winner_1() {
        let mut board = Board::new();
        let col = 0;

        board.set(0, col, BoardSpaceState::Player(Player::O));
        board.set(1, col, BoardSpaceState::Player(Player::O));
        board.set(2, col, BoardSpaceState::Player(Player::O));

        assert_eq!(
            board.determine_winner(),
            Some(BoardSpaceState::Player(Player::O))
        );
    }

    #[test]
    fn should_determine_col_winner_2() {
        let mut board = Board::new();
        let col = 1;

        board.set(0, col, BoardSpaceState::Player(Player::O));
        board.set(1, col, BoardSpaceState::Player(Player::O));
        board.set(2, col, BoardSpaceState::Player(Player::O));

        assert_eq!(
            board.determine_winner(),
            Some(BoardSpaceState::Player(Player::O))
        );
    }

    #[test]
    fn should_determine_col_winner_3() {
        let mut board = Board::new();
        let col = 2;

        board.set(0, col, BoardSpaceState::Player(Player::O));
        board.set(1, col, BoardSpaceState::Player(Player::O));
        board.set(2, col, BoardSpaceState::Player(Player::O));

        assert_eq!(
            board.determine_winner(),
            Some(BoardSpaceState::Player(Player::O))
        );
    }

    #[test]
    fn should_determine_diag_winner_1() {
        let mut board = Board::new();

        board.set(0, 0, BoardSpaceState::Player(Player::X));
        board.set(1, 1, BoardSpaceState::Player(Player::X));
        board.set(2, 2, BoardSpaceState::Player(Player::X));

        assert_eq!(
            board.determine_winner(),
            Some(BoardSpaceState::Player(Player::X))
        );
    }

    #[test]
    fn should_determine_diag_winner_2() {
        let mut board = Board::new();

        board.set(0, 2, BoardSpaceState::Player(Player::O));
        board.set(1, 1, BoardSpaceState::Player(Player::O));
        board.set(2, 0, BoardSpaceState::Player(Player::O));

        assert_eq!(
            board.determine_winner(),
            Some(BoardSpaceState::Player(Player::O))
        );
    }
}
