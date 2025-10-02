use crate::core::{HitRecord, Hittable, Point3, Ray, Vec3};
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
        let min = Point3::new(center.x() - radius, center.y(), center.z() - radius);

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

    fn compute_normal(&self, _p: Point3) -> Vec3 {
        // Cylinder normal computation not yet implemented
        Vec3::new(0.0, 1.0, 0.0)
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

    fn hit_cap(&self, _ray: &Ray, _t_min: f32, _t_max: f32, _y: f32) -> Option<HitRecord> {
        // Cylinder cap intersection not yet implemented
        None
    }

    fn hit_side(&self, _ray: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> {
        // Cylinder side intersection not yet implemented
        None
    }
}

impl Hittable for Cylinder {
    fn hit(&self, _ray: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> {
        // Cylinder intersection not yet implemented
        None
    }
}
