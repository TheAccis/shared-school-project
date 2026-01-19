use macroquad::prelude::*;
use types::TrafficLight;
use glam::Vec2;

pub struct TrafficLightView {
    pub traffic_light: TrafficLight,
    pub pos: Vec2,
    pub radius: f32,
}

impl TrafficLightView {
    pub fn new(traffic_light: TrafficLight, pos: Vec2) -> Self {
        Self {
            traffic_light,
            pos,
            radius: 10.0,
        }
    }

    pub fn draw(&self) {
        let color = if self.traffic_light.green { GREEN } else { RED };
        draw_circle(self.pos.x, self.pos.y, self.radius, color);
    }
}