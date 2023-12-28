#![allow(warnings)]
mod engine;

use std::sync::Arc;
use std::time::{Duration, Instant};

use engine::camera::*;
use engine::hittable::*;
use engine::ray::*;

use env_logger;

use crate::engine::color::Color;
use crate::engine::material::*;

use glam::Vec3;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Materials
    let material_ground = Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };

    let material_center = Lambertian {
        albedo: Color::new(0.2, 0.2, 0.2),
    };

    let material_left = Dielectric { ir: 0.1 };

    let material_right = Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(
        &Point::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(material_center),
    )));
    world.add(Box::new(Sphere::new(
        &Point::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(material_left),
    )));
    world.add(Box::new(Sphere::new(
        &Point::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(material_right),
    )));
    world.add(Box::new(Sphere::new(
        &Point::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(material_ground),
    )));

    let mut camera = Camera::new(16.0 / 9.0, 1650, 100, 50);
    let start = Instant::now();
    camera.render(Arc::new(world)).await;
    let duration = start.elapsed();
    println!("Time elapsed in camera.render() is: {:?}", duration);
}
