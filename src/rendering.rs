use crate::entities::{Point, Element, Plane, Scene,Intersection, Sphere};
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
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;
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
    fn intersect(&self, ray: &Ray) -> Option<f64>;
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }
}
impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = self.normal;
        let denom = normal.dot(ray.direction);
        if denom > 1e-6 {
            let v = self.p0 - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
}
impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vec3<f64> = self.center - ray.origin;
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj = l.dot(ray.direction);
        //Find the length-squared of the opposite side
        // dot(self, v: Vec3) -> (self * v).sum()
        // sum(self) -> self.into_iter().sum()
        // vek macros magic explained ^
        let d2 = l.dot(l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let distance = if t0 < t1 { t0 } else { t1 };
        Some(distance)
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
                let mut ray = Ray::create_prime(x, y, self);
                ray.direction.normalize();

                if self.sphere.intersect(&ray) {
                    image.put_pixel(x, y, sphere_color)
                } else {
                    image.put_pixel(x, y, background);
                }
            }
        }
        image
    }

    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.spheres
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
