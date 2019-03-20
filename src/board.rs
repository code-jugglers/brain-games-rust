use std::fmt::{Display, Formatter, Result};

/// The board is modeled as a Vec of BoardSpaces, indexed as shown below:
/// 0 1 2
/// 3 4 5
/// 6 7 8

#[derive(PartialEq, Clone, Copy)]
pub enum BoardSpace {
    X,
    O,
    Empty,
}
impl Display for BoardSpace {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}

impl BoardSpace {
    fn to_string(&self) -> &str {
        match self {
            BoardSpace::Empty => "-",
            BoardSpace::X => "X",
            BoardSpace::O => "O",
        }
    }
}

#[derive(PartialEq, Clone)]
pub enum GameResult {
    XWin,
    OWin,
    Draw,
    Undecided,
}
impl Display for GameResult {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            GameResult::XWin => write!(f, "X Wins!"),
            GameResult::OWin => write!(f, "O Wins!"),
            GameResult::Draw => write!(f, "It is a DRAW!"),
            GameResult::Undecided => write!(f, "No winner yet"),
        }
    }
}

#[derive(Clone)]
pub struct HistoryEntry {
    pub key: String,
    pub player: BoardSpace,
    pub move_index: usize,
}

pub struct Board {
    pub spaces: [BoardSpace; 9],
    pub move_history: Vec<HistoryEntry>,
}
impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.create_board_visualization())
    }
}
impl Board {
    const ROWS: usize = 3;
    const COLS: usize = 3;

    pub fn new() -> Board {
        Board {
            spaces: Board::create_board_spaces(),
            move_history: vec![],
        }
    }

    pub fn reset(&mut self) {
        self.spaces = Board::create_board_spaces();
        self.move_history = vec![];
    }

    pub fn key(&self) -> String {
        self.spaces
            .iter()
            .fold(String::new(), |key, x| key + &x.to_string())
    }

    pub fn create_board_visualization(&self) -> String {
        self.spaces
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, x)| {
                let mut res = acc;

                if i % 3 == 0 {
                    res = res + "\n"
                }

                res = res + &x.to_string() + " ";

                res
            })
    }

    pub fn set_by_index(&mut self, move_index: usize, player: BoardSpace) -> Option<usize> {
        if self.spaces[move_index] == BoardSpace::Empty {
            let key = self.key();

            self.move_history.push(HistoryEntry {
                key,
                move_index,
                player,
            });

            self.spaces[move_index] = player;

            return Some(move_index);
        }

        None
    }

    pub fn get_available_spaces(&self) -> Vec<usize> {
        let mut available_spaces = Vec::new();

        for (index, space) in self.spaces.iter().enumerate() {
            if *space == BoardSpace::Empty {
                available_spaces.push(index)
            }
        }

        available_spaces
    }

    pub fn check_board(&self) -> GameResult {
        
        // Check rows
        for index in 0..Board::ROWS {
            let row = index * Board::ROWS;
            let res = self.check_spaces(row, row + 1, row + 2);

            if res != GameResult::Undecided {
                return res;
            }
        }

        // check cols
        for col in 0..Board::COLS {
            let res = self.check_spaces(col, col + Board::ROWS, col + Board::ROWS * 2);

            if res != GameResult::Undecided {
                return res;
            }
        }

        let right_diag = self.check_spaces(0, 4, 8);
        if right_diag != GameResult::Undecided {
            return right_diag;
        }

        let left_diag = self.check_spaces(2, 4, 6);
        if left_diag != GameResult::Undecided {
            return left_diag;
        }

        if self.spaces.iter().all(|item| *item != BoardSpace::Empty) {
            return GameResult::Draw;
        }

        GameResult::Undecided
    }

    fn check_spaces(&self, space_1: usize, space_2: usize, space_3: usize) -> GameResult {
        let space_1_parsed = &self.spaces[space_1];
        let space_2_parsed = &self.spaces[space_2];
        let space_3_parsed = &self.spaces[space_3];

        if space_1_parsed == space_2_parsed && space_2_parsed == space_3_parsed {
            if *space_1_parsed != BoardSpace::Empty {
                return match space_1_parsed {
                    BoardSpace::X => GameResult::XWin,
                    BoardSpace::O => GameResult::OWin,
                    BoardSpace::Empty => GameResult::Undecided,
                };
            }
        }

        GameResult::Undecided
    }

    fn create_board_spaces() -> [BoardSpace; 9] {
        [
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_result_should_be_correct_horizontally() {
        let mut board1 = Board::new();

        board1.spaces = [
            BoardSpace::X,
            BoardSpace::X,
            BoardSpace::X,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
        ];

        assert!(board1.check_board() == GameResult::XWin);

        let mut board2 = Board::new();

        board2.spaces = [
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::O,
            BoardSpace::O,
            BoardSpace::O,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
        ];

        assert!(board2.check_board() == GameResult::OWin);

        let mut board3 = Board::new();

        board3.spaces = [
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::X,
            BoardSpace::X,
            BoardSpace::X,
        ];

        assert!(board3.check_board() == GameResult::XWin);

    }

    #[test]
    fn game_result_should_be_vertically() {
        let mut board = Board::new();

        board.spaces = [
            BoardSpace::X,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::X,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::X,
            BoardSpace::Empty,
            BoardSpace::Empty,
        ];

        assert!(board.check_board() == GameResult::XWin);
    }

    #[test]
    fn game_result_should_be_correct_diagonally() {
        let mut board = Board::new();

        board.spaces = [
            BoardSpace::X,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::X,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::X,
        ];

        assert!(board.check_board() == GameResult::XWin);

        board.reset();

        board.spaces = [
            BoardSpace::Empty,
            BoardSpace::Empty,
            BoardSpace::X,
            BoardSpace::Empty,
            BoardSpace::X,
            BoardSpace::Empty,
            BoardSpace::X,
            BoardSpace::Empty,
            BoardSpace::Empty,
        ];

        assert!(board.check_board() == GameResult::XWin);
    }
}
