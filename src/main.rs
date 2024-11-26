use std::{path::Path};

use image::{DynamicImage, GenericImageView};

mod my_macro;
mod entities;
mod rendering;

use entities::{Color, Point, Scene, Sphere};

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
        
        assert_eq!(scene.width, img.width());
        assert_eq!(scene.height, img.height());
        
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

    }
}
