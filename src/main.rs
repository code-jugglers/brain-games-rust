// use std::io::{stdin};

#[derive(Debug)]
enum BoardSpace {
    X,
    Y,
    Blank,
}

#[derive(Debug)]
struct Coords {
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

    fn make_move(&mut self, space: BoardSpace, point: Coords) {
        let position = point.x * 3 + point.y;

        self.spaces[position as usize] = space;
    }
}

fn main() {
    let mut board = Board::empty();

    board.make_move(BoardSpace::X, Coords { x: 0, y: 0 });
    board.make_move(BoardSpace::Y, Coords { x: 0, y: 1 });

    println!("{:?}", board.spaces);
}
