
//! [![v](https://img.shields.io/badge/v-0.0.4-blueviolet)]()
//! # ray tracer in rust
//! 
//! Here is attempt to write ray-tracer in rust-lang
//! 
//! 1. Simple sphere 
//! 2. Spheres on a plane 
//! 3. Basic shadows
//! 
//! 
//! ### useful resourses
//! [Scrathpixel](https://www.scratchapixel.com/index.php?redirect) is fantastic library! 
//! They have great materials and specific images and docs! You should definately check them out.
//! 
use image::DynamicImage;
use std::{path::Path, fs::File};

mod entities;
mod fractal;
mod point;
mod rendering;
mod utils;
mod vector;

pub use entities::*;
pub use fractal::*;
pub use point::*;
pub use rendering::*;
pub use utils::*;
pub use vector::*;

pub fn main() {
    let scene_path = Path::new("scenes/test.json");
    let scene_file = File::open(scene_path).expect("File not found");
    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    let img: DynamicImage = scene.render();
    let f = Fractal::default();

    save_image(img, &Path::new("output/test_scene.png"));
    f.save(&Path::new("output/fractal.png"));
}


