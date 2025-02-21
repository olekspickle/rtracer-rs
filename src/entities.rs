use crate::{
    point::Point,
    rendering::{Intersectable, Ray, TextureCoords, BLACK},
    vector::Vector3,
};
use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgba};
use serde::{Deserialize, Deserializer};
use serde_derive::Deserialize;

use std::{
    fmt,
    ops::{Add, Mul},
    path::PathBuf,
};

#[repr(C)]
#[derive(Debug)]
pub struct ViewBlock {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Deserialize, Clone, Copy, Debug)]
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
    pub fn from_rgba(rgba: Rgba<u8>) -> Color {
        Color {
            red: (rgba.0[0] as f32) / 255.0,
            green: (rgba.0[1] as f32) / 255.0,
            blue: (rgba.0[2] as f32) / 255.0,
        }
    }
    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.min(1.0).max(0.0),
            blue: self.blue.min(1.0).max(0.0),
            green: self.green.min(1.0).max(0.0),
        }
    }
}

pub fn load_texture<'a, D>(deserializer: D) -> Result<DynamicImage, D::Error>
where
    D: Deserializer<'a>,
{
    let path = PathBuf::deserialize(deserializer)?;
    Ok(image::open(path).expect("Unable to open texture file"))
}

#[derive(Deserialize)]
pub enum Coloration {
    Color(Color),
    Texture(#[serde(deserialize_with = "load_texture")] DynamicImage),
}
impl fmt::Debug for Coloration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Coloration::Color(ref c) => write!(f, "Color({:?})", c),
            Coloration::Texture(_) => write!(f, "Texture"),
        }
    }
}

fn wrap(val: f32, bound: u32) -> u32 {
    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}

impl Coloration {
    pub fn color(&self, coords: &TextureCoords) -> Color {
        match *self {
            Coloration::Color(ref c) => c.clone(),
            Coloration::Texture(ref texture) => {
                let tex_x = wrap(coords.x, texture.width());
                let tex_y = wrap(coords.y, texture.height());

                Color::from_rgba(texture.get_pixel(tex_x, tex_y))
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Material {
    pub coloration: Coloration,
    pub albedo: f32,
}

#[derive(Deserialize, Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}

#[derive(Deserialize, Debug)]
pub struct DirectionalLight {
    #[serde(deserialize_with = "Vector3::deserialize_normalized")]
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Deserialize, Debug)]
pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct Plane {
    pub origin: Point,
    pub normal: Vector3,
    pub material: Material,
}

#[derive(Deserialize, Debug)]
pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

impl Element {
    pub fn material(&self) -> &Material {
        match *self {
            Element::Sphere(ref s) => &s.material,
            Element::Plane(ref p) => &p.material,
        }
    }

    pub fn material_mut(&mut self) -> &mut Material {
        match *self {
            Element::Sphere(ref mut s) => &mut s.material,
            Element::Plane(ref mut p) => &mut p.material,
        }
    }
}

#[derive(Deserialize, Debug)]
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
        let texture_coords = intersection.element.texture_coords(&hit_point);

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
            let in_light = shadow_intersection.is_none()
                || shadow_intersection.unwrap().distance > light.distance(&hit_point);
            let light_intensity = if in_light {
                light.intensity(&hit_point)
            } else {
                0.0
            };
            let material = intersection.element.material();
            let light_power =
                (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
            let light_reflected = material.albedo / std::f32::consts::PI;
            let light_color = light.color() * light_power * light_reflected;
            color = color + (material.coloration.color(&texture_coords) * light_color);
        }
        color.clamp()
    }
}
