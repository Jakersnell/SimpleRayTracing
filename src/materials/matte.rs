use super::{brdf::lambertian, material::*};

pub struct Matte {
    pub color: Color,
    pub albedo: f64,
}

impl Matte {
    pub fn new(color: Color, albedo: f64) -> Matte {
        Matte {
            color: color.clamp(),
            albedo: albedo.clamp(0.0, 1.0),
        }
    }
}

impl Material for Matte {
    fn reflect(&self, in_direction: Vector3, normal: Vector3, point: Vector3) -> Option<Ray3> {
        let mut random_direction = Vector3::random_in_unit_sphere();
        if normal.dot(random_direction) < 0.0 {
            random_direction = -random_direction;
        }
        Some(Ray3::new(
            point + normal * f64::EPSILON,
            random_direction + normal,
        ))
    }

    fn direct_shade(&self, color: Color, d_to_light: Vector3, normal: Vector3) -> Option<Color> {
        Some(self.color * lambertian::eval(color, self.albedo) * d_to_light.dot(normal).max(0.0))
    }

    fn indirect_shade(&self, color: Color) -> Option<Color> {
        Some(self.color * lambertian::eval(color, self.albedo))
    }
}
