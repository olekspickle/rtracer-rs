use crate::{
    point::Point,
    rendering::{Intersectable, Ray, BLACK},
    vector::Vector3,
};
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use serde_derive::{Deserialize, Serialize};
use std::ops::Mul;

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

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            blue: self.blue * other.blue,
            green: self.green * other.green,
        }
    }
}
impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            red: self.red * other,
            blue: self.blue * other,
            green: self.green * other,
        }
    }
}
impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        other * self
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
    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub albedo: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Light {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,
}
impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, element: &'b Element) -> Intersection<'b> {
        Intersection { distance, element }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f32,
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
    pub fn albedo(&self) -> f32 {
        match *self {
            Element::Sphere(ref s) => s.albedo,
            Element::Plane(ref p) => p.albedo,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub light: Light,
    pub max_recursion_depth: u32,
    pub shadow_bias: f64,
}

impl Scene {
    pub fn cast_ray(&self, ray: &Ray, depth: u32) -> Color {
        if depth >= self.max_recursion_depth {
            return BLACK;
        }
        let intersection = self.trace(&ray);
        intersection.map(|_i| BLACK).unwrap_or(BLACK)
    }

    pub fn render(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let black = Rgba::from_channels(0u8, 0u8, 0u8, 0u8);
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = Ray::create_prime(x, y, self);
                let intersection = self.trace(&ray);
                let color = intersection
                    .map(|i| self.get_color(&ray, &i).to_rgba())
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

    pub fn get_color(&self, ray: &Ray, intersection: &Intersection) -> Color {
        let hit_point = ray.origin + (ray.direction * intersection.distance);
        let surface_normal = intersection.element.surface_normal(&hit_point);
        let direction_to_light = -self.light.direction.normalize();
        let shadow_ray = Ray {
            origin: hit_point + (surface_normal * self.shadow_bias),
            direction: direction_to_light,
        };
        let in_light = self.trace(&shadow_ray).is_none();
        let light_intensity = if in_light { self.light.intensity } else { 0.0 };
        let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = intersection.element.albedo() / std::f32::consts::PI;

        let color = intersection.element.color().clone()
            * self.light.color.clone()
            * light_power
            * light_reflected;
        color.clamp()
    }
}
