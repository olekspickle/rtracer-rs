//! An example of generating julia fractals.
use crate::{save_image};
use image::{ImageBuffer, DynamicImage, Rgb};
use num_complex;
use std::path::Path;

pub struct Fractal {
    pub x: u32,
    pub y: u32,
}

impl Fractal {
    pub fn new(self, x: u32, y: u32) -> Fractal {
        Fractal { x, y }
    }
    pub fn save(self, p: &Path) {
        let scalex = 3.0 / self.x as f32;
        let scaley = 3.0 / self.y as f32;

        // Create a new ImgBuf with width: self.x and height: self.y
        let mut imgbuf = ImageBuffer::new(self.x, self.y);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let r = (0.3 * x as f32) as u8;
            let b = (0.3 * y as f32) as u8;
            *pixel = Rgb([r, 0, b]);
        }

        // A redundant loop to demonstrate reading image data
        for x in 0..self.x {
            for y in 0..self.y {
                let cx = y as f32 * scalex - 1.5;
                let cy = x as f32 * scaley - 1.5;

                let c = num_complex::Complex::new(-0.4, 0.6);
                let mut z = num_complex::Complex::new(cx, cy);

                let mut i = 0;
                while i < 255 && z.norm() <= 2.0 {
                    z = z * z + c;
                    i += 1;
                }

                let pixel = imgbuf.get_pixel_mut(x, y);
                let data = (*pixel as Rgb<u8>).0;
                *pixel = Rgb([data[0], i as u8, data[2]]);
            }
        }

        save_image(DynamicImage::ImageRgb8(imgbuf), p);
    }
}

impl Default for Fractal {
    fn default() -> Fractal {
        Fractal { x: 800, y: 800 }
    }
}
