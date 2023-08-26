use crate::{
    math::{ray::Ray3, vector::Vector3},
    utils::color::Color,
};
pub fn sample_reflect_direction(direction_in: Vector3, normal: Vector3) -> Vector3 {
    // R = 2(N dot I)N - I
    direction_in - normal * normal.dot(direction_in) * 2.0
}
pub fn eval(color: Color, reflectivity: f64) -> Color {
    color * reflectivity.clamp(0.0, 1.0)
}
pub fn pdf(normal: Vector3, wi: Vector3) -> f64 {
    normal.dot(wi) * (1.0 / std::f64::consts::PI)
}
