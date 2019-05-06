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

    io::clear_screen();
    io::print_header("Tic-Tac-Toe");
    println!("Welcome to Tic Tac Toe!\n\n\n");
    io::wait();

    loop {
        io::clear_screen();
        io::print_header("Marker select");
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
            player_first = !player_first;
        } else {
            io::clear_screen();
            io::print_header("Exit");
            println!("Goodbye.");
            io::wait();
            return;
        }
    }
}
