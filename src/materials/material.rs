pub(super) use crate::{
    math::{ray::Ray3, utils::inv_sqr, vector::Vector3},
    rendercore::world::World,
    utils::color::Color,
};

pub trait Material {
    fn reflect(&self, in_direction: Vector3, point: Vector3, normal: Vector3) -> Option<Ray3>;
    fn direct_shade(&self, color: Color, d_to_light: Vector3, normal: Vector3) -> Option<Color>;
    fn indirect_shade(&self, color: Color) -> Option<Color>;
}
