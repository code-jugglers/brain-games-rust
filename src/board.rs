use std::fmt;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Space {
    Empty,
    Player(Player),
}

impl Space {
    pub fn to_str(&self) -> &str {
        match self {
            Space::Player(Player::X) => "X",
            Space::Player(Player::O) => "O",
            Space::Empty => "-",
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameResult {
    Winner(Player),
    Tie,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameResult::Winner(Player::X) => write!(f, "X"),
            GameResult::Winner(Player::O) => write!(f, "O"),
            GameResult::Tie => write!(f, "TIE"),
        }
    }
}

pub type BoardState = Vec<Space>;

#[derive(Debug, PartialEq)]
pub struct Move {
    pub key: u64,
    pub space: Space,
    pub index: usize,
}

pub struct Board {
    pub rows: usize,
    pub cols: usize,
    pub spaces: BoardState,
    pub moves: Vec<Move>,
}

impl Board {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut spaces: Vec<Space> = Vec::new();

        for _ in 0..(rows * cols) {
            spaces.push(Space::Empty);
        }

        Self {
            rows,
            cols,
            spaces,
            moves: vec![],
        }
    }

    pub fn key_as_u64(&self) -> u64 {
        let board_size = self.spaces.len() as u64;
        let mut index = 0;
        let mut total = 0;

        for space in &self.spaces {
            let space_value = match space {
                &Space::Player(Player::X) => 2,
                &Space::Player(Player::O) => 1,
                &Space::Empty => 0,
            };

            if space_value > 0 {
                total += space_value * board_size.pow(index);
            }

            index += 1;
        }

        total
    }

    #[allow(unused)]
    pub fn key_as_string(&self) -> String {
        let mut result = String::new();

        for space in &self.spaces {
            result = result + &space.to_string();
        }

        result
    }

