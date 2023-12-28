use std::{
    borrow::Borrow,
    fs::File,
    io::Write,
    ops::{Deref, DerefMut},
    sync::Arc,
    thread::{self, JoinHandle, Thread},
};

use crate::engine::{
    color::{write_color, Color},
    hittable::HitRecord,
    hittable::Hittable,
    ray::{Point, Ray},
    utils::Interval,
    vec3::*,
};
use glam::Vec3;
use rand::Rng;
use tokio::sync::Mutex;

#[derive(Clone, Copy)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,

    height: i32,
    camera_center: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, width: i32, samples_per_pixel: i32, max_depth: i32) -> Camera {
        Camera {
            aspect_ratio,
            width,
            samples_per_pixel,
            max_depth,
            height: 0,
            camera_center: vec3(0.0),
            pixel00_loc: vec3(0.0),
            pixel_delta_u: vec3(0.0),
            pixel_delta_v: vec3(0.0),
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.initialize();

        let bar = indicatif::ProgressBar::new(self.width as u64 * self.height as u64);

        let mut contents: Vec<Vec<String>> = Vec::new();

        for j in 0..self.height {
            let mut inner: Vec<String> = Vec::new();
            for i in 0..self.width {
                let mut pixel_color = vec3(0.0);
                for _ in 0..self.samples_per_pixel {
                    // sample
                    let r = self.get_ray(i, j);
                    pixel_color += self.ray_color(&r, 0, world);
                }
                inner.push(write_color(pixel_color, self.samples_per_pixel));
                bar.inc(1);
            }
            contents.insert(j as usize, inner);
        }

        let mut file = File::create("main.ppm").unwrap();
        file.write(
            format!("P3\n{} {}\n255\n", self.width, self.height)
                .as_str()
                .as_bytes(),
        )
        .unwrap();
        for content in contents {
            for inner in content {
                file.write(inner.as_str().as_bytes()).unwrap();
            }
            file.write(b"\n").unwrap();
        }

        bar.finish_with_message("done");
    }

    fn initialize(&mut self) {
        self.height = (self.width as f32 / self.aspect_ratio) as i32;
        self.camera_center = vec3(0.0);

        let focal_length = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (self.width as f32 / self.height as f32);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u / vec3(self.width as f32);
        self.pixel_delta_v = viewport_v / vec3(self.height as f32);

        let viewport_upper_left = self.camera_center
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth >= self.max_depth {
            return vec3(0.0);
        }

        let mut rec = HitRecord::new();
        if world.hit(r, &Interval::with(0.001, f32::INFINITY), &mut rec) {
            let direction = random_on_hemisphere(rec.normal());
            return 0.5 * self.ray_color(&Ray::new(rec.p, direction), depth + 1, world);
        }

        let unit_direction = r.direction().normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.3, 0.3, 1.0);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center: Color = self.pixel00_loc
            + (vec3(i as f32) * self.pixel_delta_u)
            + (vec3(j as f32) * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_squared();

        let ray_origin = self.camera_center;
        let ray_direction = (pixel_sample - self.camera_center).normalize();
        Ray::new(ray_origin, ray_direction)
    }

    fn pixel_sample_squared(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        let px = -0.5 * rng.gen::<f32>();
        let py = -0.5 * rng.gen::<f32>();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }
}
