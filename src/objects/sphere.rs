use crate::core::{Point3, Vec3, Hittable, HitRecord, Ray};
use crate::pixels::texture::Texture;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    texture: Texture,
    bounding_box: (Point3, Point3),
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, texture: Texture) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center,
            radius,
            texture,
            bounding_box: (center - rvec, center + rvec),
        }
    }

    fn bounding_box(&self) -> (Point3, Point3) {
        self.bounding_box
    }

    fn compute_normal(&self, p: Point3) -> Vec3 {
        todo!()
    }
    
    fn compute_uv(&self, p: Point3) -> (f32, f32) {
        let (min, max) = self.bounding_box();
        let u = (p.x() - min.x()) / (max.x() - min.x());
        let v = (p.y() - min.y()) / (max.y() - min.y());
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        todo!()
    }
}
