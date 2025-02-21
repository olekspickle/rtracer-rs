use image::DynamicImage;
use std::path::Path;

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
    let scene = Scene::spheres();
    let img: DynamicImage = scene.render();
    let f = Fractal::default();

    save_image(img, &Path::new("output/test_scene.png"));
    f.save(&Path::new("output/fractal.png"));
}


