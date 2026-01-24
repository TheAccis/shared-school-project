use crate::graphics::{WorldCamera, Drawable};
use crate::types::World;
use crate::io::Assets;

impl Drawable for World {
	fn draw(&self, cam: &WorldCamera, assets: &Assets) {
		for road in &self.roads {
			road.draw(cam, assets);
		}
		for lane in &self.lanes {
			lane.draw(cam, assets);
		}
	}
}