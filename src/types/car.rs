use crate::types::{TrafficLight, Direction};

use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Car {
	pub s: f32,
	pub speed: f32,
	pub desired_speed: f32,
	pub length: f32,
	pub width: f32,
	pub texture_idx: usize,
}
impl Car {
	pub fn new(desired_speed: f32, texture_idx: usize) -> Self {
		Self {
			s: 0.0,
			speed: 0.0,
			desired_speed,
			length: 4.0,
			width: 1.8,
			texture_idx,
		}
	}

	pub fn update(
		&mut self,
		dt: f32,
		leader: Option<&Car>,
		lights: &[TrafficLight],
		dir: Direction,
	) {
		let dir_sign = match dir {
			Direction::Forward => 1.0,
			Direction::Backward => -1.0,
		};

		let mut obstacle_s: Option<f32> = leader.map(|l| l.s);

		for light in lights.iter().filter(|l| l.is_red()) {
			let ahead = dir_sign * (light.stop_s - self.s);
			if ahead > 0.0 {
				match obstacle_s {
					Some(obs_s) => {
						let obs_dist = dir_sign * (obs_s - self.s);
						if ahead < obs_dist {
							obstacle_s = Some(light.stop_s);
						}
					}
					None => obstacle_s = Some(light.stop_s),
				}
			}
		}

		let safe_gap = 6.0;
		let gap = if let Some(obs_s) = obstacle_s {
			dir_sign * (obs_s - self.s) - self.length
		} else {
			f32::INFINITY
		};

		if gap < safe_gap {
			self.speed = 0.0;
		} else {
			self.speed = self.desired_speed;
		}

		self.s += dir_sign * self.speed * dt;
	}
}