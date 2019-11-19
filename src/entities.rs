use crate::{
    point::Point,
    rendering::{Intersectable, Ray, BLACK},
    vector::Vector3,
};
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use serde_derive::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug)]
pub struct ViewBlock {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub max_recursion_depth: u32,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Element,
}
impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b Element) -> Intersection<'b> {
        Intersection { distance, object }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &s.color,
            Element::Plane(ref p) => &p.color,
        }
    }
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color {
            red: red,
            green: green,
            blue: blue,
        }
    }
    pub fn to_rgba(self) -> Rgba<u8> {
        Rgba::from_channels(
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
            0,
        )
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

    pub fn render(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let black = Rgba::from_channels(0u8, 0u8, 0u8, 0u8);
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = Ray::create_prime(x, y, self);
                let intersection = self.trace(&ray);
                let color = intersection
                    .map(|i| i.object.color().to_rgba())
                    .unwrap_or(black);
                image.put_pixel(x, y, color);
            }
        }
        image
    }

    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}
