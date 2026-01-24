use macroquad::prelude::*;

pub struct WorldCamera {
	cam: Camera2D,
	pixels_per_unit: f32,
}
impl WorldCamera {
	pub fn new(pixels_per_unit: f32) -> Self {
		let mut this = Self {
			cam: Camera2D::default(),
			pixels_per_unit,
		};
		this.recalc();
		this
	}

	pub fn recalc(&mut self) {
		self.cam.zoom = vec2(
			2.0 / (screen_width() / self.pixels_per_unit),
			-2.0 / (screen_height() / self.pixels_per_unit),
		);
	}

	pub fn set_target(&mut self, world_pos: Vec2) {
		self.cam.target = world_pos;
	}

	pub fn apply(&self) {
		set_camera(&self.cam);
	}

	pub fn world_screen_size(&self) -> Vec2 {
		vec2(
			screen_width() / self.pixels_per_unit,
			screen_height() / self.pixels_per_unit,
		)
	}
}