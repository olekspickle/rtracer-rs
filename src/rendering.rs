use crate::{
    entities::{Color, Element, Plane, Sphere},
    point::Point,
    scene::Scene,
    vector::Vector3,
};
use std::f32::consts::PI;

pub const BLACK: Color = Color {
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
            }
            .normalize(),
        }
    }

    /// The more interesting question here is how to compute the reflection ray.
    /// If you’ve taken physics, you may remember the mantra that
    /// the angle of incidence equals the angle of reflection.
    /// That’s helpful enough as far as it goes, but how do we actually calculate that in terms of vectors?
    /// Reflection Ray
    /// We can separate the incident vector I into two vectors, A and B (see figure) such that I = A + B.
    /// The reflection vector R is then equal to A - B.
    /// I = A + B
    /// R = A - B
    ///
    /// We can compute B quite easily - it’s the projection of I onto the surface normal,
    /// or the dot product of I and N multiplied by N.
    /// B = (I.N)N
    ///
    /// Substitute that into both equations:
    /// I = A + (I.N)N
    /// R = A - (I.N)N
    ///
    /// Then solve the first equation for A:
    /// A = I - (I.N)N
    ///
    /// And substitute into the second equation:
    /// R = I - (I.N)N - (I.N)N
    /// R = I - 2(I.N)N
    pub fn create_reflection(
        normal: Vector3,
        incident: Vector3,
        intersection: Point,
        bias: f64,
    ) -> Ray {
        Ray {
            origin: intersection + (normal * bias),
            direction: incident - 2.0 * (incident.dot(&normal) * normal),
        }
    }

    //The transparency is the same as the reflectivity from before - the fraction of the final color that comes from refraction. Refraction is governed by a parameter called the index of refraction. When a ray of light passes from one transparent substance to another, it bends at an angle described by Snell’s Law:
    // Snell's Law
    // sin(theta_i)/sin(theta_t) = eta_t/eta_i
    //
    // Where theta_i and theta_t are the angle of incidence and angle of transmission, and eta_i and eta_t are the indices of refraction for the incident substance and the transmitting substance. We could calculate the angle of transmission using this equation, but we’ll need to do more to convert that angle into a vector.
    // As with reflection, refraction is really a two-dimensional process in the plane formed by the incident vector and the surface normal. This means that we can think of our transmitted ray as having a horizontal component (A) and vertical component (B). B is relatively simple to calculate:
    // B = cos(theta_t) * -N
    //
    // This makes some intuitive sense - the transmitted ray will be on the opposite side of the surface from the incident ray, so it’s vertical component will be some fraction of the inverse of the surface normal. We use the cosine of the transmission angle because that’s how you calculate the vertical distance.
    // We can use this same approach to get the horizontal component A, but first we need to construct a horizontal unit vector (M). To do this, we first take the incident vector and subtract it’s vertical component, leaving only the horizontal component. We can calculate the vertical component of I easily - it’s (I.N)N, just like before. Then we normalize this horizontal vector to get the horizontal unit vector we need. We can slightly cheat here, though - the length of the horizontal component of I will be equal to sin(theta_i), so we can normalize using that instead of computing the vector length the slow way.
    // M = (I - -N(I.N)) / sin(theta_i) = (I + N(I.N)) / sin(theta_i)
    // A = sin(theta_t) * M
    // B = cos(theta_t) * -N
    //
    // Putting this all back together, we get:
    // T = A + B
    // T = (sin(theta_t) * M) - N * cos(theta_t)
    // T = (sin(theta_t) * (I + N(I.N)) / sin(theta_i)) - N * cos(theta_t)
    //
    // We can use Snell’s Law to replace that sin(theta_t) / sin(theta_i) with eta_i/eta_t, like so:
    // T = (I + N(I.N)) * eta_i/eta_t - N * cos(theta_t)
    //
    // We could calculate cos(theta_t) from Snell’s Law and theta_i, but this involves lots of trigonometry, and ain’t nobody got time for that. Instead, we can express that in terms of a dot-product. We know from trigonometry that:
    // cos^2(theta_t) + sin^2(theta_t) = 1
    // cos(theta_t) = sqrt(1 - sin^2(theta_t))
    //
    // And from Snell’s Law we know that:
    // sin(theta_t) = (eta_i/eta_t) * sin(theta_i)
    //
    // Therefore:
    // cos(theta_t) = sqrt( 1 - (eta_i/eta_t)^2 * sin^2(theta_1) )
    //
    // Then we can use the same trigonometric identity from above to convert that sin to a cosine:
    // cos(theta_t) = sqrt( 1 - (eta_i/eta_t)^2 * (1 - cos^2(theta_i)) )
    //
    // And since cos(theta_i) = I.N, we get:
    // cos(theta_t) = sqrt( 1 - (eta_i/eta_t)^2 * (1 - I.N^2) )
    //
    // And so, finally, we arrive at this monster of an equation (but look, no trigonometry):
    // T = (I + N(I.N)) * eta_i/eta_t - N * sqrt( 1 - (eta_i/eta_t)^2 * (1 - I.N^2) )
    //
    // Now, there are a couple of wrinkles left to sort out. First, sometimes our ray will be leaving the transparent object rather than entering it. This is easy enough to handle, just invert the normal and swap the indices of refraction. We also need to handle total internal reflection. In some cases, if the angle of incidence is shallow enough, the refracted light ray actually reflects off the surface instead of passing through and travels back into the object. We can detect this when the term inside the sqrt is negative. Again, this makes intuitive sense - if that’s negative, the vertical component of the transmission vector would be positive (remember, B is a multiple of -N) and therefore on the same side of the surface as the incident vector. In fact, however, we can handle this by completely ignoring it, and I’ll explain why later.
    // Whew! Now that we have that giant equation, we can implement it in code, like so:
    pub fn create_transmission(
        normal: Vector3,
        incident: Vector3,
        intersection: Point,
        bias: f64,
        index: f32,
    ) -> Option<Ray> {
        let mut ref_n = normal;
        let mut eta_t = index as f64;
        let mut eta_i = 1.0;
        let mut i_dot_n = incident.dot(&normal);
        if i_dot_n < 0.0 {
            //Outside the surface
            i_dot_n = -i_dot_n;
        } else {
            //Inside the surface; invert the normal and swap the indices of refraction
            ref_n = -normal;
            eta_i = eta_t;
            eta_t = 1.0;
        }

        let eta = eta_i / eta_t;
        let k = 1.0 - (eta * eta) * (1.0 - i_dot_n * i_dot_n);
        if k < 0.0 {
            None
        } else {
            Some(Ray {
                origin: intersection + (ref_n * -bias),
                direction: (incident + i_dot_n * ref_n) * eta - ref_n * k.sqrt(),
            })
        }
    }
}

