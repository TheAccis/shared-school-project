use std::collections::HashMap;

use types::*;

pub struct SimulationOptions {
    pub spawn_chance: f32,
    pub road_capacity: usize,
}

pub struct Simulation {
    options: SimulationOptions,

    pub lanes: HashMap<Direction, Road>,
    pub lights: HashMap<Direction, TrafficLight>,
    pub time: f32,
}

impl Simulation {
    pub fn new(options: SimulationOptions) -> Self {
        let mut lanes = HashMap::new();
        let mut lights = HashMap::new();

        for dir in Direction::all() {
            lanes.insert(dir, Road::new(dir));
            lights.insert(dir, TrafficLight::new(dir));
        }

        Self {
            options,
            lanes,
            lights,
            time: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        // spawn cars
        for dir in Direction::all() {
            if fastrand::f32() < self.options.spawn_chance {
                let lane = self.lanes.get_mut(&dir).unwrap();
                if lane.queue.len() < self.options.road_capacity {
                    lane.queue.push_back(Car {
                        dir,
                        wait_time: 0.0,
                    });
                }
            }
        }

        // update lights
        for light in self.lights.values_mut() {
            light.update(dt);
        }

        // move cars
        for dir in Direction::all() {
            let light = &self.lights[&dir];
            let lane = self.lanes.get_mut(&dir).unwrap();

            if light.green {
                lane.queue.pop_front();
            }

            for car in lane.queue.iter_mut() {
                car.wait_time += dt;
            }
        }
    }
}