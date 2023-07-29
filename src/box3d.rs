//! 3-dimensional box
use crate::vector3d::*;
use crate::HashSetFnv;

/// The structure of a 3-dimensional box
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Box3D {
    /// Corresponds to `x`,`y`,`z` properties
	pub position: Vector3D,
    /// Corresponds to `w`,`h`,`l` properties
	pub size: Vector3D,
    /// ID for each box
	pub id: u32,
    /// Keeps a trace of how the box evolved (only for debugging)
	pub origin: u16,
}

impl Box3D {

	/// Create Box3D from `position`, and `size`
	pub fn from_position_size(position: Vector3D, size: Vector3D, id: u32, origin: u16) -> Box3D
	{
		Box3D { position, size, id, origin }
	}

	/// Create Box3D from `xyz`, and `whl`
	pub fn from_xyz_whl(x: i32, y: i32, z: i32, w: i32, h: i32, l: i32, id: u32, origin: u16) -> Box3D
	{
		let position = Vector3D::new(x,y,z);
		let size = Vector3D::new(w,h,l);
		Box3D { position, size, id, origin }
	}

	/// Volume of the Box
	pub fn volume(&self) -> i32
	{
		self.size.x * self.size.y * self.size.z
	}

    /// Box is smaller than other
	pub fn fits_in(&self, other: &Box3D) -> bool
	{
		self.size.x <= other.size.x && self.size.y <= other.size.y && self.size.z <= other.size.z
	}

    /// Box is inside other in terms of size AND position
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

    /// Box strictly (doesn't only touch) intersects other
	pub fn intersects(&self, other: &Box3D) -> bool
	{
		range_overlap(self.position.x, self.x2(), other.position.x, other.x2()) &&
		range_overlap(self.position.y, self.y2(), other.position.y, other.y2()) &&
		range_overlap(self.position.z, self.z2(), other.position.z, other.z2())
	}

    /// Returns whether Box is covered among other Boxes
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

    /// Gets the new 6 boxes with sizes adjusted according to rotation
	pub fn get_rotations(&self) -> Vec<Box3D>
	{
		vec![
			Box3D::from_position_size(self.position, Vector3D::new(self.size.x, self.size.y, self.size.z), self.id, 0), // w,h,l
			Box3D::from_position_size(self.position, Vector3D::new(self.size.y, self.size.x, self.size.z), self.id, 0), // h,w,l
			Box3D::from_position_size(self.position, Vector3D::new(self.size.z, self.size.y, self.size.x), self.id, 0), // l,h,w
			Box3D::from_position_size(self.position, Vector3D::new(self.size.x, self.size.z, self.size.y), self.id, 0), // w,l,h
			Box3D::from_position_size(self.position, Vector3D::new(self.size.z, self.size.x, self.size.y), self.id, 0), // l,w,h
			Box3D::from_position_size(self.position, Vector3D::new(self.size.y, self.size.z, self.size.x), self.id, 0)  // h,l,w
		]
	}
}

fn range_overlap(amin: i32, amax: i32, bmin: i32, bmax: i32) -> bool
{
	amax > bmin && bmax > amin
}