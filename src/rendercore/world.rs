use crate::{
    lights::pointlight::PointLight, materials::material::Material, math::vector::Vector3,
    objects::object::Object, utils::color::Color,
};
use rand::Rng;
use std::rc::Rc;

pub struct World {
    objects: Vec<Object>,
    lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn objects(&self) -> &Vec<Object> {
        &self.objects
    }

    pub fn lights(&self) -> &Vec<PointLight> {
        &self.lights
    }

    pub fn add_object(&mut self, object: Object) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }
}
