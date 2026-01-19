use crate::Direction;

pub struct TrafficLight {
	pub green_duration: f32,
    pub dir: Direction,
	pub green: bool,
	pub timer: f32,
}

impl TrafficLight {
    pub fn new(dir: Direction, green_duration: f32) -> Self {
        Self {
            dir,
            green: false,
            timer: 0.0,
            green_duration,
        }
    }

	pub fn update(&mut self, dt: f32) {
		self.timer += dt;
		if self.timer >= self.green_duration {
			self.green = !self.green;
			self.timer = 0.0;
		}
	}
}