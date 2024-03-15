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


