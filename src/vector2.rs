use std::ops;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Vector2 {
    pub(crate) x: usize,
    pub(crate) y: usize
}

impl Vector2 {
    // fn get_manhattan_distance(self, other: Vector2) -> usize {
    //     let absolute = (self - other).abs();
    //
    //     let mut distance = absolute.x + absolute.y;
    //
    //
    //     if(absolute.x != 0 && absolute.y != 0){
    //         distance -= 1;
    //     }
    //
    //     return distance;
    // }

    // fn abs(&self) -> Vector2 {
    //     return Vector2 {
    //         x: self.x.abs(),
    //         y: self.y.abs()
    //     }
    // }

}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, other: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, other: Vector2) -> Vector2 {
        return Vector2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<usize> for Vector2 {
    type Output = Vector2;

    fn mul(self, other: usize) -> Vector2 {
        return Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}