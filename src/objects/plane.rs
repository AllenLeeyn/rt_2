use crate::core::{HitRecord, Hittable, Point3, Ray, Vec3};
use crate::pixels::texture::Texture;

#[derive(Clone)]
pub struct Plane {
    center: Point3, // Bottom-left corner or reference point
    size: Vec3,     // Size in X and Z (Y is ignored)
    texture: Texture,
    bounding_box: (Point3, Point3),
}

impl Plane {
    pub fn new(center: Point3, size: Vec3, texture: Texture) -> Self {
        let half_size = size * 0.5;

        let min = Point3::new(
            center.x() - half_size.x(),
            center.y() - 1e-4,
            center.z() - half_size.z(),
        );

        let max = Point3::new(
            center.x() + half_size.x(),
            center.y() + 1e-4,
            center.z() + half_size.z(),
        );

        Self {
            center,
            size,
            texture,
            bounding_box: (min, max),
        }
    }

    fn bounding_box(&self) -> (Point3, Point3) {
        self.bounding_box
    }

    fn normal(&self) -> Vec3 {
        Vec3::Y
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    pub fn size(&self) -> Vec3 {
        self.size
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = texture;
    }
}

impl Hittable for Plane {
    fn hit(&self, _ray: &Ray, _t_min: f32, _t_max: f32) -> Option<HitRecord> {
        // Plane intersection not yet implemented
        None
    }
}
