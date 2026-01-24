#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
	Horizontal,
	Vertical,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
	Forward,
	Backward,
}