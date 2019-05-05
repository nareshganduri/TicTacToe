use crate::board::Board;
use crate::game::Marker;
use crate::mark::Mark;
use crate::player::Player;
use crate::square::Square;
use std::io::{self, Write};

static X_MARKER: [&str; 5] = [
    "##     ##",
    "  ## ##  ",
    "   ###   ",
    "  ## ##  ",
    "##     ##",
];

static O_MARKER: [&str; 5] = [
    "  #####  ",
    " ##   ## ",
    "##     ##",
    " ##   ## ",
    "  #####  ",
];

pub fn print_header(msg: &str) {
    println!("{}", msg);
    println!("=========================================");
}

pub fn clear_screen() {
    for _ in 0..150 {
        println!();
    }
}

pub fn wait() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    print!("Press enter to continue...");
    stdout.flush().unwrap();
    stdin.read_line(&mut input).unwrap();
}

pub fn get_yes_no(msg: &str) -> bool {
    let mut input = String::new();

    let msg = format!("{} [y/n]: ", msg);
    get_prompt(&msg, &mut input);

    loop {
        input.make_ascii_lowercase();

        if input == "yes" || input == "y" {
            return true;
        } else if input == "no" || input == "n" {
            return false;
        } else {
            get_prompt("Invalid input. Try again [y/n]: ", &mut input);
        }
    }
}

fn get_prompt(msg: &str, input: &mut String) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    input.clear();

    print!("{}", msg);
    stdout.flush().unwrap();
    stdin.read_line(input).unwrap();
    input.retain(|c| !c.is_whitespace());
}

pub fn get_player_move(board: Board) -> Mark {
    let mut input = String::new();

    get_prompt("Enter a number [0-8]: ", &mut input);

    loop {
        if let Ok(index) = input.parse::<usize>() {
            let mark = index.into();
            let Mark { x, y } = mark;

            if index >= 9 {
                get_prompt(
                    "Input must be between 0 and 8. Try again. [0-8]:",
                    &mut input,
                );
            } else if board.get(x, y).is_empty() {
                return mark;
            } else {
                get_prompt("Spot is already taken. Try again. [0-8]: ", &mut input);
            }
        } else {
            get_prompt("Invalid input. Try again. [0-8]: ", &mut input);
        }
    }
}

pub fn get_comp_move(board: Board) -> Mark {
    let mut max = std::isize::MIN;
    let mut max_mark = 0.into();

    for i in 0..9 {
        let mark = i.into();
        let Mark { x, y } = mark;

        if board.get(x, y).is_empty() {
            let board = board.mark(x, y, Player::Computer);
            let utility = board.utility(Player::Computer);
            if utility > max {
                max = utility;
                max_mark = mark;
            }
        } else {
            continue;
        }
    }

    max_mark
}

pub fn render(board: Board, player_marker: Marker) {
    let (player_marker, comp_marker) = match player_marker {
        Marker::Cross => (X_MARKER, O_MARKER),
        Marker::Naught => (O_MARKER, X_MARKER),
    };

    println!("+---------+---------+---------+");

    for x in 0..3 {
        for line in 0..5 {
            for y in 0..3 {
                let spot = board.get(x, y);
                match spot {
                    Square::Human => print!("|{}", player_marker[line]),
                    Square::Computer => print!("|{}", comp_marker[line]),
                    Square::Empty => {
                        if line == 0 {
                            let index = x * 3 + y;
                            print!("|{}        ", index);
                        } else {
                            print!("|         ");
                        }
                    }
                }
            }
            println!("|");
        }
        println!("+---------+---------+---------+");
    }
}

pub fn get_marker() -> Marker {
    let mut input = String::new();

    get_prompt("Choose marker [X/O]: ", &mut input);
    input.make_ascii_lowercase();

    loop {
        if input == "x" {
            return Marker::Cross;
        } else if input == "o" {
            return Marker::Naught;
        } else {
            get_prompt("Invalid input. Try again. [X/O]: ", &mut input);
        }
    }
}