    pub fn get_available_spaces(&self) -> Vec<usize> {
        self.spaces
            .iter()
            .enumerate()
            .filter_map(|(i, space)| {
                if *space == Space::Empty {
                    Some(i)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>()
    }

    pub fn set_by_index(&mut self, index: usize, space: Space) -> Result<(), ()> {
        if let Some(current_space) = self.spaces.get(index) {
            if current_space == &Space::Empty {
                let key = self.key_as_u64();

                self.moves.push(Move { index, key, space });

                self.spaces[index] = space;

                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    #[allow(unused)]
    pub fn set(&mut self, row: usize, col: usize, space: Space) -> Result<(), ()> {
        let index = row * self.rows + col;

        self.set_by_index(index, space)
    }

    pub fn moves_available(&self) -> bool {
        self.spaces.iter().any(|&space| space == Space::Empty)
    }

    pub fn determine_winner(&self) -> Option<GameResult> {
        let mut rl_res: Vec<Space> = Vec::new();
        let mut lr_res: Vec<Space> = Vec::new();

        for x in 0..self.rows {
            rl_res.push(self.spaces[x * self.rows + x]);
            lr_res.push(self.spaces[x * self.rows + (self.rows - 1 - x)]);

            let mut row_res: Vec<Space> = Vec::new();
            let mut col_res: Vec<Space> = Vec::new();

            for y in 0..self.cols {
                row_res.push(self.spaces[x * self.rows + y]);
                col_res.push(self.spaces[y * self.rows + x]);
            }

            if row_res.windows(2).all(|w| w[0] == w[1]) {
                if let Space::Player(player) = row_res[0] {
                    return Some(GameResult::Winner(player));
                }
            }

            if col_res.windows(2).all(|w| w[0] == w[1]) {
                if let Space::Player(player) = col_res[0] {
                    return Some(GameResult::Winner(player));
                }
            }
        }

        if rl_res.windows(2).all(|w| w[0] == w[1]) {
            if let Space::Player(player) = rl_res[0] {
                return Some(GameResult::Winner(player));
            }
        }

        if lr_res.windows(2).all(|w| w[0] == w[1]) {
            if let Space::Player(player) = lr_res[0] {
                return Some(GameResult::Winner(player));
            }
        }

        if !self.moves_available() {
            return Some(GameResult::Tie);
        }

        None
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = String::new();

        for (i, space) in self.spaces.iter().enumerate() {
            if i % self.rows == 0 {
                grid += "\n"
            }

            grid += space.to_str();
            grid += " ";
        }

        write!(f, "{}", grid)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_set_by_index() {
        let mut board = Board::new(3, 3);

        board.set_by_index(0, Space::Player(Player::X));

        assert_eq!(
            board.spaces,
            [
                Space::Player(Player::X),
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Empty,
            ]
        )
    }

    #[test]
    fn should_set_by_row_col() {
        let mut board = Board::new(3, 3);

        board.set(0, 0, Space::Player(Player::X));
        board.set(1, 1, Space::Player(Player::X));
        board.set(2, 2, Space::Player(Player::X));

        assert_eq!(
            board.spaces,
            [
                Space::Player(Player::X),
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Player(Player::X),
                Space::Empty,
                Space::Empty,
                Space::Empty,
                Space::Player(Player::X),
            ]
        )
    }

    #[test]
    fn should_track_past_moves() {
        let mut board = Board::new(3, 3);

        board.set(0, 0, Space::Player(Player::X));
        board.set(1, 1, Space::Player(Player::O));
        board.set(2, 2, Space::Player(Player::X));

        assert_eq!(
            board.moves,
            [
                Move {
                    index: 0,
                    key: 0 as u64,
                    space: Space::Player(Player::X)
                },
                Move {
                    index: 4,
                    key: 2 as u64,
                    space: Space::Player(Player::O)
                },
                Move {
                    index: 8,
                    key: 6563 as u64,
                    space: Space::Player(Player::X)
                }
            ]
        )
    }

    #[test]
    fn should_check_if_moves_available_1() {
        let board = Board::new(3, 3);

        assert_eq!(board.moves_available(), true);

        let mut board = Board::new(3, 3);

        for space in 0..9 {
            board.set_by_index(space, Space::Player(Player::X));
        }

        assert_eq!(board.moves_available(), false);
    }

    #[test]
    fn should_check_if_moves_available_2() {
        let mut board = Board::new(3, 3);

        board.set_by_index(0, Space::Player(Player::X));
        board.set_by_index(1, Space::Player(Player::O));

        assert_eq!(board.moves_available(), true);
    }

    #[test]
    fn should_determine_row_winner_1() {
        let mut board = Board::new(3, 3);
        let row = 0;

        board.set(row, 0, Space::Player(Player::X));
        board.set(row, 1, Space::Player(Player::X));
        board.set(row, 2, Space::Player(Player::X));

        assert_eq!(
            board.determine_winner(),
            Some(GameResult::Winner(Player::X))
        );
    }

    #[test]
    fn should_determine_row_winner_2() {
        let mut board = Board::new(3, 3);
        let row = 1;

        board.set(row, 0, Space::Player(Player::X));
        board.set(row, 1, Space::Player(Player::X));
        board.set(row, 2, Space::Player(Player::X));

        assert_eq!(
            board.determine_winner(),
            Some(GameResult::Winner(Player::X))
        );
    }

    #[test]
    fn should_determine_row_winner_3() {
        let mut board = Board::new(3, 3);
        let row = 2;

        board.set(row, 0, Space::Player(Player::X));
        board.set(row, 1, Space::Player(Player::X));
        board.set(row, 2, Space::Player(Player::X));

        assert_eq!(
            board.determine_winner(),
            Some(GameResult::Winner(Player::X))
        );
    }

    #[test]
    fn should_determine_col_winner_1() {
        let mut board = Board::new(3, 3);
        let col = 0;

        board.set(0, col, Space::Player(Player::O));
        board.set(1, col, Space::Player(Player::O));
        board.set(2, col, Space::Player(Player::O));

        assert_eq!(
            board.determine_winner(),
            Some(GameResult::Winner(Player::O))
        );
    }

    #[test]
    fn should_determine_col_winner_2() {
        let mut board = Board::new(3, 3);
        let col = 1;

        board.set(0, col, Space::Player(Player::O));
        board.set(1, col, Space::Player(Player::O));
        board.set(2, col, Space::Player(Player::O));

        assert_eq!(
            board.determine_winner(),
            Some(GameResult::Winner(Player::O))
        );
    }

    #[test]
    fn should_determine_col_winner_3() {
        let mut board = Board::new(3, 3);
        let col = 2;

        board.set(0, col, Space::Player(Player::O));
        board.set(1, col, Space::Player(Player::O));
        board.set(2, col, Space::Player(Player::O));

        assert_eq!(
            board.determine_winner(),
            Some(GameResult::Winner(Player::O))
        );
    }

    #[test]
    fn should_determine_diag_winner_1() {
        let mut board = Board::new(3, 3);

        board.set(0, 0, Space::Player(Player::X));
        board.set(1, 1, Space::Player(Player::X));
        board.set(2, 2, Space::Player(Player::X));

        assert_eq!(
            board.determine_winner(),
            Some(GameResult::Winner(Player::X))
        );
    }

    #[test]
    fn should_determine_diag_winner_2() {
        let mut board = Board::new(3, 3);

        board.set(0, 2, Space::Player(Player::O));
        board.set(1, 1, Space::Player(Player::O));
        board.set(2, 0, Space::Player(Player::O));

        assert_eq!(
            board.determine_winner(),
            Some(GameResult::Winner(Player::O))
        );
    }
}
