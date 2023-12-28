mod engine;

use std::sync::Arc;

use engine::camera::*;
use engine::hittable::*;
use engine::ray::*;

use env_logger;

#[tokio::main]
async fn main() {
    env_logger::init();

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(&Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(&Point::new(0.0, -100.5, -1.0), 100.0)));

    let mut camera = Camera::new(1.0 / 1.0, 500, 100, 50);
    camera.render(Arc::new(world)).await;
}
