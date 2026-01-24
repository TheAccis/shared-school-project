use macroquad::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
	Horizontal,
	Vertical,
}

pub trait Drawable {
	fn draw(&self, cam: &WorldCamera, resources: &Resources);
}

pub struct WorldCamera {
	cam: Camera2D,
	pixels_per_unit: f32,
}
impl WorldCamera {
	pub fn new(pixels_per_unit: f32) -> Self {
		let mut this = Self {
			cam: Camera2D::default(),
			pixels_per_unit,
		};
		this.recalc();
		this
	}

	pub fn recalc(&mut self) {
		self.cam.zoom = vec2(
			2.0 / (screen_width() / self.pixels_per_unit),
			-2.0 / (screen_height() / self.pixels_per_unit),
		);
	}

	pub fn set_target(&mut self, world_pos: Vec2) {
		self.cam.target = world_pos;
	}

	pub fn apply(&self) {
		set_camera(&self.cam);
	}

	pub fn world_screen_size(&self) -> Vec2 {
		vec2(
			screen_width() / self.pixels_per_unit,
			screen_height() / self.pixels_per_unit,
		)
	}
}

pub struct Resources {
	pub car_textures: Vec<Texture2D>,
	pub light_textures: HashMap<LightState, Texture2D>,
}
impl Resources {
	pub async fn load_from_dirs(cars_dir: &str, lights_dir: &str) -> Self {
		let mut car_textures: Vec<Texture2D> = Vec::new();

		if let Ok(entries) = fs::read_dir(cars_dir) {
			for entry in entries.flatten() {
				let p = entry.path();
				if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
					let ext = ext.to_lowercase();
					if ext == "webp" {
						if let Some(pstr) = p.to_str() {
							if let Ok(tex) = Self::load_webp_texture(pstr).await {
								tex.set_filter(FilterMode::Linear);
								car_textures.push(tex);
							} else {
								eprintln!("Failed to load car texture: {}", pstr);
							}
						}
					}
				}
			}
		} else {
			eprintln!("Warning: could not read cars dir: {}", cars_dir);
		}

		let mut light_textures: HashMap<LightState, Texture2D> = HashMap::new();
		let mapping = [
			(LightState::Red, "red.webp"),
			(LightState::Yellow, "yellow.webp"),
			(LightState::Green, "green.webp"),
		];
		for (state, fname) in mapping.iter() {
			let p = Path::new(lights_dir).join(fname);
			if p.exists() {
				if let Some(pstr) = p.to_str() {
					match load_texture(pstr).await {
						Ok(t) => {
							t.set_filter(FilterMode::Linear);
							light_textures.insert(*state, t);
						}
						Err(_) => {
							eprintln!("Failed to load light texture: {}", pstr);
						}
					}
				}
			} else {
				eprintln!("Light texture not found: {}", p.display());
			}
		}

		Self {
			car_textures,
			light_textures,
		}
	}

	pub fn random_car_texture_idx(&self) -> usize {
		if self.car_textures.is_empty() {
			0
		} else {
			let n = self.car_textures.len() as i32;
			let idx = macroquad::rand::gen_range(0, n);
			idx as usize
		}
	}

	async fn load_webp_texture(path: &str) -> Result<Texture2D, String> {
		let bytes = load_file(path)
			.await
			.map_err(|e| format!("Не удалось прочитать файл '{}': {:?}", path, e))?;

		let img = image::io::Reader::new(std::io::Cursor::new(bytes))
			.with_guessed_format()
			.map_err(|e| format!("Ошибка формата файла '{}': {}", path, e))?
			.decode()
			.map_err(|e| format!("Ошибка декодирования WebP '{}': {}", path, e))?
			.to_rgba8();

		let (width, height) = (img.width() as u16, img.height() as u16);

		let texture = Texture2D::from_rgba8(width, height, &img.into_raw());

		Ok(texture)
	}
}

pub struct Road {
	pos: Vec2,
	half_width: f32,
	orientation: Orientation,
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
impl Drawable for Road {
	fn draw(&self, cam: &WorldCamera, _resources: &Resources) {
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
	Forward,
	Backward,
}

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

	pub fn is_red(&self) -> bool {
		matches!(self.state, LightState::Red)
	}

