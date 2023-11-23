use crate::board::Board;
use crate::board::Winner;
use crate::io;
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
    pub fn new(first_player: Player, player_marker: Marker) -> Self {
        Self {
            board: Board::new(),
            curr_player: first_player,
            player_marker,
        }
    }

    pub fn run(&mut self) -> Winner {
        loop {
            io::clear_screen();

            if let Some(winner) = self.board.get_winner() {
                io::print_header("GAME OVER");
                io::render(self.board, self.player_marker);

                return winner;
            } else {
                match self.curr_player {
                    Player::Human => {
                        io::print_header("Your Turn");
                        io::render(self.board, self.player_marker);

                        let Mark { x, y } = io::get_player_move(self.board);
                        self.board = self.board.mark(x, y, Player::Human);
                    }
                    Player::Computer => {
                        io::print_header("Computer's Turn");
                        io::render(self.board, self.player_marker);

                        let Mark { x, y } = io::get_comp_move(self.board);
                        self.board = self.board.mark(x, y, Player::Computer);
                        io::wait();
                    }
                }

                self.curr_player = self.curr_player.opponent();
            }
        }
    }
}
