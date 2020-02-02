use std::fmt::{Display, Formatter, Result};

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BoardSpace {
    Empty,
    X,
    O,
}
impl Display for BoardSpace {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.to_string())
    }
}
impl BoardSpace {
    pub fn to_string(&self) -> &str {
        match self {
            BoardSpace::Empty => "-",
            BoardSpace::X => "X",
            BoardSpace::O => "O",
        }
    }
}
