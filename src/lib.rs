use crate::board::Winner;
use crate::game::Game;

mod board;
mod game;
mod io;
mod mark;
mod player;
mod square;

pub fn run_game() {
    let mut player_first = true;

    println!("Welcome to Tic Tac Toe!\n\n\n");

    loop {
        let player_marker = io::get_marker();
        let mut game = Game::new(player_first, player_marker);

        let winner = game.run();
        match winner {
            Winner::Tie => println!("There was a tie."),
            Winner::Human => println!("You won!"),
            Winner::Computer => println!("The computer won."),
        }

        let play_again = io::get_yes_no("Play again?");
        if play_again {
            io::clear_screen();
            player_first = !player_first;
        } else {
            println!("Goodbye.");
            io::wait();
            return;
        }
    }
}
