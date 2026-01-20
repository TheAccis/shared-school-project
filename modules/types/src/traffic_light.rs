use crate::Direction;

pub struct TrafficLight {
    pub dir: Direction,
    pub green: bool,
    pub timer: f32,
}

impl TrafficLight {
	pub const GREEN_TIME: f32 = 5.0; // seconds

    pub fn new(dir: Direction) -> Self {
        Self {
            dir,
            green: false,
            timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.timer += dt;
        if self.timer >= Self::GREEN_TIME {
            self.green = !self.green;
            self.timer = 0.0;
        }
    }
}