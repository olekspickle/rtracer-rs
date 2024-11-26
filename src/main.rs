use std::path::Path;

use image::{DynamicImage, GenericImageView};

mod entities;
mod my_macro;
mod rendering;

use entities::{Color, Point, Scene, Sphere};

fn main() {
    let scene = Scene::default();
    let img: DynamicImage = scene.render();
    let _result = save_image(img, &Path::new("output/test.png"));
}

pub fn render(scene: &Scene) -> DynamicImage {
    DynamicImage::new_rgb8(scene.width, scene.height)
}

pub fn save_image(img: DynamicImage, p: &Path) {
    match img.save(p) {
        Ok(ok) => println!("saved successfully {:?}", ok),
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
