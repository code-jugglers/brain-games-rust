#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BoardSpaceState {
    Empty,
    X,
    O,
}

pub type BoardState = [BoardSpaceState; 9];

pub struct Board {
    pub spaces: BoardState,
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
        }
    }

    pub fn set_by_index(&mut self, i: usize, space: BoardSpaceState) {
        self.spaces[i] = space;
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

    pub fn determine_winner(&self) -> Option<BoardSpaceState> {
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

        None
    }

    fn check_spaces(&self, i_1: usize, i_2: usize, i_3: usize) -> Option<BoardSpaceState> {
        let space_1 = self.spaces[i_1];
        let space_2 = self.spaces[i_2];
        let space_3 = self.spaces[i_3];

        if space_1 == space_2 && space_2 == space_3 {
            if space_1 != BoardSpaceState::Empty {
                return Some(space_1);
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

        board.set_by_index(0, BoardSpaceState::X);

        assert_eq!(
            board.spaces,
            [
                BoardSpaceState::X,
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

        board.set(0, 0, BoardSpaceState::X);
        board.set(1, 1, BoardSpaceState::X);
        board.set(2, 2, BoardSpaceState::X);

        assert_eq!(
            board.spaces,
            [
                BoardSpaceState::X,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::X,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::Empty,
                BoardSpaceState::X,
            ]
        )
    }

    #[test]
    fn should_check_if_moves_available_1() {
        let board = Board::new();

        assert_eq!(board.moves_available(), true);

        let mut board = Board::new();

        for space in 0..9 {
            board.set_by_index(space, BoardSpaceState::X);
        }

        assert_eq!(board.moves_available(), false);
    }

    #[test]
    fn should_check_if_moves_available_2() {
        let mut board = Board::new();

        board.set_by_index(0, BoardSpaceState::X);
        board.set_by_index(1, BoardSpaceState::O);

        assert_eq!(board.moves_available(), true);
    }

    #[test]
    fn should_determine_row_winner_1() {
        let mut board = Board::new();
        let row = 0;

        board.set(row, 0, BoardSpaceState::X);
        board.set(row, 1, BoardSpaceState::X);
        board.set(row, 2, BoardSpaceState::X);

        assert_eq!(board.determine_winner(), Some(BoardSpaceState::X));
    }

    #[test]
    fn should_determine_row_winner_2() {
        let mut board = Board::new();
        let row = 1;

        board.set(row, 0, BoardSpaceState::X);
        board.set(row, 1, BoardSpaceState::X);
        board.set(row, 2, BoardSpaceState::X);

        assert_eq!(board.determine_winner(), Some(BoardSpaceState::X));
    }

    #[test]
    fn should_determine_row_winner_3() {
        let mut board = Board::new();
        let row = 2;

        board.set(row, 0, BoardSpaceState::X);
        board.set(row, 1, BoardSpaceState::X);
        board.set(row, 2, BoardSpaceState::X);

        assert_eq!(board.determine_winner(), Some(BoardSpaceState::X));
    }

    #[test]
    fn should_determine_col_winner_1() {
        let mut board = Board::new();
        let col = 0;

        board.set(0, col, BoardSpaceState::O);
        board.set(1, col, BoardSpaceState::O);
        board.set(2, col, BoardSpaceState::O);

        assert_eq!(board.determine_winner(), Some(BoardSpaceState::O));
    }

    #[test]
    fn should_determine_col_winner_2() {
        let mut board = Board::new();
        let col = 1;

        board.set(0, col, BoardSpaceState::O);
        board.set(1, col, BoardSpaceState::O);
        board.set(2, col, BoardSpaceState::O);

        assert_eq!(board.determine_winner(), Some(BoardSpaceState::O));
    }

    #[test]
    fn should_determine_col_winner_3() {
        let mut board = Board::new();
        let col = 2;

        board.set(0, col, BoardSpaceState::O);
        board.set(1, col, BoardSpaceState::O);
        board.set(2, col, BoardSpaceState::O);

        assert_eq!(board.determine_winner(), Some(BoardSpaceState::O));
    }

    #[test]
    fn should_determine_diag_winner_1() {
        let mut board = Board::new();

        board.set(0, 0, BoardSpaceState::X);
        board.set(1, 1, BoardSpaceState::X);
        board.set(2, 2, BoardSpaceState::X);

        assert_eq!(board.determine_winner(), Some(BoardSpaceState::X));
    }

    #[test]
    fn should_determine_diag_winner_2() {
        let mut board = Board::new();

        board.set(0, 2, BoardSpaceState::O);
        board.set(1, 1, BoardSpaceState::O);
        board.set(2, 0, BoardSpaceState::O);

        assert_eq!(board.determine_winner(), Some(BoardSpaceState::O));
    }
}
