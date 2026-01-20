use glam::Vec2;
use graphics::{CarView, TrafficLightView};
use macroquad::prelude::*;
use types::*;

const LINE_THICKNESS: f32 = 50.0;

#[macroquad::main("Traffic Intersection Demo")]
async fn main() {
	let font = load_ttf_font("res/Inter_18pt-Medium.ttf").await.unwrap();

	let mut car_views: Vec<CarView> = vec![];
	let spawn_prob = 2; // %
	let speed: f32 = 2.0;

	let screen_size = get_screen_size();
	let stop_lines = [
		StopLine {
			dir: Direction::North,
			pos: screen_size.y / 2.0 + 20.0,
		},
		StopLine {
			dir: Direction::South,
			pos: screen_size.y / 2.0 - 20.0,
		},
		StopLine {
			dir: Direction::West,
			pos: screen_size.x / 2.0 + 20.0,
		},
		StopLine {
			dir: Direction::East,
			pos: screen_size.x / 2.0 - 20.0,
		},
	];

	let lights = vec![
		TrafficLight::new(Direction::North, 5.0),
		TrafficLight::new(Direction::South, 5.0),
		TrafficLight::new(Direction::West, 5.0),
		TrafficLight::new(Direction::East, 5.0),
	];

	let mut light_views: Vec<TrafficLightView> = lights
		.into_iter()
		.enumerate()
		.map(|(_i, l)| {
			let pos = match l.dir {
				Direction::North => Vec2::new(screen_size.x / 2.0 - 50.0, 50.0),
				Direction::South => Vec2::new(screen_size.x / 2.0 + 50.0, screen_size.y - 50.0),
				Direction::West => Vec2::new(50.0, screen_size.y / 2.0 - 50.0),
				Direction::East => Vec2::new(screen_size.x - 50.0, screen_size.y / 2.0 + 50.0),
			};
			TrafficLightView::new(l, pos)
		})
		.collect();

	let mut adaptive_mode = false;

	fastrand::seed(0);

	loop {
		clear_background(BLACK);

		draw_crossroad();

		if fastrand::i32(0..100) < spawn_prob {
			car_views.push(CarView::new(speed));
		}

		for car_view in car_views.iter_mut() {
			let car = &mut car_view.car;

			car.moving = true;

			for stop in &stop_lines {
				if car.reached_stop_line(stop) {
					car.moving = false;
					break;
				}
			}

			car.step();
		}

		let dt = get_frame_time();

		for light in light_views.iter_mut() {
			light.traffic_light.update(dt);
		}

		for view in light_views.iter() {
			view.draw();
		}

		for car_view in car_views.iter() {
			car_view.draw();
		}

		car_views.retain(|car_view| !car_view.car.is_outside(get_screen_size()));

		let mode_text = if adaptive_mode {
			"Режим: Адаптивный"
		} else {
			"Режим: Обычный"
		};
		draw_text_ex(
			mode_text,
			20.0,
			30.0,
			TextParams {
				font: Some(&font),
				font_size: 30,
				color: BLACK,
				..Default::default()
			},
		);

		if is_key_pressed(KeyCode::Space) {
			adaptive_mode = !adaptive_mode;
		}
		if is_key_pressed(KeyCode::R) {
			println!("Симуляция перезапущена!");
		}

		next_frame().await;
	}
}

fn get_screen_size() -> Vec2 {
	Vec2::new(screen_width(), screen_height())
}

fn draw_crossroad() {
	let w = screen_width();
	let h = screen_height();

	draw_rectangle(0.0, h / 2.0 - LINE_THICKNESS / 2.0, w, LINE_THICKNESS, GRAY);
	draw_rectangle(w / 2.0 - LINE_THICKNESS / 2.0, 0.0, LINE_THICKNESS, h, GRAY);
}