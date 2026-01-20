use crate::Direction;

#[derive(Clone)]
pub struct Car {
    pub dir: Direction,
    pub wait_time: f32,
}