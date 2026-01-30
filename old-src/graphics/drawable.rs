use crate::graphics::WorldCamera;
use crate::io::Assets;

pub trait Drawable {
	fn draw(&self, cam: &WorldCamera, resources: &Assets);
}