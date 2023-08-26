use super::object::*;
pub struct Triangle {
    v1: Vector3,
    v2: Vector3,
    v3: Vector3,
    a: Vector3,
    b: Vector3,
    normal: Vector3,
}

impl Triangle {
    pub fn new(v1: Vector3, v2: Vector3, v3: Vector3) -> Triangle {
        let a = v3 - v1;
        let b = v2 - v1;
        let normal = a.cross(b);

        Triangle {
            v1,
            v2,
            v3,
            a,
            b,
            normal,
        }
    }
    pub fn check_intersect(&self, ray: &Ray3, t_min: f64, t_max: f64) -> Option<f64> {
        None
    }
}
