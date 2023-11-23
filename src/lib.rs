use crate::board::Winner;
use crate::game::Game;
use crate::player::Player;

mod board;
mod game;
mod io;
mod mark;
mod player;
mod square;

pub fn run_game() {
    let mut first_player = Player::Human;

    io::clear_screen();
    io::print_header("Tic-Tac-Toe");
    println!("Welcome to Tic Tac Toe!\n\n\n");
    io::wait();

    loop {
        io::clear_screen();
        io::print_header("Marker select");
        let player_marker = io::get_marker();
        let mut game = Game::new(first_player, player_marker);

        let winner = game.run();
        match winner {
            Winner::Tie => println!("There was a tie."),
            Winner::Human => println!("You won!"),
            Winner::Computer => println!("The computer won."),
        }

        let play_again = io::get_yes_no("Play again?");
        if play_again {
            first_player = first_player.opponent();
        } else {
            io::clear_screen();
            io::print_header("Exit");
            println!("Goodbye.");
            io::wait();
            return;
        }
    }
}
