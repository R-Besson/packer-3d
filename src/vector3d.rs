#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Vector3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3 {
    // Method to create a new instance of Vector3
    pub fn new(x: i32, y: i32, z: i32) -> Vector3 {
        Vector3 {x, y, z}
    }
}