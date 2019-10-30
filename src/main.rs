use image::{DynamicImage, GenericImageView};
use std::path::Path;

mod entities;
mod fractal;
mod rendering;
mod utils;

use entities::{Color, Point, Scene, Sphere};
use fractal::Fractal;
use utils::{print_green, print_italic};

fn main() {
    let scene = Scene::default();
    let img: DynamicImage = scene.render();
    let f = Fractal::default();

    save_image(img, &Path::new("output/test_scene.png"));
    f.save(&Path::new("output/fractal.png"));
}

pub fn save_image(img: DynamicImage, p: &Path) {
    print_italic(&format!("saving as {:?}", p));

    match img.save(p) {
        Ok(_) => print_green("success!"),
        Err(err) => println!("failed to save {:?}", err),
    }
}

pub fn test_save_image(img: DynamicImage, p: &Path) -> bool {
    match img.save(p) {
        Ok(_) => true,
        Err(_) => false,
    }
}

mod test {
    #[allow(unused)]
    use super::*;

    #[test]
    fn test_can_render_scene() {
        let scene = Scene::default();
        let img: DynamicImage = scene.render();
        assert_eq!(scene.width, img.width());
        assert_eq!(scene.height, img.height());

        let result = test_save_image(img, &Path::new("output/test.png"));
        assert_eq!(result, true);
    }
}
