use crate::core::{Point3, Vec3, Hittable, HitRecord, Ray};
use crate::pixels::texture::Texture;

#[derive(Clone)]
pub struct Cylinder {
    center: Point3,
    radius: f32,
    height: f32,
    texture: Texture,
    bounding_box: (Point3, Point3),
}

impl Cylinder {
    pub fn new(center: Point3, radius: f32, height: f32, texture: Texture) -> Self {
        let min = Point3::new(
            center.x() - radius,
            center.y(),
            center.z() - radius,
        );

        let max = Point3::new(
            center.x() + radius,
            center.y() + height,
            center.z() + radius,
        );

        Self {
            center,
            radius,
            height,
            texture,
            bounding_box: (min, max),
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

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = texture;
    }

    fn hit_cap(&self, ray: &Ray, t_min: f32, t_max: f32, y: f32) -> Option<HitRecord> {
        todo!()
    }

    fn hit_side(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        todo!()
    }

}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        todo!()
    }
}
