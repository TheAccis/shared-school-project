use std::collections::VecDeque;
use crate::{Car, Direction};

pub struct Road {
    pub dir: Direction,
    pub queue: VecDeque<Car>,
}

impl Road {
    pub fn new(dir: Direction) -> Self {
        Self { dir, queue: VecDeque::new() }
    }
}