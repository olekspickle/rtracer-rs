use crate::entities::{Color, Element, Intersection, Plane, Scene, Sphere, ViewBlock};
use crate::{point::Point, vector::Vector3};
use image::{DynamicImage, GenericImage, Pixel, Rgba};

const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Default for Ray {
    fn default() -> Ray {
        Ray {
            origin: Point::zero(),
            direction: Vector3::zero(),
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
            direction: Vector3 {
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
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
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
        let l: Vector3 = self.center - ray.origin;
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj = l.dot(&ray.direction);
        //Find the length-squared of the opposite side
        let d2 = l.dot(&l) - (adj * adj);
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
    pub fn cast_ray(&self, ray: &Ray, depth: u32) -> Color {
        if depth >= self.max_recursion_depth {
            return BLACK;
        }
        let intersection = self.trace(&ray);
        intersection.map(|i| BLACK).unwrap_or(BLACK)
    }

    pub fn render(scene: &Scene) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
        let black = Rgba::from_channels(0u8, 0u8, 0u8, 0u8);
        for x in 0..scene.width {
            for y in 0..scene.height {
                let ray = Ray::create_prime(x, y, scene);
    
                let intersection = scene.trace(&ray);
                let color = intersection.map(|i| &i.object.color.to_rgba())
                    .unwrap_or(&black);
                image.put_pixel(x, y, *color);
            }
        }
        image
    }

    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }

    pub fn spheres() -> Scene {
        let s_1 = Element::Sphere(Sphere {
            center: Point::new(0.0, 0.0, -5.0),
            radius: 2.0,
            color: Color::new(0.4, 0.7, 0.4),
        });
        let s_2 = Element::Sphere(Sphere {
            center: Point::new(-5.0, 1.0, -5.0),
            radius: 1.0,
            color: Color::new(0.4, 0.4, 0.7),
        });
        let s_3 = Element::Sphere(Sphere {
            center: Point::new(5.0, 0.0, -5.0),
            radius: 3.0,
            color: Color::new(0.7, 0.4, 0.4),
        });

        Scene {
            width: 800,
            height: 600,
            fov: 90.0,
            elements: vec![s_1, s_2, s_3],
            max_recursion_depth: 0,
        }
    }
}
