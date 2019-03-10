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
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
pub struct Board {
    pub spaces: Vec<BoardSpace>,
}
impl Board {
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
        }
    }

    pub fn set(&mut self, space: &BoardSpace, pos: Position) {
        let position = pos.x * 3 + pos.y;

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

    pub fn print(&self) {
        for (index, space) in self.spaces.iter().enumerate() {
            if index % 3 == 0 {
                println!();
            }

            print!("{}", space);
        }

        println!();
    }
}
