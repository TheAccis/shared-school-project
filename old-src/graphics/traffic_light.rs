use macroquad::prelude::*;

use crate::graphics::{Drawable, WorldCamera};
use crate::io::Assets;
use crate::types::{LightState, TrafficLight};

impl Drawable for TrafficLight {
	fn draw(&self, _cam: &WorldCamera, assets: &Assets) {
		if let Some(tex) = assets.light_textures.get(&self.state) {
			const LIGHT_TEXTURE_WORLD_SIZE: f32 = 2.0;
			let dest_size = Some(vec2(LIGHT_TEXTURE_WORLD_SIZE, LIGHT_TEXTURE_WORLD_SIZE));

			let draw_pos = vec2(
				self.pos.x - LIGHT_TEXTURE_WORLD_SIZE / 2.0,
				self.pos.y - LIGHT_TEXTURE_WORLD_SIZE / 2.0,
			);

			draw_texture_ex(
				tex,
				draw_pos.x,
				draw_pos.y,
				WHITE,
				DrawTextureParams {
					dest_size,
					rotation: 0.0,
					flip_x: false,
					flip_y: false,
					source: None,
					pivot: None,
				},
			);
		} else {
			let color = match self.state {
				LightState::Red => RED,
				LightState::Green => GREEN,
				LightState::Yellow => YELLOW,
			};
			draw_circle(self.pos.x, self.pos.y, 1.0, color);
		}
	}
}