mod graphics;
mod types;
mod io;

use crate::{graphics::*, types::*, io::*};

use macroquad::prelude::*;

#[macroquad::main("Traffic Intersection with Textures")]
async fn main() {
	let cars_dir = "res/cars";
	let lights_dir = "res/traffic-lights";

	let assets = Assets::load(cars_dir, lights_dir).await;

	let mut camera = WorldCamera::new(10.0);
	let world_center = vec2(0.0, 0.0);

	let mut world = World::new(world_center, 10.0, 200.0, assets);

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

		world.draw(&camera, &world.assets);

		set_default_camera();
		draw_fps();
		next_frame().await;
	}
}