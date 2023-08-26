use crate::math::vector::Vector3;

#[derive(Copy, Clone)]
pub struct Ray3 {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray3 {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray3 {
        Ray3 {
            origin,
            direction: direction.normalized(),
        }
    }

    pub fn origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn from_points(origin: &Vector3, endpoint: &Vector3) -> Ray3 {
        Ray3::new(*origin, *endpoint - *origin)
    }

    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 1e-10;

    fn approx_eq_vec3(v1: &Vector3, v2: &Vector3) -> bool {
        (v1.x - v2.x).abs() < EPSILON && (v1.y - v2.y).abs() < EPSILON && (v1.z - v2.z).abs() < EPSILON
    }

    #[test]
    fn test_ray3_new() {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(1.0, 2.0, 3.0);
        let ray = Ray3::new(origin, direction);

        assert_eq!(*ray.origin(), origin);
        assert!(approx_eq_vec3(ray.direction(), &direction.normalized()));
    }

    #[test]
    fn test_ray3_origin() {
        let origin = Vector3::new(0.0, 1.0, 2.0);
        let direction = Vector3::new(3.0, 4.0, 5.0);
        let ray = Ray3::new(origin, direction);

        assert_eq!(*ray.origin(), origin);
    }

    #[test]
    fn test_ray3_direction() {
        let origin = Vector3::new(0.0, 1.0, 2.0);
        let direction = Vector3::new(3.0, 4.0, 5.0);
        let ray = Ray3::new(origin, direction);

        assert!(approx_eq_vec3(ray.direction(), &direction.normalized()));
    }

    #[test]
    fn test_ray3_from_points() {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let endpoint = Vector3::new(1.0, 1.0, 1.0);
        let ray = Ray3::from_points(&origin, &endpoint);

        assert_eq!(*ray.origin(), origin);
        assert!(approx_eq_vec3(ray.direction(), &Vector3::new(1.0, 1.0, 1.0).normalized()));
    }

    #[test]
    fn test_ray3_at() {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let direction = Vector3::new(1.0, 2.0, 3.0);
        let ray = Ray3::new(origin, direction);

        let point_at_1 = ray.at(1.0);
        let expected_point = origin + direction.normalized();
        assert!(approx_eq_vec3(&point_at_1, &expected_point));
    }
}

