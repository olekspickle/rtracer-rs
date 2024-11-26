use serde_derive::Deserialize;
use vek::Vec3;

// #[derive(Debug, Deserialize)]
// pub struct Ray {
//     A: Vec3<f32, f32, f32>,
//     B: Vec3<f32, f32, f32>,
//     time: f32,
// }



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