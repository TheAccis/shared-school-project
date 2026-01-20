use std::collections::VecDeque;
use crate::{Car, Direction};

pub struct Road {
    pub dir: Direction,
    queue: VecDeque<Car>,
}

impl Road {
    fn new(dir: Direction) -> Self {
        Self { dir, queue: VecDeque::new() }
    }
}