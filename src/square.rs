#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Square {
    Empty,
    Human,
    Computer,
}

impl Square {
    pub fn is_empty(self) -> bool {
        self == Square::Empty
    }
}