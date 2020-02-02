mod board;
mod board_space;
mod bot;

use board::Board;
use board_space::BoardSpace;
use bot::Bot;

fn main() {
    let player_1 = Bot::new(String::from("0"), BoardSpace::X);
    let player_2 = Bot::new(String::from("1"), BoardSpace::O);

    for _ in 0..1000 {
        let mut board = Board::new();

        play(&mut board, &player_1, &player_2);

        println!("{}", board);
        println!("The Winnder Is: {} \n", board.determine_winner());
    }

    fn play(board: &mut Board, player_1: &Bot, player_2: &Bot) {
        let mut current_player: &Bot = player_1;

        while board.determine_winner() == BoardSpace::Empty {
            if let Some(m) = current_player.determine_move(board.spaces) {
                if board.make_move(current_player.space, m[0], m[1]) {
                    current_player = if current_player.id == player_1.id {
                        player_2
                    } else {
                        player_1
                    }
                }
            }
        }
    }
}
