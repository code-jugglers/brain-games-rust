mod board;
mod bot;

use board::{Board, BoardSpaceState, Player};
use bot::Bot;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Game {
    board: Board,
    player_x: Bot,
    player_o: Bot,
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            player_x: Bot::new(BoardSpaceState::Player(Player::X)),
            player_o: Bot::new(BoardSpaceState::Player(Player::O)),
        }
    }

    pub fn board(&self) -> String {
        let mut val = String::new();

        for (i, space) in self.board.spaces.iter().enumerate() {
            if i > 0 && i % 3 == 0 {
                val += &String::from("\n")
            }

            val += &space.to_string();
            val += &String::from(" ");
        }

        val
    }

    pub fn reset_board(&mut self) {
        self.board = Board::new();
    }

    pub fn make_move_x(&mut self, index: usize) {
        self.board.set_by_index(index, BoardSpaceState::Player(Player::X));

        let bot_move = self.player_o.determine_move(&self.board);

        if let Some(m) = bot_move {
            self.board
                .set_by_index(m, BoardSpaceState::Player(Player::O));
        }
    }

    pub fn train(&mut self, game_count: u32) -> String {
        let mut x_win = 0;
        let mut o_win = 0;
        let mut tie = 0;

        for _ in 1..=game_count {
            let res = self.play();

            if let Some(res) = res {
                if res == "X Wins!" {
                    x_win += 1;
                } else if res == "O Wins!" {
                    o_win += 1;
                } else {
                    tie += 1;
                }
            }
        }

        self.reset_board();

        "X: ".to_string()
            + &x_win.to_string()
            + "\nO: "
            + &o_win.to_string()
            + "\nTIE: "
            + &tie.to_string()
    }

    pub fn play(&mut self) -> Option<String> {
        self.reset_board();

        let mut current_player: BoardSpaceState = BoardSpaceState::Player(Player::X);
        let mut winner: Option<BoardSpaceState> = None;
        let mut moves_available = true;

        while moves_available && winner == None {
            if current_player == BoardSpaceState::Player(Player::X) {
                let current_move = self.player_x.determine_move(&self.board);
                let space_state = BoardSpaceState::Player(Player::X);

                self.board.set_by_index(current_move.unwrap(), space_state);

                current_player = BoardSpaceState::Player(Player::O);
            } else {
                let current_move = self.player_o.determine_move(&self.board);
                let space_state = BoardSpaceState::Player(Player::O);

                self.board.set_by_index(current_move.unwrap(), space_state);

                current_player = BoardSpaceState::Player(Player::X);
            }

            winner = self.board.determine_winner();
            moves_available = self.board.moves_available();

            if let Some(res) = winner {
                match res {
                    BoardSpaceState::Player(Player::X) => {
                        self.player_x.learn(&self.board, true);
                        self.player_o.learn(&self.board, false);

                        return Some(String::from("X Wins!"));
                    }
                    BoardSpaceState::Player(Player::O) => {
                        self.player_x.learn(&self.board, false);
                        self.player_o.learn(&self.board, true);

                        return Some(String::from("O Wins!"));
                    }
                    BoardSpaceState::Empty => {}
                }
            } else if !moves_available {
                return Some(String::from("TIE!"));
            }
        }

        None
    }
}
