use serde_derive::Deserialize;
use vek::Vec3;

#[derive(Debug, Deserialize)]
pub struct Ray {
    // A: Vec3<u32>,
    // B: Vec3<u32>,
    time: f32,
}