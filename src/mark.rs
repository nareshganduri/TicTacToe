#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mark {
    pub x: usize,
    pub y: usize,
}

impl From<usize> for Mark {
    fn from(index: usize) -> Self {
        let x = index / 3;
        let y = index % 3;

        Self { x, y }
    }
}