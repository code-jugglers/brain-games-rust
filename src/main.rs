// use std::io::{stdin};

#[derive(Debug, PartialEq)]
enum BoardSpace {
    X,
    O,
    Blank,
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Board {
    spaces: Vec<BoardSpace>,
}

impl Board {
    fn empty() -> Board {
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

    fn set(&mut self, space: BoardSpace, pos: Position) {
        // cast i32 to ussize. could probably just use usize in the spaces
        let position = (pos.x * 3 + pos.y) as usize;

        if self.spaces[position] == BoardSpace::Blank {
            self.spaces[position] = space;
        }
    }

    // TODO: print actual representation of board
    fn print(&self) {
        println!("{:?}", &self.spaces[0..=2]);
        println!("{:?}", &self.spaces[3..=5]);
        println!("{:?}", &self.spaces[6..=8]);
    }
}

struct Player {
    player_space: BoardSpace,
}

impl Player {
    fn new(player_space: BoardSpace) -> Player {
        Player { player_space }
    }

    fn make_move(self, board: &mut Board, post: Position) {
        board.set(self.player_space, post)
    }
}

fn main() {
    let mut board: Board = Board::empty();

    let x = Player::new(BoardSpace::X);
    let o = Player::new(BoardSpace::O);

    x.make_move(&mut board, Position { x: 0, y: 0 });

    o.make_move(&mut board, Position { x: 0, y: 1 });

    board.print();
}
