use crate::core::vec3::{Point3, Vec3};
 
#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, dir: Vec3) -> Self {
        Self { origin, dir }
    }
 
    pub fn origin(&self) -> Point3 {
        self.origin
    }
 
    pub fn direction(&self) -> Vec3 {
        self.dir
    }
 
    pub fn at(&self, t: f32) -> Point3 {
        self.origin + t * self.dir
    }
}