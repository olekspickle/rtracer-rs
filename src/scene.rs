use crate::{
    entities::{Color, Element, Intersection, Light, SurfaceType, DEPTH},
    point::Point,
    rendering::{Intersectable, Ray, BLACK},
    vector::Vector3,
};
use image::{DynamicImage, GenericImage, Pixel, Rgba};
use serde_derive::Deserialize;

use std::f32::consts::PI;

#[derive(Deserialize, Debug)]
pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
    pub shadow_bias: f64,
    pub max_recursion_depth: u32,
}

impl Scene {
    fn cast_ray(&self, ray: &Ray, depth: u32) -> Color {
        if depth >= self.max_recursion_depth {
            return BLACK;
        }
        let intersection = self.trace(&ray);
        intersection
            .map(|i| self.get_color(ray, &i, depth))
            .unwrap_or(BLACK)
    }

    pub fn render(&self) -> DynamicImage {
        let mut image = DynamicImage::new_rgb8(self.width, self.height);
        let black = Rgba::from_channels(0u8, 0u8, 0u8, 0u8);
        for x in 0..self.width {
            for y in 0..self.height {
                let ray = Ray::create_prime(x, y, self);
                let intersection = self.trace(&ray);
                let color = intersection
                    .map(|i| self.get_color(&ray, &i, DEPTH).to_rgba())
                    .unwrap_or(black);
                image.put_pixel(x, y, color);
            }
        }
        image
    }

    fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|e| e.intersect(ray).map(|d| Intersection::new(d, e)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }

    fn shade_diffuse(&self, element: &Element, hit_point: Point, surface_normal: Vector3) -> Color {
        let texture_coords = element.texture_coords(&hit_point);
        let mut color = BLACK;
        for light in &self.lights {
            let direction_to_light = light.direction_from(&hit_point);

            let shadow_ray = Ray {
                origin: hit_point + (surface_normal * self.shadow_bias),
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
            let material = element.material();
            let light_power =
                (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
            let light_reflected = material.albedo / PI;

            let light_color = light.color() * light_power * light_reflected;
            color = color + (material.coloration.color(&texture_coords) * light_color);
        }
        color.clamp()
    }

    fn get_color(&self, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
        let hit_point = ray.origin + (ray.direction * intersection.distance);
        let surface_normal = intersection.element.surface_normal(&hit_point);

        let mut color = self.shade_diffuse(intersection.element, hit_point, surface_normal);

        if let SurfaceType::Reflective { reflectivity } = intersection.element.material().surface {
            let reflection_ray =
                Ray::create_reflection(surface_normal, ray.direction, hit_point, self.shadow_bias);

            color = color * (1.0 - reflectivity);
            color = color + (self.cast_ray(&reflection_ray, depth + 1) * reflectivity);
        }
        color
    }
}
