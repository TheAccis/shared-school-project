use glam::Vec2;

pub struct TrafficLight {
	pub green_duration: f32,
	pub green: bool,
	pub timer: f32,
	pub pos: Vec2,
}

impl TrafficLight {
	pub fn update(&mut self, dt: f32) {
		self.timer += dt;
		if self.timer >= self.green_duration {
			self.green = !self.green;
			self.timer = 0.0;
		}
	}
}