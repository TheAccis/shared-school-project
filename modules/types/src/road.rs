use std::collections::VecDeque;
use crate::Car;

struct Road {
    queue: VecDeque<Car>,
}

impl Road {
    fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}