use crate::types::LightState;

use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TrafficLightBehaviour {
	Standart,
	Adaptive,
}

#[derive(Clone, Debug)]
pub struct TrafficLight {
	pub behaviour: TrafficLightBehaviour,
	pub stop_s: f32,
	pub state: LightState,
	pub pos: Vec2,
    pub timer: f32,
}
impl TrafficLight {
	const GREEN_TIME: f32 = 8.0;
const YELLOW_TIME: f32 = 2.0;
const RED_TIME: f32 = 8.0;

	pub fn new(behaviour: TrafficLightBehaviour, stop_s: f32, state: LightState, pos: Vec2) -> Self {
		Self {
			behaviour,
			stop_s,
			state,
			pos,
			timer: 0.0,
		}
	}

pub fn update(&mut self, dt: f32) {
    match self.behaviour {
        TrafficLightBehaviour::Standart => {
            self.timer += dt;

            match self.state {
                LightState::Green => {
                    if self.timer >= Self::GREEN_TIME {
                        self.state = LightState::Yellow;
                        self.timer = 0.0;
                    }
                }
                LightState::Yellow => {
                    if self.timer >= Self::YELLOW_TIME {
                        self.state = LightState::Red;
                        self.timer = 0.0;
                    }
                }
                LightState::Red => {
                    if self.timer >= Self::RED_TIME {
                        self.state = LightState::Green;
                        self.timer = 0.0;
                    }
                }
            }
        }

        TrafficLightBehaviour::Adaptive => {
            // Пока пусто, чтобы стандарт не мешался
        }
    }
}


	pub fn is_red(&self) -> bool { self.state == LightState::Red }
}