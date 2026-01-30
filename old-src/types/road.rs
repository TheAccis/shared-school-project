use crate::types::Orientation;
use macroquad::math::Vec2;

pub struct Road {
	pub(crate) pos: Vec2,
	pub(crate) half_width: f32,
	pub(crate) orientation: Orientation,
}
impl Road {
	pub fn new(pos: Vec2, width: f32, orientation: Orientation) -> Self {
		Self {
			pos,
			half_width: width * 0.5,
			orientation,
		}
	}
}