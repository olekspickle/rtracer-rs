use crate::{
    point::Point,
    rendering::{Intersectable, Ray, BLACK},
    vector::Vector3,
};
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use serde_derive::{Deserialize, Serialize};
use std::ops::{Mul, Add};

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

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            blue: self.blue + other.blue,
            green: self.green + other.green,
        }
    }
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

pub enum Coloration {
    Color(Color),
    Texture(DynamicImage)
}

pub struct Material {
    pub color: Coloration,
    pub albedo: f32,
}

impl Coloration {
    pub fn color(&self, texture_coords: &TextureCoords) -> Color {
        match *self {
            Coloration::Color(c) => c,
            Coloration::Texture(tex) => {
                Color {
                    red: 0.0,
                    blue: 0.0,
                    green: 0.0,
                }
            }
        }
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DirectionalLight {
    #[serde(deserialize_with="Vector3::deserialize_normalized")]
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

impl Light {
    pub fn color(&self) -> Color {
        match *self {
            Light::Directional(ref d) => d.color,
            Light::Spherical(ref s) => s.color,
        }
    }

    pub fn direction_from(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Light::Directional(ref d) => -d.direction,
            Light::Spherical(ref s) => (s.position - *hit_point).normalize(),
        }
    }

    pub fn intensity(&self, hit_point: &Point) -> f32 {
        match *self {
            Light::Directional(ref d) => d.intensity,
            Light::Spherical(ref s) => {
                let r2 = (s.position - *hit_point).norm() as f32;
                s.intensity / (4.0 * ::std::f32::consts::PI * r2)
            }
        }
    }

    pub fn distance(&self, hit_point: &Point) -> f64 {
        match *self {
            Light::Directional(_) => ::std::f64::INFINITY,
            Light::Spherical(ref s) => (s.position - *hit_point).length(),
        }
    }
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
    pub material: Material,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn material(&self) -> Material {
        match *self {
            Element::Sphere(ref s) => s.material,
            Element::Plane(ref p) => p.material,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
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
        let material = intersection.element.material();
        let mut color = Color {
            red: 0.0,
            blue: 0.0,
            green: 0.0,
        };
        for light in &self.lights {
            let direction_to_light = light.direction_from(&hit_point);
        
            let shadow_ray = Ray {
                origin: hit_point + (direction_to_light * self.shadow_bias),
                direction: direction_to_light,
            };
            let shadow_intersection = self.trace(&shadow_ray);
            let in_light = shadow_intersection.is_none() ||
            shadow_intersection.unwrap().distance > light.distance(&hit_point);
            let light_intensity = if in_light { light.intensity(&hit_point) } else { 0.0 };
            let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) *
                              light_intensity;
            let light_reflected = material.albedo / std::f32::consts::PI;
            let light_color = light.color() * light_power * light_reflected;
            color = color + (material.color * light_color);
        }
        color.clamp()
    }
}
