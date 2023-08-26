use super::{brdf::specular, material::*};
pub struct Mirror; // perfect mirror
impl Material for Mirror {
    fn direct_shade(&self, color: Color, d_to_light: Vector3, normal: Vector3) -> Option<Color> {
        // if d_to_light.dot(normal) > 0.99 {
        //     Some(color)
        // } else {
        //     None
        // }
        None
    }
    fn indirect_shade(&self, color: Color) -> Option<Color> {
        Some(color)
    }
    fn reflect(&self, direction_in: Vector3, point: Vector3, normal: Vector3) -> Option<Ray3> {
        Some(Ray3::new(
            point,
            specular::sample_reflect_direction(direction_in, normal),
        ))
    }
}
