#![allow(unused)]

use std::{fs::File, io::BufWriter};

use image::{ImageBuffer, ImageOutputFormat, Rgb, RgbImage};
use lights::pointlight::PointLight;
use materials::{matte::Matte, mirror::Mirror};
use math::vector::Vector3;
use objects::{object::Object, sphere::Sphere};
use rendercore::{camera::Camera, world::World};
use utils::color::Color;

use crate::objects::plane::Plane;

pub mod rendercore {
    pub mod camera;
    pub mod render;
    pub mod world;
}
pub mod lights {
    pub mod lightsource;
    pub mod pointlight;
}
pub mod materials {
    pub mod material;
    pub mod matte;
    pub mod mirror;
    pub mod brdf {
        pub mod lambertian;
        pub mod specular;
    }
}
pub mod math {
    pub mod ray;
    pub mod utils;
    pub mod vector;
}
pub mod objects {
    pub mod object;
    pub mod plane;
    pub mod sphere;
    pub mod triangle;
}
pub mod utils {
    pub mod interaction;
    pub mod boundingbox;
    pub mod color;
}

fn main() -> std::io::Result<()> {
    // Instantiate settings.
    const HEIGHT: u32 = 800;
    const WIDTH: u32 = 800;
    const SAMPLES: u64 = 1;
    const MAX_DEPTH: u8 = 2;
    let filename = "test.png".to_owned();
    // Initilize constructs.
    let mut world = World::new();
    let mut camera = Camera::new(HEIGHT, WIDTH, 1.0, 1.0, 1.0);
    // Initialize physical objects.

    // let test_wall = Object::new(
    //     Box::new(Plane::new(
    //         Vector3::new(0.0, 0.0, -10.0),
    //         Vector3::new(0.0, 0.0, -1.0)
    //     )),
    //     Box::new(Matte::new(Color::new(1.0, 0.0, 0.0), 1.0)),
    // );
    // world.add_object(test_wall);

    let right_wall = Object::new(
        Box::new(Plane::new(
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(1.0, 0.0, 0.0),
        )),
        Box::new(Matte::new(Color::new(0.2, 0.4, 0.8), 0.3)),
    );
    world.add_object(right_wall);

    let left_wall = Object::new(
        Box::new(Plane::new(
            Vector3::new(-1.0, 0.0, 0.0),
            Vector3::new(-1.0, 0.0, 0.0),
        )),
        Box::new(Matte::new(Color::new(0.8, 0.6, 0.2), 0.3)),
    );
    world.add_object(left_wall);

    let back_wall = Object::new(
        Box::new(Plane::new(
            Vector3::new(0.0, 0.0, -3.0),
            Vector3::new(0.0, 0.0, -1.0),
        )),
        Box::new(Matte::new(Color::new(0.8, 0.5, 0.5), 0.3)),
    );
    world.add_object(back_wall);

    // let ceiling = Object::new( // causes undefined behavior.
    //     Box::new(Plane::new(
    //         Vector3::new(0.0, -1.4, 0.0),
    //         Vector3::new(0.0, -1.0, 0.0),
    //     )),
    //     Box::new(Matte::new(Color::new(0.32, 0.85, 0.234), 0.3)),
    // );
    // world.add_object(ceiling);

    // let floor = Object::new(
    //     Box::new(Plane::new(
    //         Vector3::new(0.0, 2.0, 0.0),
    //         Vector3::new(0.0, 1.0, 0.0),
    //     )),
    //     Box::new(Matte::new(Color::new(0.235, 0.584, 0.238), 0.3)),
    // );
    // world.add_object(floor);

    // let front_wall = Object::new( // this causes undefined behavior.
    //     Box::new(Plane::new(
    //         Vector3::new(0.0, 0.0, 2.0),
    //         Vector3::new(0., 0.0, 1.0),
    //     )),
    //     Box::new(Matte::new(Color::new(0.8, 0.2, 0.0), 0.3)),
    // );
    // world.add_object(front_wall);

    let mirror_sphere = Object::new(
        Box::new(Sphere::new(Vector3::new(-0.5, 0.2, -1.0), 0.5)),
        Box::new(Mirror),
    );
    world.add_object(mirror_sphere);

    let matte_blue_sphere = Object::new(
        Box::new(Sphere::new(Vector3::new(0.4, -0.4, -1.2), 0.25)),
        Box::new(Matte::new(Color::new(0.2, 0.3, 0.8), 0.3)),
    );
    world.add_object(matte_blue_sphere);

    let matte_green_sphere = Object::new(
        Box::new(Sphere::new(Vector3::new(0.6, 0.4, -1.4), 0.3)),
        Box::new(Matte::new(Color::new(0.2, 0.99, 0.1), 0.3)),
    );
    world.add_object(matte_green_sphere);

    let matte_red_sphere = Object::new(
        Box::new(Sphere::new(Vector3::new(-0.1, -0.4, -2.0), 0.2)),
        Box::new(Matte::new(Color::new(0.9, 0.3, 0.0), 0.3)),
    );
    world.add_object(matte_red_sphere);

    // Initialize lights.
    let pointlight = PointLight::new(Vector3::new(0.0, -0.8, 1.0), Color::by_name("white"), 400.0);
    world.add_light(pointlight);
    // Given camera and world, render the scene, return render as Vec<[u8;3]>.
    let byte_buf = rendercore::render::render_pixels(
        &camera,
        &world,
        f64::EPSILON,
        f64::MAX,
        SAMPLES,
        MAX_DEPTH,
    );
    // Iterate over pixels, write raw pixel to each pixel.
    let mut pixel_iter = byte_buf.into_iter();
    let mut image = RgbImage::new(WIDTH, HEIGHT);
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if let Some(pixel) = pixel_iter.next() {
                image.put_pixel(x, y, Rgb::from(pixel))
            } else {
                panic!()
            }
        }
    }
    // Create output filestream.
    let file = File::create(format!("output/{}", filename))?;
    // Write to image
    image.write_to(&mut BufWriter::new(file), ImageOutputFormat::Png);
    Ok(())
}
