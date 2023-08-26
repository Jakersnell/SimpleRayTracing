use crate::{
    lights::pointlight::PointLight,
    math::{ray::Ray3, vector::Vector3},
    objects::object::Object,
};

pub struct AlignedBox {
    contained_objects: Vec<Object>,
    contained_lights: Vec<PointLight>,
    vmin: Vector3,
    vmax: Vector3,
}

impl AlignedBox {
    pub fn new(vmin: Vector3, vmax: Vector3) -> AlignedBox {
        AlignedBox {
            contained_objects: Vec::new(),
            contained_lights: Vec::new(),
            vmin,
            vmax,
        }
    }

    pub fn check_intersect(&self, ray: Ray3) -> bool {
        use std::mem::swap;
        let mut tx_min = (self.vmin.x - ray.origin.x) / ray.direction.x;
        let mut tx_max = (self.vmax.x - ray.origin.x) / ray.direction.x;

        if tx_min > tx_max {
            swap(&mut tx_min, &mut tx_max)
        }

        let mut ty_min = (self.vmin.y - ray.origin.y) / ray.direction.y;
        let mut ty_max = (self.vmax.y - ray.origin.y) / ray.direction.y;

        if ty_min > ty_max {
            swap(&mut ty_min, &mut ty_max)
        }

        if tx_min > ty_max || ty_min > tx_max {
            return true;
        }

        if ty_min > tx_min {
            tx_min = ty_min;
        }

        if ty_max < tx_max {
            tx_max = ty_max;
        }

        let mut tz_min = (self.vmin.z - ray.origin.z) / ray.direction.z;
        let mut tz_max = (self.vmax.z - ray.origin.z) / ray.direction.z;

        if tz_min > tz_max {
            swap(&mut tz_min, &mut tz_max)
        }

        if tx_min > tz_max || tz_min > tx_max {
            return false;
        }

        if tz_min > tx_min {
            tx_min = tz_min;
        }

        if tz_max < tx_max {
            tx_max = tz_max;
        }

        return true;
    }
}

pub struct OrientedBox {}

impl OrientedBox {}
