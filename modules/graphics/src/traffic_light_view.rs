use macroquad::prelude::*;
use types::*;

pub struct TrafficLightView {
    pub light: TrafficLight,
    pub dir: Direction,
}

impl TrafficLightView {
    pub fn draw(&self) {
        let color = if self.light.green { GREEN } else { RED };
        draw_circle(self.light.pos.x, self.light.pos.y, 10.0, color);
    }
}