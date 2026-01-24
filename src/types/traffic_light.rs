use crate::types::LightState;

use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct TrafficLight {
	pub stop_s: f32,
	pub state: LightState,
	pub lane_id: usize,
	pub pos: Vec2,
}
impl TrafficLight {
	pub fn new(stop_s: f32, state: LightState, lane_id: usize, pos: Vec2) -> Self {
		Self {
			stop_s,
			state,
			lane_id,
			pos,
		}
	}

	pub fn is_red(&self) -> bool { self.state == LightState::Red }
}