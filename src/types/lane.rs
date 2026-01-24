use crate::types::*;

use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Lane {
	pub id: usize,
	pub offset_from_center: f32,
	pub cars: Vec<Car>,
	pub length: f32,
	pub direction: Direction,
	pub width: f32,
	pub orientation: Orientation,
	pub lights: Vec<TrafficLight>,
	pub spawner: Option<Spawner>,
}
impl Lane {
	pub fn new(
		id: usize,
		offset: f32,
		length: f32,
		width: f32,
		direction: Direction,
		orientation: Orientation,
	) -> Self {
		Self {
			id,
			offset_from_center: offset,
			cars: vec![],
			length,
			direction,
			width,
			orientation,
			lights: vec![],
			spawner: None,
		}
	}

	pub fn update(&mut self, dt: f32) {
		let dir_sign = match self.direction {
			Direction::Forward => 1.0,
			Direction::Backward => -1.0,
		};

		self.cars.sort_by(|a, b| {
			let a_key = dir_sign * a.s;
			let b_key = dir_sign * b.s;
			b_key.partial_cmp(&a_key).unwrap_or(std::cmp::Ordering::Equal)
		});

		let len = self.cars.len();
		for i in 0..len {
			let (head, tail) = self.cars.split_at_mut(i);
			let leader = head.last();
			let car = &mut tail[0];
			car.update(dt, leader, &self.lights, self.direction);
		}

		self.cars.retain(|c| {
			let progress = dir_sign * c.s;
			progress > -1000.0 && progress < self.length + 1000.0
		});
	}

	pub fn world_pos(&self, s: f32) -> Vec2 {
		match self.orientation {
			Orientation::Horizontal => vec2(s, self.offset_from_center),
			Orientation::Vertical => vec2(self.offset_from_center, s),
		}
	}

	pub fn spawn_car(&mut self, car: Car) {
		self.cars.push(car);
	}
}