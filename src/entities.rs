use crate::{point::Point, vector::Vector3};
use num_traits::identities::Zero;
use serde_derive::{Deserialize, Serialize};
use std::ops::Add;

#[repr(C)]
#[derive(Debug)]
pub struct ViewBlock {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub max_recursion_depth: u32,
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

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Sphere,
}
impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b Sphere) -> Intersection<'b> {
        Intersection { distance, object }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Plane {
    pub p0: Point,
    pub normal: Vector3,
    pub color: Color,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &s.color,
            Element::Plane(ref p) => &p.color,
        }
    }
}


impl Color {
    
}