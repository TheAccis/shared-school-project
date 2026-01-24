use crate::io::Assets;
use crate::types::*;

use macroquad::prelude::*;

pub struct World {
	pub assets: Assets,

	pub lanes: Vec<Lane>,
	pub roads: Vec<Road>,
}
impl World {
	pub fn new(
		road_center: Vec2,
		road_width: f32,
		lane_length: f32,
		assets: Assets,
	) -> Self {
		let h_road = Road::new(road_center, road_width, Orientation::Horizontal);
		let v_road = Road::new(road_center, road_width, Orientation::Vertical);

		let mut lane_east = Lane::new(
			-road_width / 4.0,
			lane_length,
			Direction::Forward,
			Orientation::Horizontal,
		);
		let mut lane_west = Lane::new(
			road_width / 4.0,
			lane_length,
			Direction::Backward,
			Orientation::Horizontal,
		);
		let mut lane_north = Lane::new(
			road_width / 4.0,
			lane_length,
			Direction::Forward,
			Orientation::Vertical,
		);
		let mut lane_south = Lane::new(
			-road_width / 4.0,
			lane_length,
			Direction::Backward,
			Orientation::Vertical,
		);

		let half = road_width * 0.5;
		let corner_margin = 1.5;

		let nw = vec2(-half - corner_margin, half + corner_margin);
		let ne = vec2(half + corner_margin, half + corner_margin);
		let sw = vec2(-half - corner_margin, -half - corner_margin);
		let se = vec2(half + corner_margin, -half - corner_margin);

		let compute_light_pos = |lane: &Lane| -> Vec2 {
			match lane.orientation {
				Orientation::Horizontal => match lane.direction {
					Direction::Forward => {
						if lane.offset_from_center < 0.0 {
							sw
						} else {
							nw
						}
					}
					Direction::Backward => {
						if lane.offset_from_center < 0.0 {
							se
						} else {
							ne
						}
					}
				},
				Orientation::Vertical => match lane.direction {
					Direction::Forward => {
						if lane.offset_from_center < 0.0 {
							sw
						} else {
							se
						}
					}
					Direction::Backward => {
						if lane.offset_from_center < 0.0 {
							nw
						} else {
							ne
						}
					}
				},
			}
		};

		lane_east.lights.push(TrafficLight::new(
			0.0,
			LightState::Red,
			compute_light_pos(&lane_east),
		));
		lane_west.lights.push(TrafficLight::new(
			0.0,
			LightState::Red,
			compute_light_pos(&lane_west),
		));
		lane_north.lights.push(TrafficLight::new(
			0.0,
			LightState::Green,
			compute_light_pos(&lane_north),
		));
		lane_south.lights.push(TrafficLight::new(
			0.0,
			LightState::Red,
			compute_light_pos(&lane_south),
		));

		lane_east.spawner = Some(Spawner {
			spawn_s: -60.0,
			safe_distance: 12.0,
			desired_speed: 8.0,
		}); // east (x+)
		lane_west.spawner = Some(Spawner {
			spawn_s: 60.0,
			safe_distance: 12.0,
			desired_speed: 8.0,
		}); // west (x-)
		lane_north.spawner = Some(Spawner {
			spawn_s: -60.0,
			safe_distance: 12.0,
			desired_speed: 6.0,
		}); // north (y+)
		lane_south.spawner = Some(Spawner {
			spawn_s: 60.0,
			safe_distance: 12.0,
			desired_speed: 6.0,
		}); // south (y-)

		let lanes = vec![lane_east, lane_west, lane_north, lane_south];

		Self {
			lanes,
			roads: vec![h_road, v_road],
			assets,
		}
	}

	pub fn update(&mut self, dt: f32) {
		for lane in &mut self.lanes {
			if let Some(sp) = &lane.spawner {
				if let Some(mut car) = sp.try_spawn(lane) {
					car.texture_idx = self.assets.random_car_texture_idx();
					lane.spawn_car(car);
				}
			}
		}

		for lane in &mut self.lanes {
			lane.update(dt);
		}
	}
}