use crate::types::enums::Orientation;
use crate::graphics::WorldCamera;
use crate::graphics::Drawable;
use crate::types::Road;
use crate::io::Assets;

use macroquad::prelude::*;

impl Drawable for Road {
	fn draw(&self, cam: &WorldCamera, _assets: &Assets) {
		let half_screen = cam.world_screen_size() / 2.0;

		match self.orientation {
			Orientation::Horizontal => {
				draw_rectangle(
					self.pos.x - half_screen.x,
					self.pos.y - self.half_width,
					half_screen.x * 2.0,
					self.half_width * 2.0,
					DARKGRAY,
				);
			}
			Orientation::Vertical => {
				draw_rectangle(
					self.pos.x - self.half_width,
					self.pos.y - half_screen.y,
					self.half_width * 2.0,
					half_screen.y * 2.0,
					DARKGRAY,
				);
			}
		}
	}
}