pub struct TextureCoords {
    pub x: f32,
    pub y: f32,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;

    fn surface_normal(&self, hit_point: &Point) -> Vector3;
    fn texture_coords(&self, hit_point: &Point) -> TextureCoords;
}

impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }
    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Element::Sphere(ref s) => s.surface_normal(hit_point),
            Element::Plane(ref p) => p.surface_normal(hit_point),
        }
    }
    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        match *self {
            Element::Sphere(ref s) => s.texture_coords(hit_point),
            Element::Plane(ref p) => p.texture_coords(hit_point),
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let distance = v.dot(normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }
    fn surface_normal(&self, _: &Point) -> Vector3 {
        -self.normal.normalize()
    }
    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        if x_axis.length() == 0.0 {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
        }
        let y_axis = self.normal.cross(&x_axis);
        let hit_vec = *hit_point - self.origin;
        TextureCoords {
            x: hit_vec.dot(&x_axis) as f32,
            y: hit_vec.dot(&y_axis) as f32,
        }
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let l: Vector3 = self.center - ray.origin;
        let adj = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;

        if t0 < 0.0 && t1 < 0.0 {
            None
        } else if t0 < 0.0 {
            Some(t1)
        } else if t1 < 0.0 {
            Some(t0)
        } else {
            let distance = if t0 < t1 { t0 } else { t1 };
            Some(distance)
        }
    }
    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }
    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let hit_vec = *hit_point - self.center;
        TextureCoords {
            x: (1.0 + (hit_vec.z.atan2(hit_vec.x) as f32) / PI) * 0.5,
            y: (hit_vec.y / self.radius).acos() as f32 / PI,
        }
    }
}
