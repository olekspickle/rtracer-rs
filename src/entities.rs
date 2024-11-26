use image::DynamicImage;
use num_traits::identities::Zero;
use serde_derive::{Deserialize, Serialize};
use std::ops::{Add, Sub};
use vek::Vec3;

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
}

impl Add<Vec3<f64>> for Point {
    type Output = Point;

    fn add(self, other: Vec3<f64>) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Add<Point> for Vec3<f64> {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        other + self
    }
}

impl Sub<Vec3<f64>> for Point {
    type Output = Point;

    fn sub(self, other: Vec3<f64>) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Sub<Point> for Vec3<f64> {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        other - self
    }
}

impl Sub<Point> for Point {
    type Output = Vec3<f64>;

    fn sub(self, other: Point) -> Vec3<f64> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
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

impl Default for Scene {
    fn default() -> Scene {
        Scene {
            width: 800,
            height: 600,
            fov: 90.0,
            sphere: Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.4,
                    green: 1.0,
                    blue: 0.4,
                },
            },
        }
    }
}

impl Scene {
    pub fn render(&self) -> DynamicImage {
        DynamicImage::new_rgb8(self.width, self.height)
    }
}
