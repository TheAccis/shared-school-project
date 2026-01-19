use crate::Direction;
use glam::Vec2;

pub struct Car {
    pos: Vec2,
    velocity: Vec2,
    dir: Direction,
}

impl Car {
    pub const SPAWN_MARGIN: f32 = 50.0;
    pub const WIDTH: f32 = 10.0;
    pub const HEIGHT: f32 = 20.0;

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
        const VERTICAL: Vec2 = Vec2::new(Car::WIDTH, Car::HEIGHT);
        const HORIZONTAL: Vec2 = Vec2::new(Car::HEIGHT, Car::WIDTH);

        match self.dir {
            Direction::North | Direction::South => VERTICAL,
            Direction::West | Direction::East => HORIZONTAL,
        }
    }

    pub fn is_outside(&self, screen_size: Vec2) -> bool {
        let min = Vec2::splat(-Self::SPAWN_MARGIN);
        let max = screen_size + Self::SPAWN_MARGIN;

        self.pos.cmpgt(max).any() || self.pos.cmplt(min).any()
    }
}