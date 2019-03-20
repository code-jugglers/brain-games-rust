mod board;
mod bot;

use board::{Board, BoardSpace, GameResult};
use bot::Bot;

fn main() {
    let mut board = Board::new();
    let mut bot_x = Bot::new(BoardSpace::X);
    let mut bot_o = Bot::new(BoardSpace::O);
    let mut wins_x = 0;
    let mut wins_o = 0;
    let mut draws = 0;

    for i in 0..10000000 {
        board.reset();

        let res = play(&mut board, &mut bot_x, &mut bot_o);

        if res == GameResult::XWin {
            wins_x = wins_x + 1;
        } else if res == GameResult::OWin {
            wins_o = wins_o + 1;
        } else {
            draws = draws + 1;
        }

        bot_x.learn(&board);
        bot_o.learn(&board);

        if i % 10000 == 0 {
            println!("{}", board);
            println!("=====================");
            println!("{}", res);
            println!("=====================");
            println!("X Wins: {}", wins_x);
            println!("O Wins: {}", wins_o);
            println!("Draws: {}", draws);
        }
    }
}

fn play(board: &mut Board, bot_x: &mut Bot, bot_o: &mut Bot) -> GameResult {
    bot_x.make_move(board);

    let x_res = board.check_board();

    if x_res == GameResult::Undecided {
        bot_o.make_move(board);

        let o_res = board.check_board();

        if o_res == GameResult::Undecided {
            return play(board, bot_x, bot_o);
        }

        return o_res;
    }

    x_res
}
