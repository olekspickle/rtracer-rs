use crate::vector::Vector3;
use num_traits::identities::Zero;
use serde_derive::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn zero() -> Point {
        Point::from_one(0.0)
    }

    pub fn from_one(v: f64) -> Point {
        Point { x: v, y: v, z: v }
    }
    
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point { x, y, z }
    }
}

impl Add<Vector3> for Point {
    type Output = Point;

    fn add(self, other: Vector3) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Add<Point> for Vector3 {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        other + self
    }
}

impl Sub<Vector3> for Point {
    type Output = Point;

    fn sub(self, other: Vector3) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Sub<Point> for Vector3 {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        other - self
    }
}

impl Sub<Point> for Point {
    type Output = Vector3;

    fn sub(self, other: Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Add<Point> for Point {
    type Output = Point;
    fn add(self, p: Point) -> Point {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
            z: self.z + p.z,
        }
    }
}

impl Zero for Point {
    fn zero() -> Point {
        Point {
            x: 0.0f64,
            y: 0.0f64,
            z: 0.0f64,
        }
    }
    fn is_zero(&self) -> bool {
        let Point { x, y, z } = self;

        if *x == 0f64 || *y == 0f64 || *z == 0f64 {
            true
        } else {
            false
        }
    }
}