	pub fn draw_for_lane(&self, _cam: &WorldCamera, _lane: &Lane, resources: &Resources) {
		if let Some(tex) = resources.light_textures.get(&self.state) {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LightState {
	Red,
	Green,
	Yellow,
}

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
			b_key.partial_cmp(&a_key).unwrap_or(Ordering::Equal)
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

	pub fn spawn_car_with_texture(&mut self, mut car: Car, texture_idx: usize) {
		car.texture_idx = texture_idx;
		self.cars.push(car);
	}
}
impl Drawable for Lane {
	fn draw(&self, cam: &WorldCamera, resources: &Resources) {
		for car in &self.cars {
			let car_pos = self.world_pos(car.s);
			let car_len = car.length;
			let car_w = car.width;

			if let Some(tex) = resources.car_textures.get(car.texture_idx) {
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
			light.draw_for_lane(cam, self, resources);
		}
	}
}

pub struct World {
	pub resources: Resources,

	pub lanes: Vec<Lane>,
	pub roads: Vec<Road>,
}
impl World {
	pub fn new(
		road_center: Vec2,
		road_width: f32,
		lane_length: f32,
		resources: Resources,
	) -> Self {
		let h_road = Road::new(road_center, road_width, Orientation::Horizontal);
		let v_road = Road::new(road_center, road_width, Orientation::Vertical);

		let mut lane_east = Lane::new(
			0,
			-road_width / 4.0,
			lane_length,
			road_width / 2.0,
			Direction::Forward,
			Orientation::Horizontal,
		);
		let mut lane_west = Lane::new(
			1,
			road_width / 4.0,
			lane_length,
			road_width / 2.0,
			Direction::Backward,
			Orientation::Horizontal,
		);
		let mut lane_north = Lane::new(
			2,
			road_width / 4.0,
			lane_length,
			road_width / 2.0,
			Direction::Forward,
			Orientation::Vertical,
		);
		let mut lane_south = Lane::new(
			3,
			-road_width / 4.0,
			lane_length,
			road_width / 2.0,
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
			lane_east.id,
			compute_light_pos(&lane_east),
		));
		lane_west.lights.push(TrafficLight::new(
			0.0,
			LightState::Red,
			lane_west.id,
			compute_light_pos(&lane_west),
		));
		lane_north.lights.push(TrafficLight::new(
			0.0,
			LightState::Green,
			lane_north.id,
			compute_light_pos(&lane_north),
		));
		lane_south.lights.push(TrafficLight::new(
			0.0,
			LightState::Red,
			lane_south.id,
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
			resources,
		}
	}

	pub fn update(&mut self, dt: f32) {
		for lane in &mut self.lanes {
			if let Some(sp) = &lane.spawner {
				if let Some(car) = sp.try_spawn(lane) {
					let tex_idx = self.resources.random_car_texture_idx();
					lane.spawn_car_with_texture(car, tex_idx);
				}
			}
		}

		for lane in &mut self.lanes {
			lane.update(dt);
		}
	}
}
impl Drawable for World {
	fn draw(&self, cam: &WorldCamera, resources: &Resources) {
		for road in &self.roads {
			road.draw(cam, resources);
		}
		for lane in &self.lanes {
			lane.draw(cam, resources);
		}
	}
}

#[macroquad::main("Traffic Intersection with Textures")]
async fn main() {
	let cars_dir = "res/cars";
	let lights_dir = "res/traffic-lights";

	let resources = Resources::load(cars_dir, lights_dir).await;

	let mut camera = WorldCamera::new(10.0);
	let world_center = vec2(0.0, 0.0);

	let mut world = World::new(world_center, 10.0, 200.0, resources);

	const FIXED_DT: f32 = 1.0 / 60.0;
	let mut accumulator: f32 = 0.0;

	loop {
		let dt = get_frame_time();
		accumulator += dt;

		while accumulator >= FIXED_DT {
			world.update(FIXED_DT);
			accumulator -= FIXED_DT;
		}

		clear_background(LIGHTGRAY);
		camera.recalc();
		camera.set_target(world_center);
		camera.apply();

		world.draw(&camera, &world.resources);

		set_default_camera();
		draw_fps();
		next_frame().await;
	}
}