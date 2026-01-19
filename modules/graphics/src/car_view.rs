use macroquad::prelude::*;
use types::{Car, Direction};

pub struct CarView {
	car: Car,
	color: Color
}

impl CarView {
	pub fn new(speed: f32) -> CarView {
		let w = screen_width();
		let h = screen_height();

		let n = fastrand::i32(0..4);

		let pos = match n {
			0 => Vec2::new(w/speed - Car::WIDTH, h),
			1 => Vec2::new(w/speed - Car::WIDTH, -Car::HEIGHT),
			2 => Vec2::new(w, h/speed - Car::WIDTH/2.0),
			3 => Vec2::new(-Car::HEIGHT, h/speed - Car::WIDTH/2.0),
			_ => unreachable!()
		};

		let car = match n {
			0 => Car::new(pos, Direction::North, speed),
			1 => Car::new(pos, Direction::South, speed),
			2 => Car::new(pos, Direction::West, speed),
			3 => Car::new(pos, Direction::East, speed),
			_ => unreachable!()
		};

    	let color = Color::from_rgba(
    	    fastrand::u8(0..=255),
    	    fastrand::u8(0..=255),
    	    fastrand::u8(0..=255),
    	    255
    	);

		Self { car, color }
	}

	pub fn draw(&self) {
        let size = self.car.size();
		let pos = self.car.pos();
        draw_rectangle(pos.x, pos.y, size.x, size.y, self.color);
	}
}