use crate::entities::{Point, Scene};
use vek::Vec3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3<f64>,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        Ray {
            origin: Point::zero(),
            direction: Vec3::zero(),
        }
    }
}

pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
    let sensor_y = 1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0;
    // take non-quadratic images into account
    let aspect_ratio = (scene.width as f64) / (scene.height as f64);
    let sensor_x = if aspect_ratio == 1.0 {
        ((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0
    } else {
        ((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0 * aspect_ratio
    };

    Ray {
        origin: Point::zero(),
        direction: Vec3 {
            x: sensor_x,
            y: sensor_y,
            z: -1.0,
        },
    }
}
