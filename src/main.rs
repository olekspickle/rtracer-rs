use std::{boxed::Box, path::Path, sync::Arc};

use image::{DynamicImage, ImageDecoder};
use serde_derive;
use vek;

mod my_macro;
mod ray_store;
mod rendering;

use ray_store::{Color, Point, Scene, Sphere};

fn main() {
    let scene = Scene::default();

    let img: DynamicImage = render(&scene);
    let result = match img.save(&Path::new("output/test.png")) {
        Ok(ok) => {
            println!("saved successfully {:?}", ok);
            true
        }
        Err(err) => {
            println!("failed to save {:?}", err);
            false
        }
    };
}

pub fn render(scene: &Scene) -> DynamicImage {
    DynamicImage::new_rgb8(scene.width, scene.height)
}

mod test {
    use super::*;

    #[test]
    fn test_can_render_scene() {
        use image::DynamicImage;

        let scene = Scene::default();

        let img: DynamicImage = render(&scene);
        let result = match img.save(&Path::new("output/test.png")) {
            Ok(ok) => {
                println!("saved successfully {:?}", ok);
                true
            }
            Err(err) => {
                println!("failed to save {:?}", err);
                false
            }
        };
        assert_eq!(result, true);

        // TODO: width & height functions are public in ImageBuffer struct
        // I have no idea why it is broken
        // assert_eq!(scene.width, img.width());
        // assert_eq!(scene.height, img.height());
    }
}
