pub(super) use crate::{
    materials::material::Material,
    math::{ray::Ray3, vector::Vector3},
};

// For volumetric objects, surface normals are standardized outward.
// For non-volumetric objects, surface normals are given towards incoming ray.
pub struct Object {
    pub geometry: Box<dyn Geometry>,
    pub material: Box<dyn Material>,
}

impl Object {
    pub fn new(geometry: Box<dyn Geometry>, material: Box<dyn Material>) -> Object {
        Object { geometry, material }
    }

    pub fn geometry(&self) -> &dyn Geometry {
        &*self.geometry
    }

    pub fn material(&self) -> &dyn Material {
        &*self.material
    }
}

pub trait Geometry {
    fn position(&self) -> Vector3;
    fn check_intersect(&self, ray: &Ray3, t_min: f64, t_max: f64) -> Option<f64>;
    fn opposing_normal(&self, point: &Vector3, ray_direction: &Vector3) -> Vector3;
}
