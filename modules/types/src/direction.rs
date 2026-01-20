#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn all() -> [Direction; 4] {
        [Self::North, Self::South, Self::West, Self::East]
    }
}