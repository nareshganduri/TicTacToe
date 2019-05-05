use crate::player::Player;
use crate::square::Square;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Winner {
    Human,
    Computer,
    Tie,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Board {
    tiles: [Square; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            tiles: [Square::Empty; 9],
        }
    }

    fn is_over(self) -> bool {
        for x in 0..3 {
            for y in 0..3 {
                if self.get(x, y).is_empty() {
                    return false;
                }
            }
        }

        true
    }

    fn check(self, xs: [usize; 3], ys: [usize; 3]) -> Option<Winner> {
        let s1 = self.get(xs[0], ys[0]);
        let s2 = self.get(xs[1], ys[1]);
        let s3 = self.get(xs[2], ys[2]);

        match (s1, s2, s3) {
            (Square::Human, Square::Human, Square::Human) => Some(Winner::Human),
            (Square::Computer, Square::Computer, Square::Computer) => Some(Winner::Computer),
            _ => None,
        }
    }

    pub fn get_winner(self) -> Option<Winner> {
        for x in 0..3 {
            if let Some(winner) = self.check([0, 1, 2], [x, x, x]) {
                return Some(winner);
            }

            if let Some(winner) = self.check([x, x, x], [0, 1, 2]) {
                return Some(winner);
            }
        }

        if let Some(winner) = self.check([0, 1, 2], [0, 1, 2]) {
            return Some(winner);
        }

        if let Some(winner) = self.check([2, 1, 0], [0, 1, 2]) {
            return Some(winner);
        }

        if self.is_over() {
            Some(Winner::Tie)
        } else {
            None
        }
    }

    pub fn get(self, x: usize, y: usize) -> Square {
        let index = x * 3 + y;
        self.tiles[index]
    }

    pub fn mark(self, x: usize, y: usize, player: Player) -> Self {
        let mut tiles = self.tiles;
        let index = x * 3 + y;

        let marker = player.marker();
        tiles[index] = marker;

        Self { tiles }
    }

    pub fn moves(self, player: Player) -> Moves {
        Moves {
            board: self,
            player,
            index: 0,
        }
    }

    pub fn utility(self, player: Player) -> isize {
        if let Some(winner) = self.get_winner() {
            match (winner, player) {
                (Winner::Tie, _) => return 0,
                (Winner::Human, Player::Human) => return 1,
                (Winner::Human, Player::Computer) => return -1,
                (Winner::Computer, Player::Computer) => return 1,
                (Winner::Computer, Player::Human) => return -1,
            }
        } else {
            let mut utility = std::isize::MIN;
            let opponent = player.opponent();

            for board in self.moves(opponent) {
                let curr = board.utility(opponent);
                if curr > utility {
                    utility = curr;
                }
            }

            -utility
        }
    }
}

pub struct Moves {
    board: Board,
    player: Player,
    index: usize,
}

impl Iterator for Moves {
    type Item = Board;

    fn next(&mut self) -> Option<Board> {
        while self.index < 9 {
            let x = self.index / 3;
            let y = self.index % 3;

            if !self.board.get(x, y).is_empty() {
                self.index += 1;
                continue;
            } else {
                let board = self.board.mark(x, y, self.player);
                self.index += 1;
                return Some(board);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::Square;

    #[test]
    fn test_winner() {
        let board = Board {
            tiles: [
                Square::Computer, Square::Empty, Square::Human,
                Square::Empty, Square::Human, Square::Empty,
                Square::Computer, Square::Empty, Square::Empty,
            ]
        };

        assert_eq!(board.get_winner(), None);
    }
}