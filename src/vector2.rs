use std::fmt::{Display, Formatter};
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

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Vector2i {
    pub(crate) x: i32,
    pub(crate) y: i32
}

impl Vector2i {
    pub fn get_manhattan_distance(self, other: Vector2i) -> i32 {
        let absolute = (self - other).abs();

        absolute.x + absolute.y
    }

    fn abs(&self) -> Vector2i {
        return Vector2i {
            x: self.x.abs(),
            y: self.y.abs()
        }
    }

}

impl Display for Vector2i {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&("(" .to_owned() + self.x.to_string().as_str() + ", " + self.y.to_string().as_str() + ")"))
    }
}

impl ops::Add<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn add(self, other: Vector2i) -> Vector2i {
        return Vector2i {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Vector2i> for Vector2i {
    type Output = Vector2i;

    fn sub(self, other: Vector2i) -> Vector2i {
        return Vector2i {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<i32> for Vector2i {
    type Output = Vector2i;

    fn mul(self, other: i32) -> Vector2i {
        return Vector2i {
            x: self.x * other,
            y: self.y * other,
        }
    }
}


#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub struct Vector2i64 {
    pub(crate) x: i64,
    pub(crate) y: i64
}

impl Vector2i64 {
    pub fn get_manhattan_distance(self, other: Vector2i64) -> i64 {
        let absolute = (self - other).abs();

        absolute.x + absolute.y
    }

    fn abs(&self) -> Vector2i64 {
        return Vector2i64 {
            x: self.x.abs(),
            y: self.y.abs()
        }
    }

}

impl ops::Add<Vector2i64> for Vector2i64 {
    type Output = Vector2i64;

    fn add(self, other: Vector2i64) -> Vector2i64 {
        return Vector2i64 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub<Vector2i64> for Vector2i64 {
    type Output = Vector2i64;

    fn sub(self, other: Vector2i64) -> Vector2i64 {
        return Vector2i64 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl ops::Mul<i64> for Vector2i64 {
    type Output = Vector2i64;

    fn mul(self, other: i64) -> Vector2i64 {
        return Vector2i64 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}