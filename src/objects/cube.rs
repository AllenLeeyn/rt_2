use std::ops::Neg;

use crate::core::{Point3, Vec3, Hittable, HitRecord, Ray};
use crate::pixels::texture::Texture;

#[derive(Clone)]
pub struct Cube {
    pub min: Point3,
    pub max: Point3,
    pub texture: Texture,
}

impl Cube {
    pub fn new(center: Point3, size: f32, texture: Texture) -> Self {
        let half = size / 2.0;
        let min = center - Vec3::new(half, half, half);
        let max = center + Vec3::new(half, half, half);
        Self {
            min,
            max,
            texture,
        }
    }
    
    fn compute_normal(&self, p: Point3) -> Vec3 {
        todo!()
    }

    fn compute_uv(&self, p: Point3) -> (f32, f32) {
        let u = (p.x() - self.min.x()) / (self.max.x() - self.min.x());
        let v = (p.y() - self.min.y()) / (self.max.y() - self.min.y());
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = texture;
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        todo!()
    }
}
