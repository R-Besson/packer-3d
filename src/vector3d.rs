//! 3-dimensional vector

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Vector3D {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

impl Vector3D {
	/// Constructor
	pub fn new(x: i32, y: i32, z: i32) -> Vector3D {
		Vector3D {x, y, z}
	}
}