use super::{ray::Ray3, vector::Vector3};

pub fn inv_sqr(x: f64) -> f64 {
    1.0 / (x.powi(2) + x + 1.0)
}

pub fn is_near_zero(x: f64) -> bool {
    if x > -f64::EPSILON && x < f64::EPSILON {
        true
    } else {
        false
    }
}
