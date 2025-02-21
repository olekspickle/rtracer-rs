use crate::entities::{Point, Scene, Sphere};
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use vek::Vec3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3<f64>,
}

impl Default for Ray {
    fn default() -> Ray {
        Ray {
            origin: Point::zero(),
            direction: Vec3::zero(),
        }
    }
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        // take non-quadratic images into account
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x =
            (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0 * aspect_ratio) * fov_adjustment;
        let sensor_y = 1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0;
        Ray {
            origin: Point::zero(),
            direction: Vec3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            },
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vec3<f64> = self.center - ray.origin;
        println!("z axis sphere center {:?} - ray origin {:?}", self.center.z, ray.origin.z);
        
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(ray.direction);
        //Find the length-squared of the opposite side
        // dot(self, v: Vec3) -> (self * v).sum()
        // sum(self) -> self.into_iter().sum()
        // vek macros magic explained ^
        let d2 = l.dot(l) - (adj2 * adj2);

        // print!("{} - {} = ", l.dot(l), adj2);
        // print!("{} =?", l.dot(l) - (adj2 * adj2));
        // println!(" {} ", d2);
        //pythagorean theorem
        //If that length-squared is less than radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
    }
}

impl Scene {
    pub fn render_simple_sphere(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let background = Rgba::from_channels(0, 0, 0, 0);
        let sphere_color = Rgba::from_channels(
            (self.sphere.color.red * 255.0) as u8,
            (self.sphere.color.green * 255.0) as u8,
            (self.sphere.color.blue * 255.0) as u8,
            0,
        );
        
        for x in 0..self.width {
            for y in 0..self.height {   
                let ray = Ray::create_prime(x, y, self);

                if self.sphere.intersect(&ray) {
                    image.put_pixel(x, y, sphere_color)
                } else {
                    println!("made it! x{} y{}", x, y);
                    image.put_pixel(x, y, background);
                }
            }
        }
        image
    }
}
