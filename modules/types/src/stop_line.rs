use crate::Direction;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StopLine {
    pub dir: Direction,
    pub pos: f32,
}