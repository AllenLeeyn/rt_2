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
        let half_size = size / 2.0;

        let min = Point3::new(
            center.x - half_size.x,
            center.y - 1e-4,
            center.z - half_size.z,
        );

        let max = Point3::new(
            center.x + half_size.x,
            center.y + 1e-4,
            center.z + half_size.z,
        );

        Self {
            center,
            size,
            texture,
            bounding_box: (min, max),
        }
    }

    fn normal(&self) -> Vec3 {
        Vec3::Y
    }

    fn compute_uv(&self, p: Point3) -> (f32, f32) {
        let half_size = self.size / 2.0;
        let u = (p.x - (self.center.x - half_size.x)) / self.size.x;
        let v = (p.z - (self.center.z - half_size.z)) / self.size.z;
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Check if ray is parallel to plane (no intersection)
        if ray.direction().y.abs() < 1e-8 {
            return None;
        }

        // Calculate intersection parameter t
        let t = (self.center.y - ray.origin().y) / ray.direction().y;

        // Check if intersection is within valid range
        if t < t_min || t > t_max {
            return None;
        }

        // Calculate intersection point
        let p = ray.at(t);

        // Check if intersection point is within plane bounds
        let (min, max) = self.bounding_box;
        if p.x < min.x || p.x > max.x || p.z < min.z || p.z > max.z {
            return None;
        }

        // Calculate surface properties
        let outward_normal = self.normal();
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
        let (u, v) = self.compute_uv(p);
        let color = self.texture.value_at(u, v, p);

        Some(HitRecord {
            p,
            normal,
            t,
            color,
            u,
            v,
            front_face,
        })
    }
}
