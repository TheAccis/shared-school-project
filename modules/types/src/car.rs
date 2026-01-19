use crate::Direction;
use glam::Vec2;

pub struct Car {
    pos: Vec2,
    velocity: Vec2,
    dir: Direction,
}

impl Car {
    pub fn new(pos: Vec2, dir: Direction, speed: f32) -> Self {
        let velocity = match dir {
            Direction::North => Vec2::new(0.0, -speed),
            Direction::South => Vec2::new(0.0, speed),
            Direction::West  => Vec2::new(-speed, 0.0),
            Direction::East  => Vec2::new(speed, 0.0),
        };

        Self { pos, velocity, dir }
    }

    #[inline]
    pub fn update(&mut self) {
        self.pos += self.velocity;
    }

    pub fn size(&self) -> Vec2 {
        const VERTICAL: Vec2 = Vec2::new(10.0, 20.0);
        const HORIZONTAL: Vec2 = Vec2::new(20.0, 10.0);

        match self.dir {
            Direction::North | Direction::South => VERTICAL,
            Direction::West | Direction::East => HORIZONTAL,
        }
    }

    pub fn is_outside(&self, screen_size: Vec2) -> bool {
        let min = Vec2::splat(-50.0);
        let max = screen_size + 50.0;

        self.pos.cmpgt(max).any() || self.pos.cmplt(min).any()
    }
}