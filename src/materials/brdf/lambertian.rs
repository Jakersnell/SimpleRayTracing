use crate::{
    math::{ray::Ray3, utils::is_near_zero, vector::Vector3},
    utils::color::Color,
};
pub fn sample_reflect_direction(normal: Vector3, point: Vector3) -> Vector3 {
    let mut random_vec = Vector3::random_in_unit_sphere();
    let theta = normal.dot(random_vec);
    if is_near_zero(theta) {
        random_vec = normal;
    } else if theta < 0.0 - f64::EPSILON {
        random_vec = -random_vec
    }
    random_vec
}
pub fn eval(color: Color, albedo: f64) -> Color {
    color * albedo / std::f64::consts::PI
}
pub fn pdf(normal: Vector3, w_i: Vector3) -> f64 {
    normal.dot(w_i) * (1. / std::f64::consts::PI)
}
