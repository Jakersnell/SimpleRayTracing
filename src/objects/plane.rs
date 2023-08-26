use crate::math::vector::Vector2;

use super::object::*;
pub struct Plane {
    origin: Vector3,
    normal: Vector3,
}

impl Plane {
    pub fn new(origin: Vector3, normal: Vector3) -> Plane {
        Plane { origin, normal }
    }
    /// Intersection test for the given Plane and Ray.
    pub fn check_intersect(&self, ray: &Ray3, t_min: f64, t_max: f64) -> Option<f64> {
        let mut result = None;
        let denom = self.normal.dot(ray.direction);
        if denom > 1e-6 {
            let origins = self.origin - ray.origin;
            let t = origins.dot(self.normal) / denom;
            if t > t_min && t < t_max {
                result = Some(t)
            };
        }
        result
    }

    /// Per default all normals are given in the direction opposing the incoming ray.
    pub fn opposing_surface_normal(&self, ray_direction: &Vector3) -> Vector3 {
        self.normal.neg()
    }
}

impl Geometry for Plane {
    fn check_intersect(&self, ray: &Ray3, t_min: f64, t_max: f64) -> Option<f64> {
        self.check_intersect(ray, t_min, t_max)
    }

    fn opposing_normal(&self, point: &Vector3, ray_direction: &Vector3) -> Vector3 {
        self.opposing_surface_normal(ray_direction)
    }

    fn position(&self) -> Vector3 {
        self.origin
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_intersect() {
        let test_plane = Plane::new(Vector3::new(0.0, 0.0, -10.0), Vector3::new(0.0, 0.0, -1.0));
        let test_ray = Ray3::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -1.0));
        let test_t = 10.0;

        assert_eq!(
            test_plane
                .check_intersect(&test_ray, 0.0, f64::MAX)
                .unwrap(),
            test_t
        );
    }
}
