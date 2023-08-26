use std::sync::Arc;

use super::{camera::Camera, world::World};
use crate::{
    math::{
        utils::inv_sqr,
        {ray::Ray3, vector::Vector3},
    },
    objects::object::Object,
    utils::color::Color,
};
use rand::Rng;

pub struct RenderCore<'a> {
    camera: &'a Camera,
    world: &'a World,
    height: u32,
    width: u32,
    t_min: f64,
    t_max: f64,
}

impl<'a> RenderCore<'a> {
    pub fn new(
        camera: &'a Camera,
        world: &'a World,
        height: u32,
        width: u32,
        t_min: f64,
        t_max: f64,
    ) -> Self {
        Self {
            camera,
            world,
            height,
            width,
            t_min,
            t_max,
        }
    }

    pub fn render(&self, max_depth: u8, samples: u32) -> Vec<u8> {
        let mut output = Vec::new();
        // Iterate over pixels, starting at the bottom left cornere of the "screen".
        for x in 0..self.camera.image_width {
            // Which orientation of x & y iteration is best, is this causing pixel inversion?
            for y in 0..self.camera.image_height {
                // Monte carlo method sampling for ray effects on current pixel.
                let mut colors = Vec::new();
                for _sample_ in 0..samples {
                    let ray = self.camera.gen_ray(x, y);
                    if let Some(color) = self.render_ray(ray, max_depth, samples) {
                        colors.push(color);
                    }
                }
                // Average samples together, push bytes.
                Color::average(colors).out().map(|byte| output.push(byte));
            }
        }
        output
    }

    pub fn render_ray(&self, ray: Ray3, depth: u8, samples: u32) -> Option<Color> {
        let mut result_color: Option<Color> = None;
        if depth != 0 {
            if let Some((object, point, normal, distance)) = self.trace_ray(&ray) {
                let mut return_colors = Vec::new();
                return_colors.append(&mut self.direct_shading(object, point, normal));
                return_colors
                    .append(&mut self.indirect_shading(&ray, object, point, normal, depth));
                // averaging colors
                if !return_colors.is_empty() {
                    result_color = Some(Color::average(return_colors));
                }
            }
        }
        result_color
    }

    fn direct_shading(&self, object: &Object, point: Vector3, normal: Vector3) -> Vec<Color> {
        let mut return_colors = Vec::new();
        for light in self.world.lights() {
            let distance = light.position.distance_to(point);
            let shadow_ray = Ray3::new(light.position, light.position - point);
            if trace_ray(&shadow_ray, self.world, self.t_min, distance).is_none() {
                let light_color = light.color * light.intensity * inv_sqr(distance);
                object
                    .material
                    .direct_shade(light_color, shadow_ray.direction, normal)
                    .map(|shaded_color| return_colors.push(shaded_color));
            }
        }
        return_colors
    }

    fn indirect_shading(
        &self,
        ray: &Ray3,
        object: &Object,
        point: Vector3,
        normal: Vector3,
        depth: u8,
    ) -> Vec<Color> {
        let mut return_colors = Vec::new();
        object
            .material()
            .reflect(ray.direction, point, normal)
            .map(|out_ray| {
                render_ray(out_ray, self.world, self.t_min, self.t_max, depth - 1).map(|color| {
                    object
                        .material
                        .indirect_shade(color)
                        .map(|shaded_color| return_colors.push(shaded_color))
                });
            });
        return_colors
    }

    pub fn trace_ray(&self, ray: &Ray3) -> Option<(&'a Object, Vector3, Vector3, f64)> {
        let mut distance = self.t_max;
        let mut result: Option<(&'a Object, Vector3, Vector3, f64)> = None;

        for object in self.world.objects() {
            if let Some(t) = object.geometry().check_intersect(ray, self.t_min, distance) {
                let point = ray.at(t);
                let normal = object.geometry().opposing_normal(&point, &ray.direction);
                distance = ray.origin.distance_to(point);
                assert_eq!(distance, t);
                result = Some((&object, point, normal, distance))
            }
        }
        result
    }
}

pub fn render_pixels(
    camera: &Camera,
    world: &World,
    t_min: f64,
    t_max: f64,
    samples: u64,
    depth: u8,
) -> Vec<[u8; 3]> {
    let mut output = Vec::new();
    for x in 0..camera.image_width {
        for y in 0..camera.image_height {
            let mut colors = Vec::new();
            for _sample_ in 0..samples {
                let ray = camera.gen_ray(x, y);
                if let Some(color) = render_ray(ray, world, t_min, t_max, depth) {
                    colors.push(color);
                }
            }
            output.push(Color::average(colors).out());
        }
    }
    output
}

fn render_ray(ray: Ray3, world: &World, t_min: f64, t_max: f64, depth: u8) -> Option<Color> {
    let mut result_color: Option<Color> = None;
    if depth != 0 {
        if let Some((object, point, normal, distance)) = trace_ray(&ray, &world, t_min, t_max) {
            let mut return_colors = Vec::new();
            // direct shading
            for light in world.lights() {
                let distance = light.position.distance_to(point);
                let shadow_ray = Ray3::new(light.position, light.position - point);
                if trace_ray(&shadow_ray, world, t_min, distance).is_none() {
                    let light_color = light.color * light.intensity * inv_sqr(distance);
                    object
                        .material
                        .direct_shade(light_color, shadow_ray.direction, normal)
                        .map(|shaded_color| return_colors.push(shaded_color));
                }
            }
            // indirect shading
            object
                .material()
                .reflect(ray.direction, point, normal)
                .map(|out_ray| {
                    render_ray(out_ray, world, t_min, t_max, depth - 1).map(|color| {
                        object
                            .material
                            .indirect_shade(color)
                            .map(|shaded_color| return_colors.push(shaded_color))
                    });
                });
            // averaging colors
            if !return_colors.is_empty() {
                result_color = Some(Color::average(return_colors));
            }
        }
    }
    result_color
}

pub fn trace_ray<'a>(
    ray: &Ray3,
    world: &'a World,
    t_min: f64,
    t_max: f64,
) -> Option<(&'a Object, Vector3, Vector3, f64)> {
    let mut distance = t_max;
    let mut result: Option<(&'a Object, Vector3, Vector3, f64)> = None;

    for object in world.objects() {
        if let Some(t) = object.geometry().check_intersect(ray, t_min, distance) {
            let point = ray.at(t);
            let normal = object.geometry().opposing_normal(&point, &ray.direction);
            distance = ray.origin.distance_to(point);
            result = Some((&object, point, normal, distance))
        }
    }

    result
}
