use macroquad::prelude::*;
use glam::Vec2;

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

struct Car {
    x: f32,
    y: f32,
    dir: Direction,
    color: Color,
    speed: f32,
}

impl Car {
    fn update(&mut self) {
        match self.dir {
            Direction::North => self.y -= self.speed,
            Direction::South => self.y += self.speed,
            Direction::West  => self.x -= self.speed,
            Direction::East  => self.x += self.speed,
        }
    }

    fn draw(&self) {
        let size= self.size();
        draw_rectangle(self.x, self.y, size.x, size.y, self.color);
    }

    fn size(&self) -> Vec2 {
        match self.dir {
            Direction::North | Direction::South => Vec2::new(10.0, 20.0),
            Direction::West | Direction::East => Vec2::new(20.0, 10.0),
        }
    }

    fn is_outside(&self) -> bool {
        self.x < -50.0
            || self.x > screen_width() + 50.0
            || self.y < -50.0
            || self.y > screen_height() + 50.0
    }
}

#[macroquad::main("Traffic Intersection Demo")]
async fn main() {
    let font = load_ttf_font("res/Inter_18pt-Medium.ttf").await.unwrap();

    let mut cars: Vec<Car> = vec![];
    let spawn_prob = 2; // %

    let mut adaptive_mode = false;

    fastrand::seed(0);

    loop {
        clear_background(BLACK);

        draw_crossroad();

        if fastrand::i32(0..100) < spawn_prob {
            cars.push(spawn_car());
        }

        for car in cars.iter_mut() {
            car.update();
        }

        for car in cars.iter() {
            car.draw();
        }

        cars.retain(|car| !car.is_outside());

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

fn spawn_car() -> Car {
    let color = Color::from_rgba(
        fastrand::u8(0..=255),
        fastrand::u8(0..=255),
        fastrand::u8(0..=255),
        255
    );

    let w = screen_width();
    let h = screen_height();
    let speed = 2.0;

    match fastrand::i32(0..4) {
        0 => Car { x: w/2.0 - 10.0, y: h, dir: Direction::North, color, speed },
        1 => Car { x: w/2.0 - 10.0, y: -10.0, dir: Direction::South, color, speed },
        2 => Car { x: w, y: h/2.0 - 5.0, dir: Direction::West, color, speed },
        3 => Car { x: -20.0, y: h/2.0 - 5.0, dir: Direction::East, color, speed },
        _ => unreachable!()
    }
}

fn draw_crossroad() {
    let w = screen_width();
    let h = screen_height();

    let line_thickness = 40.0;

    draw_rectangle(0.0, h/2.0 - line_thickness/2.0, w, line_thickness, GRAY);
    draw_rectangle(w/2.0 - line_thickness/2.0, 0.0, line_thickness, h, GRAY);
}