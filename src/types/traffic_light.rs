use crate::types::LightState;

use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct TrafficLight {
	pub stop_s: f32,
	pub state: LightState,
	pub pos: Vec2,
}
impl TrafficLight {
	pub fn new(stop_s: f32, state: LightState, pos: Vec2) -> Self {
		Self {
			stop_s,
			state,
			pos,
		}
	}

	pub fn is_red(&self) -> bool { self.state == LightState::Red }
}