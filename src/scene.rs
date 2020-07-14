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
        let hit = ray.origin + (ray.direction * intersection.distance);
        let normal = intersection.element.surface_normal(&hit);

        let material = intersection.element.material();
        match material.surface {
            SurfaceType::Diffuse => self.shade_diffuse(intersection.element, hit, normal),
            SurfaceType::Reflective { reflectivity } => {
                let mut color = self.shade_diffuse(intersection.element, hit, normal);
                let reflection_ray =
                    Ray::create_reflection(normal, ray.direction, hit, self.shadow_bias);
                color = color * (1.0 - reflectivity);
                color = color + (self.cast_ray(&reflection_ray, depth + 1) * reflectivity);
                color
            }
            SurfaceType::Refractive {
                index,
                transparency,
            } => {
                let mut refraction_color = BLACK;
                let kr = self.fresnel(ray.direction, normal, index) as f32;
                let surface_color = material
                    .coloration
                    .color(&intersection.element.texture_coords(&hit));

                if kr < 1.0 {
                    let transmission_ray = Ray::create_transmission(
                        normal,
                        ray.direction,
                        hit,
                        self.shadow_bias,
                        index,
                    )
                    .unwrap();
                    refraction_color = self.cast_ray(&transmission_ray, depth + 1);
                }

                let reflection_ray =
                    Ray::create_reflection(normal, ray.direction, hit, self.shadow_bias);
                let reflection_color = self.cast_ray(&reflection_ray, depth + 1);
                let mut color = reflection_color * kr + refraction_color * (1.0 - kr);
                color = color * transparency * surface_color;
                color
            }
        }
    }
    fn fresnel(&self, incident: Vector3, normal: Vector3, index: f32) -> f64 {
        let i_dot_n = incident.dot(&normal);
        let mut eta_i = 1.0;
        let mut eta_t = index as f64;
        if i_dot_n > 0.0 {
            eta_i = eta_t;
            eta_t = 1.0;
        }
        let sin_t = eta_i / eta_t * (1.0 - i_dot_n * i_dot_n).max(0.0).sqrt();
        if sin_t > 1.0 {
            //Total internal reflection
            return 1.0;
        } else {
            let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
            let cos_i = cos_t.abs();
            let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
            let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
            return (r_s * r_s + r_p * r_p) / 2.0;
        }
    }
}
