use crate::graphics::{WorldCamera, Drawable};
use crate::types::{Orientation, Direction, Lane};
use crate::io::Assets;

use macroquad::prelude::*;

impl Drawable for Lane {
	fn draw(&self, cam: &WorldCamera, assets: &Assets) {
		for car in &self.cars {
			let car_pos = self.world_pos(car.s);
			let car_len = car.length;
			let car_w = car.width;

			if let Some(tex) = assets.car_textures.get(car.texture_idx) {
				const SCALE: f32 = 5.0;

				let dest_w = car_len * (SCALE / 2.0);
				let dest_h = car_w * SCALE;

				let rotation = match (self.orientation, self.direction) {
					(Orientation::Horizontal, Direction::Forward) => std::f32::consts::FRAC_PI_2, // 90
					(Orientation::Horizontal, Direction::Backward) => -std::f32::consts::FRAC_PI_2, // -90
					(Orientation::Vertical, Direction::Forward) => std::f32::consts::PI, // 180
					(Orientation::Vertical, Direction::Backward) => 0.0,                 // 0
				};

				let draw_pos = vec2(car_pos.x - dest_w / 2.0, car_pos.y - dest_h / 2.0);

				draw_texture_ex(
					tex,
					draw_pos.x,
					draw_pos.y,
					WHITE,
					DrawTextureParams {
						dest_size: Some(vec2(dest_w, dest_h)),
						rotation,
						flip_x: false,
						flip_y: false,
						source: None,
						pivot: None,
					},
				);
			} else {
				if matches!(self.orientation, Orientation::Horizontal) {
					draw_rectangle(
						car_pos.x - car_len / 2.0,
						car_pos.y - car_w / 2.0,
						car_len,
						car_w,
						DARKBLUE,
					);
				} else {
					draw_rectangle(
						car_pos.x - car_w / 2.0,
						car_pos.y - car_len / 2.0,
						car_w,
						car_len,
						DARKBLUE,
					);
				}
			}
		}

		for light in &self.lights {
			light.draw(cam, assets);
		}
	}
}