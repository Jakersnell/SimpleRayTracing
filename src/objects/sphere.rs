use crate::objects::object::*;
pub struct Sphere {
    center: Vector3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn boxed(center: Vector3, radius: f64) -> Box<Sphere> {
        Box::new(Sphere::new(center, radius))
    }

    pub fn point_is_on_surface(&self, point: &Vector3) -> bool {
        point.x * point.x + point.y * point.y + point.z * point.z == self.radius * self.radius
    }

    pub fn contains_point(&self, point: &Vector3) -> bool {
        point.x * point.x + point.y * point.y + point.z * point.z < self.radius * self.radius
    }

    pub fn random_on_surface(&self) -> Vector3 {
        let random_ray = Ray3::new(self.center, Vector3::random_in_unit_sphere());

        if let Some(distance) =
            self.check_intersect(&random_ray, f64::EPSILON, self.radius + f64::EPSILON)
        {
            return random_ray.at(distance);
        } else {
            self.center + self.center.normalized() * self.radius
        }
    }

    fn check_intersect(&self, ray: &Ray3, t_min: f64, t_max: f64) -> Option<f64> {
        /*
        Given a ray, calculates and returns the closest T where the ray intersects the object in 3D space,
        for the ray equation p(T) = A+BT,
        where A & B are vectors.
        */
        // get terms for quadratic.
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        // calculate.
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        Some(root)
        // return closest root that is in front of ray, and within parameters t_min & t_max,
        // if none exist, return None.
        // needs some way to tell if ray is inside object.
    }

    fn outward_normal(&self, point: &Vector3) -> Vector3 {
        Vector3::new(
            (point.x - self.center.x) / self.radius,
            (point.y - self.center.y) / self.radius,
            (point.z - self.center.z) / self.radius,
        )
        .normalized()
    }
}

impl Geometry for Sphere {
    fn check_intersect(&self, ray: &Ray3, t_min: f64, t_max: f64) -> Option<f64> {
        self.check_intersect(ray, t_min, t_max)
    }

    fn opposing_normal(&self, point: &Vector3, ray_direction: &Vector3) -> Vector3 {
        self.outward_normal(&point)
    }

    fn position(&self) -> Vector3 {
        self.center
    }
}
