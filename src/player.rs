use crate::square::Square;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Player {
    Human,
    Computer,
}

impl Player {
    pub fn marker(self) -> Square {
        match self {
            Player::Human => Square::Human,
            Player::Computer => Square::Computer,
        }
    }

    pub fn opponent(self) -> Self {
        match self {
            Player::Human => Player::Computer,
            Player::Computer => Player::Human,
        }
    }
}
