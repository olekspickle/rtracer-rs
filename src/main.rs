use std::path::Path;
use image::{DynamicImage, GenericImageView, Rgba};

mod entities;
mod fractal;
mod my_macro;
mod rendering;

use entities::{Color, Point, Scene, Sphere};
use fractal::{Fractal};
use rendering::{Intersectable, Ray};

fn main() {
    let scene = Scene::default();
    let img: DynamicImage = scene.render();
    let f = Fractal::default();

    save_image(img, &Path::new("output/test_scene.png"));
    f.save(&Path::new("output/fractal.png"));
}

// pub fn render(scene: &Scene) -> DynamicImage {
//     let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
//     let black = Rgba::from_channels(0, 0, 0, 0);
//     for x in 0..scene.width {
//         for y in 0..scene.height {
//             let ray = Ray::create_prime(x, y, scene);

//             if scene.sphere.intersect(&ray) {
//                 image.put_pixel(x, y, to_rgba(&scene.sphere.color))
//             } else {
//                 image.put_pixel(x, y, black);
//             }
//         }
//     }
//     image
// }

pub fn save_image(img: DynamicImage, p: &Path) {
    println!("saving image to {:?} ", p);

    match img.save(p) {
        Ok(_) => println!("saved successfully"),
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
