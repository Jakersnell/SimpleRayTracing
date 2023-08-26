use crate::{
    math::{ray::Ray3, vector::Vector3},
    rendercore::{render::trace_ray, world::World},
    utils::color::Color,
};

pub struct PointLight {
    pub position: Vector3,
    pub color: Color,
    pub intensity: f64,
}

impl PointLight {
    pub fn new(position: Vector3, color: Color, intensity: f64) -> PointLight {
        PointLight {
            position,
            color,
            intensity,
        }
    }
}
