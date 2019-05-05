use crate::board::Board;
use crate::board::Winner;
use crate::io::{clear_screen, get_comp_move, get_player_move, print_header, render, wait};
use crate::mark::Mark;
use crate::player::Player;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Marker {
    Naught,
    Cross,
}

pub struct Game {
    board: Board,
    curr_player: Player,
    player_marker: Marker,
}

impl Game {
    pub fn new(player_first: bool, player_marker: Marker) -> Self {
        let first = if player_first {
            Player::Human
        } else {
            Player::Computer
        };

        Self {
            board: Board::new(),
            curr_player: first,
            player_marker,
        }
    }

    pub fn run(&mut self) -> Winner {
        loop {
            clear_screen();

            if let Some(winner) = self.board.get_winner() {
                print_header("GAME OVER");
                render(self.board, self.player_marker);

                return winner;
            } else {
                match self.curr_player {
                    Player::Human => {
                        print_header("Your Turn");
                        render(self.board, self.player_marker);

                        let Mark { x, y } = get_player_move(self.board);
                        self.board = self.board.mark(x, y, Player::Human);
                    }
                    Player::Computer => {
                        print_header("Computer's Turn");
                        render(self.board, self.player_marker);

                        let Mark { x, y } = get_comp_move(self.board);
                        self.board = self.board.mark(x, y, Player::Computer);
                        wait();
                    }
                }

                self.curr_player = self.curr_player.opponent();
            }
        }
    }
}
