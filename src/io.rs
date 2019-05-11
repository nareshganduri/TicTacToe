use crate::board::Board;
use crate::game::Marker;
use crate::mark::Mark;
use crate::player::Player;
use crate::square::Square;
use std::io::Write;

use console::{style, Alignment, Style, Term};
use rand::seq::SliceRandom;

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
    let style = Style::new().blue().on_black();
    let mut term = Term::stdout();
    let bar = "=========================================";
    let msg = console::pad_str(msg, bar.len() - 2, Alignment::Center, None);
    let bar = style.apply_to(bar);
    let pipe = style.apply_to("|");

    writeln!(term, "{}", bar).unwrap();
    writeln!(term, "{}{}{}", pipe, style.apply_to(msg), pipe).unwrap();
    writeln!(term, "{}", bar).unwrap();
}

pub fn clear_screen() {
    let term = Term::stdout();
    term.clear_screen().unwrap();
}

pub fn wait() {
    let mut term = Term::stdout();

    write!(term, "Press enter to continue...").unwrap();
    term.read_line().unwrap();
}

pub fn get_yes_no(msg: &str) -> bool {
    let msg = format!("{} [y/n]: ", msg);
    let mut input = get_prompt(&msg);

    loop {
        input.make_ascii_lowercase();

        if input == "yes" || input == "y" {
            return true;
        } else if input == "no" || input == "n" {
            return false;
        } else {
            input = get_prompt("Invalid input. Try again [y/n]: ");
        }
    }
}

fn get_prompt(msg: &str) -> String {
    let mut term = Term::stdout();

    write!(term, "{}", msg).unwrap();
    term.read_line().unwrap()
}

pub fn get_player_move(board: Board) -> Mark {
    let mut input = get_prompt("Enter a number [0-8]: ");

    loop {
        if let Ok(index) = input.parse::<usize>() {
            let mark = index.into();
            let Mark { x, y } = mark;

            if index >= 9 {
                input = get_prompt("Input must be between 0 and 8. Try again. [0-8]:");
            } else if board.get(x, y).is_empty() {
                return mark;
            } else {
                input = get_prompt("Spot is already taken. Try again. [0-8]: ");
            }
        } else {
            input = get_prompt("Invalid input. Try again. [0-8]: ");
        }
    }
}

pub fn get_comp_move(board: Board) -> Mark {
    let mut max_utility = std::isize::MIN;
    let mut marks = vec![];

    for i in 0..9 {
        let mark = i.into();
        let Mark { x, y } = mark;

        if board.get(x, y).is_empty() {
            let board = board.mark(x, y, Player::Computer);
            let utility = board.utility(Player::Computer);
            if utility > max_utility {
                max_utility = utility;
            }

            marks.push((mark, utility));
        } else {
            continue;
        }
    }

    marks.retain(|(_, utility)| *utility == max_utility);
    let max_marks: Vec<_> = marks.into_iter().map(|(mark, _)| mark).collect();
    let mut rng = rand::thread_rng();

    *max_marks.choose(&mut rng).unwrap()
}

pub fn render(board: Board, player_marker: Marker) {
    let mut term = Term::stdout();
    let (player_marker, comp_marker) = match player_marker {
        Marker::Cross => (X_MARKER, O_MARKER),
        Marker::Naught => (O_MARKER, X_MARKER),
    };

    term.write_line("+---------+---------+---------+").unwrap();

    for x in 0..3 {
        for line in 0..5 {
            for y in 0..3 {
                let spot = board.get(x, y);
                match spot {
                    Square::Human => {
                        let line = style(player_marker[line]).green();
                        write!(term, "|{}", line).unwrap()
                    }
                    Square::Computer => {
                        let line = style(comp_marker[line]).red();
                        write!(term, "|{}", line).unwrap()
                    }
                    Square::Empty => {
                        if line == 0 {
                            let index = x * 3 + y;
                            write!(term, "|{}        ", index).unwrap();
                        } else {
                            write!(term, "|         ").unwrap();
                        }
                    }
                }
            }
            term.write_line("|").unwrap();
        }
        term.write_line("+---------+---------+---------+").unwrap();
    }
}

pub fn get_marker() -> Marker {
    println!();
    for i in 0..5 {
        println!("       {}         {}", X_MARKER[i], O_MARKER[i]);
    }
    println!();

    let mut input = get_prompt("Choose marker [X/O]: ");
    input.make_ascii_lowercase();

    loop {
        if input == "x" {
            return Marker::Cross;
        } else if input == "o" {
            return Marker::Naught;
        } else {
            input = get_prompt("Invalid input. Try again. [X/O]: ");
        }
    }
}
