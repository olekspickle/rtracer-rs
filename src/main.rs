//! [![v](https://img.shields.io/badge/v-0.0.5-blueviolet)]()
//! ![raytracing example](/pics/example.png)
//!
//! Thanks to amazing [criterion author](https://github.com/bheisler) for his raytraycing project!
//!
//! It turned out to be hell of a mutch bigger deal I initially thought it would. 
//! Consider this a version update. 
//!
//! This stuff is hard for me personally without any background in rendering and
//! basic understanding of linear transformations.
//! Also some parts are different two years later, so I am not able to blindly re-type all of the code anyway.
//! And this is great experience! <3
//!
//! 1. Simple sphere
//! 2. Spheres on a plane
//! 3. Basic shadows
//! 4. Texturing: using vector products to calculate texture
//! 5. Reflections: basic implementation with recursive restriction.
//!
//! ### useful resourses
//! [Scrathpixel](https://www.scratchapixel.com/index.php?redirect) is fantastic library!
//! They have great materials and specific images and docs! You should definately check them out.
//!
use image::DynamicImage;
use std::{fs::File, path::Path, time::Instant};

mod entities;
mod fractal;
mod point;
mod rendering;
mod scene;
mod utils;
mod vector;

pub use entities::*;
pub use fractal::*;
pub use point::*;
pub use rendering::*;
use scene::Scene;
pub use utils::*;
pub use vector::*;

pub fn main() {
    let scene_path = Path::new("scenes/test.json");
    let scene_file = File::open(scene_path).expect("File not found");
    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    let start = Instant::now();
    println!("Start ray tracing image render...");
    let img: DynamicImage = scene.render();
    save_image(img, &Path::new("output/test_scene.png"));
    println!("Elapsed: {:?}", start.elapsed());

    let start = Instant::now();
    println!("Start fractal image render...");
    let f = Fractal::default();
    f.save(&Path::new("output/fractal.png"));
    println!("Elapsed: {:?}", start.elapsed());
}
