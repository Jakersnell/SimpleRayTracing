use crate::{
    math::{
        ray::Ray3,
        vector::Vector3
    },
    lights::lightsource::LightSource,
    objects::object::Object
};

pub struct ObjectInteraction<'a> {
    object: &'a Object,
    surface_normal: Vector3,
    incident_point: Vector3,
    in_ray: Ray3,
    distance: f64,

}

impl <'a> ObjectInteraction<'a> {
    pub fn new(object: &'a Object, surface_normal: Vector3, incident_point: Vector3, in_ray: Ray3, distance: f64) -> ObjectInteraction<'a> {
        ObjectInteraction {
            object,
            surface_normal,
            incident_point,
            in_ray,
            distance
        }
    } 

    pub fn object(&self) -> &'a Object {
        self.object
    }

    pub fn surface_normal(&self) -> &Vector3 {
        &self.surface_normal
    }

    pub fn incident_point(&self) -> &Vector3 {
        &self.incident_point
    }

    pub fn in_ray(&self) -> &Ray3 {
        &self.in_ray
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}


pub struct LightInteraction<'a> {
    lightsource: &'a dyn LightSource,
    incident_point: Vector3,
    in_ray: Ray3,
    distance: f64,

}

impl <'a> LightInteraction<'a> {
    pub fn new(lightsource: &'a dyn LightSource, incident_point: Vector3, in_ray: Ray3, distance: f64) -> LightInteraction<'a> {
        LightInteraction {
            lightsource,
            incident_point,
            in_ray,
            distance
        }
    } 

    pub fn lightsource(&self) -> &'a dyn LightSource {
        self.lightsource
    }

    pub fn incident_point(&self) -> &Vector3 {
        &self.incident_point
    }

    pub fn in_ray(&self) -> &Ray3 {
        &self.in_ray
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}


// depreciated 

pub struct Interaction<'a, T> {
    object: &'a T,
    surface_normal: Vector3,
    incident_point: Vector3,
    in_ray: Ray3,
    distance: f64,

}

impl <'a, T> Interaction<'a, T> {
    pub fn new(object: &'a T, surface_normal: Vector3, incident_point: Vector3, in_ray: Ray3, distance: f64) -> Interaction<'a, T> {
        Interaction {
            object,
            surface_normal,
            incident_point,
            in_ray,
            distance
        }
    } 

    pub fn object(&self) -> &'a T {
        self.object
    }

    pub fn surface_normal(&self) -> &Vector3 {
        &self.surface_normal
    }

    pub fn incident_point(&self) -> &Vector3 {
        &self.incident_point
    }

    pub fn in_ray(&self) -> &Ray3 {
        &self.in_ray
    }

    pub fn distance(&self) -> f64 {
        self.distance
    }
}

impl <'a, Object> Interaction<'a, Object> {
    pub fn reflect(&self) -> Option<Ray3> {
        todo!()
    }
}

impl <'a, LightSource> Interaction<'a, LightSource> {
    pub fn unblocked_emmitance(&self) {
        todo!()
    }
}