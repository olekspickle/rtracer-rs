use image::DynamicImage;
use std::path::Path;

pub mod entities;
pub mod fractal;
pub mod point;
pub mod rendering;
pub mod utils;
pub mod vector;

#[allow(unused)]
use entities::{Color, Scene, Sphere, ViewBlock};
use fractal::Fractal;
use utils::{print_green, print_italic};

pub fn test_save() {
    let scene = Scene::spheres();
    let img: DynamicImage = scene.render();
    let f = Fractal::default();

    if directory_exist(Path::new("output")) {
        save_image(img, &Path::new("output/test_scene.png"));
        f.save(&Path::new("output/fractal.png"));
    }
}

pub fn save_image<T>(img: T, p: &Path) {
    print_italic(&format!("saving as {:?}...", p));

    match img.save(p) {
        Ok(_) => print_green("success!"),
        Err(err) => println!("failed to save {:?}", err),
    }
}

pub fn directory_exist(p: &Path) -> bool {
    p.is_dir()
}
