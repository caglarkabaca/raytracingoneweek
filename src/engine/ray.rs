use glam::*;

pub type Point = Vec3;

#[derive(Clone)]
pub struct Ray {
    orig: Point,
    dir: Vec3,
}

impl Ray {
    pub fn new_null() -> Ray {
        Ray {
            orig: vec3(0.0, 0.0, 0.0),
            dir: vec3(0.0, 0.0, 0.0),
        }
    }
    pub fn new(origin: Point, dir: Vec3) -> Ray {
        Ray {
            orig: origin.clone(),
            dir: dir.clone(),
        }
    }
    pub fn at(&self, t: f32) -> Point {
        self.orig + t * self.dir
    }

    pub fn origin(&self) -> Point {
        self.orig.clone()
    }

    pub fn direction(&self) -> Vec3 {
        self.dir.clone()
    }
}
