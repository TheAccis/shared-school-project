use macroquad::prelude::*;
use types::*;
use simulation::simulation::*;

struct RenderCar {
    dir: Direction,
    index: usize,
}

struct RenderLight {
    dir: Direction,
    green: bool,
}

struct RenderState {
    cars: Vec<RenderCar>,
    lights: Vec<RenderLight>,
}

fn project(sim: &Simulation) -> RenderState {
    let mut cars = Vec::new();
    let mut lights = Vec::new();

    for (&dir, lane) in &sim.lanes {
        for (i, _) in lane.queue.iter().enumerate() {
            cars.push(RenderCar { dir, index: i });
        }
    }

    for (&dir, light) in &sim.lights {
        lights.push(RenderLight {
            dir,
            green: light.green,
        });
    }

    RenderState { cars, lights }
}

/* =========================
   VIEW
   ========================= */

fn draw(render: &RenderState) {
    clear_background(DARKGRAY);

    let w = screen_width();
    let h = screen_height();

    // crossroad
    draw_rectangle(0.0, h / 2.0 - 30.0, w, 60.0, GRAY);
    draw_rectangle(w / 2.0 - 30.0, 0.0, 60.0, h, GRAY);

    // cars
    for car in &render.cars {
        let offset = car.index as f32 * 14.0;

        let (x, y) = match car.dir {
            Direction::North => (w / 2.0 - 8.0, h / 2.0 + 40.0 + offset),
            Direction::South => (w / 2.0 + 8.0, h / 2.0 - 40.0 - offset),
            Direction::West => (w / 2.0 + 40.0 + offset, h / 2.0 + 8.0),
            Direction::East => (w / 2.0 - 40.0 - offset, h / 2.0 - 8.0),
        };

        draw_rectangle(x, y, 10.0, 10.0, BLUE);
    }

    // lights (1 per direction)
    for light in &render.lights {
        let color = if light.green { GREEN } else { RED };

        let (x, y) = match light.dir {
            Direction::North => (w / 2.0, h / 2.0 - 50.0),
            Direction::South => (w / 2.0, h / 2.0 + 50.0),
            Direction::West => (w / 2.0 - 50.0, h / 2.0),
            Direction::East => (w / 2.0 + 50.0, h / 2.0),
        };

        draw_circle(x, y, 8.0, color);
    }
}

const FIXED_DT: f32 = 0.1;

#[macroquad::main("Traffic Simulation")]
async fn main() {
    fastrand::seed(1);

	let options = SimulationOptions {
		spawn_chance: 0.15,
		road_capacity: 15,
	};
    let mut sim = Simulation::new(options);
    let mut accumulator = 0.0;

    loop {
        accumulator += get_frame_time();

        while accumulator >= FIXED_DT {
            sim.update(FIXED_DT);
            accumulator -= FIXED_DT;
        }

        let render = project(&sim);
        draw(&render);

        next_frame().await;
    }
}