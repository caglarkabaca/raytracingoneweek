use async_trait::async_trait;
use glam::Vec3;

use crate::engine::ray::{Point, Ray};
use crate::engine::utils::*;
use crate::engine::vec3;
use crate::engine::vec3::vec3;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point,
    normal: Vec3,
    t: f32,
    front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: vec3(0.0),
            normal: vec3(0.0),
            t: 0.0,
            front_face: false,
        }
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = vec3::dot(&ray.direction(), outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone();
        } else {
            self.normal = -outward_normal.clone();
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: &Interval, rec: &mut HitRecord) -> bool;
}
// List
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = interval.max;

        for obj in self.objects.iter() {
            if obj.hit(ray, interval, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                rec.clone_from(&temp_record.clone());
                break;
            }
        }

        return hit_anything;
    }
}

// Sphere
pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(_center: &Point, _radius: f32) -> Sphere {
        Sphere {
            center: _center.clone(),
            radius: _radius,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let b = vec3::dot(&oc, &ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discrimant = b * b - a * c;
        if discrimant < 0.0 {
            return false;
        }
        let sqrtd = discrimant.sqrt();

        let mut root = (-b - sqrtd) / a;
        if root <= interval.min || interval.max <= root {
            root = (-b + sqrtd) / a;
            if root <= interval.min || interval.max <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, &outward_normal);

        return true;
    }
}
