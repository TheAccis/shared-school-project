use crate::types::{Lane, Car};

#[derive(Clone, Debug)]
pub struct Spawner {
	pub spawn_s: f32,
	pub safe_distance: f32,
	pub desired_speed: f32,
}
impl Spawner {
	pub fn try_spawn(&self, lane: &Lane) -> Option<Car> {
		for c in &lane.cars {
			if (c.s - self.spawn_s).abs() < self.safe_distance {
				return None;
			}
		}

		let mut car = Car::new(self.desired_speed, 0);
		car.s = self.spawn_s;
		Some(car)
	}
}