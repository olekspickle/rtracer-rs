//! [![v](https://img.shields.io/badge/v-0.0.6-blueviolet)]()
//! ![raytracing example](https://user-images.githubusercontent.com/22867443/182672124-c3fa0155-8215-41e7-8ecd-4c58dd8afa18.png)
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
//! They have great materials and specific images and docs! You should definitely check them out.
//! #### (although they seem to have some certificate issues as of today 02/06/2020...)
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
    let scene_path = Path::new("scenes/main.json");
    let scene_file = File::open(scene_path).expect("File not found");
    let scene: Scene = serde_json::from_reader(scene_file).unwrap();

    // Render main scene with 3 spheres and 2 planes image
    let start = Instant::now();
    println!("Start ray tracing image render...");
    let img: DynamicImage = scene.render();
    save_image(img, &Path::new("output/test_scene.png"));

    // // Render fractal image
    // println!("Start fractal image render...");
    // let f = Fractal::default();
    // f.save(&Path::new("output/fractal.png"));

    println!("Elapsed: {:?}", start.elapsed());
}
