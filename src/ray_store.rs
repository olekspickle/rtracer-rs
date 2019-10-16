use num_traits::identities::Zero;
use serde_derive::Deserialize;
use std::ops::Add;
use vek::Vec3;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
            x: 0.0f32,
            y: 0.0f32,
            z: 0.0f32,
        }
    }
    fn is_zero(&self) -> bool {
        let Point { x, y, z } = self;

        if *x == 0f32 || *y == 0f32 || *z == 0f32 {
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
