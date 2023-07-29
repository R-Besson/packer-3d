use crate::vector3d::*;
use crate::HashSetFnv;

// CUBE //
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Box3D {
	pub position: Vector3,
	pub size: Vector3,
	pub id: u32,
	pub origin: u16,
}

impl Box3D {

	// Method to create a new instance of Box3D
	pub fn from_size_position(position: Vector3, size: Vector3, id: u32, origin: u16) -> Box3D
	{
		Box3D { position, size, id, origin }
	}

	// Method 2 to create a new instance of Box3D
	pub fn from_xyz_whl(x: i32, y: i32, z: i32, w: i32, h: i32, l: i32, id: u32, origin: u16) -> Box3D
	{
		let position = Vector3::new(x,y,z);
		let size = Vector3::new(w,h,l);
		Box3D { position, size, id, origin }
	}

	// Method to calculate the volume of the cube
	pub fn volume(&self) -> i32
	{
		self.size.x * self.size.y * self.size.z
	}

	pub fn fits_in(&self, other: &Box3D) -> bool
	{
		self.size.x <= other.size.x && self.size.y <= other.size.y && self.size.z <= other.size.z
	}

	pub fn is_in(&self, other: &Box3D) -> bool 
	{   // Check if shape IS in 'other' shape using
		// a comparison of positions
		self.position.x >= other.position.x &&       // BOTTOM corner of shape is inside 'other'
		self.position.y >= other.position.y &&       //
		self.position.z >= other.position.z &&       //

		self.x2() <= other.x2() &&     // TOP corner of shape is inside 'other'
		self.y2() <= other.y2() &&     //
		self.z2() <= other.z2()        //
	}

	pub fn x2(&self) -> i32
	{
		self.position.x + self.size.x
	}
	pub fn y2(&self) -> i32
	{
		self.position.y + self.size.y
	}
	pub fn z2(&self) -> i32 
	{
		self.position.z + self.size.z
	}

	pub fn intersects(&self, other: &Box3D) -> bool
	{
		range_overlap(self.position.x, self.x2(), other.position.x, other.x2()) &&
		range_overlap(self.position.y, self.y2(), other.position.y, other.y2()) &&
		range_overlap(self.position.z, self.z2(), other.position.z, other.z2())
	}

	pub fn is_covered_among(&self, boxes: &HashSetFnv<Box3D>) -> bool
	{
		// Iterates through all the shapes and checks if it is covered by a Shape AND
		for other in boxes.iter() {
			if self.is_in(other) {
				return true;
			}
		}

		false
	}

	pub fn get_rotations(&self) -> Vec<Box3D>
	{
		vec![
			Box3D::from_size_position(self.position, Vector3::new(self.size.x, self.size.y, self.size.z), self.id, 0), // w,h,l
			Box3D::from_size_position(self.position, Vector3::new(self.size.y, self.size.x, self.size.z), self.id, 0), // h,w,l
			Box3D::from_size_position(self.position, Vector3::new(self.size.z, self.size.y, self.size.x), self.id, 0), // l,h,w
			Box3D::from_size_position(self.position, Vector3::new(self.size.x, self.size.z, self.size.y), self.id, 0), // w,l,h
			Box3D::from_size_position(self.position, Vector3::new(self.size.z, self.size.x, self.size.y), self.id, 0), // l,w,h
			Box3D::from_size_position(self.position, Vector3::new(self.size.y, self.size.z, self.size.x), self.id, 0)  // h,l,w
		]
	}
}

fn range_overlap(amin: i32, amax: i32, bmin: i32, bmax: i32) -> bool
{
	amax > bmin && bmax > amin
}