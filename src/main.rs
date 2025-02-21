use std::{boxed::Box, path::Path, sync::Arc};

use serde_derive;
use vek;
use image::{DynamicImage, ImageDecoder};

mod ray_store;
mod rendering;
mod my_macro;

use ray_store::{Scene, Sphere, Color, Point};

fn main() {
    println!("Hello, world!");
}

pub fn render(scene: &Scene) -> DynamicImage {
    DynamicImage::new_rgb8(scene.width, scene.height)
}



mod test{
    use super::*;

    #[test]
    fn test_can_render_scene() {
        use image::DynamicImage;

        let scene = Scene {
            width: 800,
            height: 600,
            fov: 90.0,
            sphere: Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.4,
                    green: 1.0,
                    blue: 0.4,
                },
            },
        };

        let img: DynamicImage = render(&scene);
        match img.save(&Path::new("/output/test.png")) {
            Ok(ok) => println!("saved successfully {:?}", ok),
            Err(err) => println!("failed to save {:?}", err),
        }
        
        // TODO: width & height functions are public in ImageBuffer struct 
        // I have no idea why it is broken 
        // assert_eq!(scene.width, img.width());
        // assert_eq!(scene.height, img.height());
    }
}

