use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq, Clone)]
pub enum BoardSpace {
    X,
    O,
    Blank,
}
impl Display for BoardSpace {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}
impl BoardSpace {
    fn to_string(&self) -> &str {
        match self {
            BoardSpace::Blank => "-",
            BoardSpace::X => "X",
            BoardSpace::O => "O",
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug)]
pub struct Board {
    pub spaces: Vec<BoardSpace>,
    result_cache: HashMap<String, BoardSpace>,
}
impl Board {
    const ROWS: usize = 3;
    const COLS: usize = 3;

    pub fn empty() -> Board {
        Board {
            spaces: vec![
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
                BoardSpace::Blank,
            ],
            result_cache: HashMap::new(),
        }
    }

    pub fn set(&mut self, space: &BoardSpace, pos: Position) {
        let position = pos.row * 3 + pos.col;

        if self.spaces[position] == BoardSpace::Blank {
            self.spaces[position] = space.clone();
        }
    }

    pub fn set_by_index(&mut self, space: &BoardSpace, index: usize) {
        if self.spaces[index] == BoardSpace::Blank {
            self.spaces[index] = space.clone();
        }
    }

    pub fn key(&self) -> String {
        let mut key_value = String::new();

        for space in self.spaces.iter() {
            key_value = key_value + space.to_string()
        }

        key_value
    }

    pub fn determine_winner(&mut self) -> BoardSpace {
        // Check rows
        for index in 0..Board::ROWS {
            let row = index * Board::ROWS;

            if let Some(value) = &self.check_spaces(row, row + 1, row + 2) {
                return value.clone();
            }
        }

        // check cols
        for col in 0..Board::COLS {
            if let Some(value) = &self.check_spaces(col, col + Board::ROWS, col + Board::ROWS * 2) {
                return value.clone();
            }
        }

        if let Some(value) = &self.check_spaces(0, 4, 8) {
            return value.clone();
        } else if let Some(value) = &self.check_spaces(2, 4, 6) {
            return value.clone();
        }

        BoardSpace::Blank
    }

    pub fn print(&self) {
        for (index, space) in self.spaces.iter().enumerate() {
            if index % 3 == 0 {
                println!();
            }

            print!("{}", space);
        }

        println!();
    }

    fn check_spaces(
        &mut self,
        space_1: usize,
        space_2: usize,
        space_3: usize,
    ) -> Option<BoardSpace> {
        let space_1_parsed = &self.spaces[space_1];
        let space_2_parsed = &self.spaces[space_2];
        let space_3_parsed = &self.spaces[space_3];

        if space_1_parsed == space_2_parsed && space_2_parsed == space_3_parsed {
            if *space_1_parsed != BoardSpace::Blank {
                let key = &self.key();

                &self
                    .result_cache
                    .insert(key.clone(), space_1_parsed.clone());

                return Some(space_1_parsed.clone());
            }
        }

        None
    }
}
