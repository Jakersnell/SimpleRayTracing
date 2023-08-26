use super::world::World;
use crate::{
    math::{ray::Ray3, vector::Vector3},
    utils::color::Color,
};
use rand::Rng;

pub struct Camera {
    pub image_height: u32,
    pub image_width: u32,
    anti_aliasing: bool,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
    lower_left: Vector3,
}

impl Camera {
    pub fn new(
        image_height: u32,
        image_width: u32,
        viewport_height: f64,
        viewport_width: f64,
        focal_length: f64,
    ) -> Camera {
        let origin = Vector3::new(0.0, 0.0, 0.0);
        let horizontal = Vector3::new(viewport_width, 0.0, 0.0);
        let vertical = Vector3::new(0.0, viewport_height, 0.0);
        let lower_left =
            origin - (horizontal / 2.0) - (vertical / 2.0) - Vector3::new(0.0, 0.0, focal_length);

        Camera {
            image_height,
            image_width,
            anti_aliasing: true,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left,
        }
    }

    pub fn gen_ray(&self, x: u32, y: u32) -> Ray3 {
        let j = self.viewport_height
            * ((y as f64 + (rand::thread_rng().gen::<f64>()) / 2.0)
                / (self.image_height - 1) as f64);

        let k = self.viewport_width
            * ((x as f64 + (rand::thread_rng().gen::<f64>()) / 2.0)
                / (self.image_width - 1) as f64);

        Ray3::new(
            self.origin,
            self.lower_left + Vector3::new(k, 0.0, 0.0) + Vector3::new(0.0, j, 0.0) - self.origin,
        )
    }

    pub fn viewport_height(&self) -> f64 {
        self.viewport_height
    }

    pub fn viewport_width(&self) -> f64 {
        self.viewport_width
    }

    pub fn focal_length(&self) -> f64 {
        self.focal_length
    }

    pub fn origin(&self) -> &Vector3 {
        &self.origin
    }

    pub fn horizontal(&self) -> &Vector3 {
        &self.horizontal
    }

    pub fn vertical(&self) -> &Vector3 {
        &self.vertical
    }

    pub fn lower_left(&self) -> &Vector3 {
        &self.lower_left
    }
}

fn anti_alias(given_direction: Vector3, cam_direction: Vector3) -> Vector3 {
    Vector3::new(0.0, 0.0, 0.0)
}
