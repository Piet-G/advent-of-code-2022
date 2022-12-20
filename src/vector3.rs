use std::ops;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Vector3<T> {
    pub(crate) x: T,
    pub(crate) y: T,
    pub(crate) z: T,
}

impl<T: std::ops::Add<Output = T>> ops::Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Vector3<T> {
        return Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}