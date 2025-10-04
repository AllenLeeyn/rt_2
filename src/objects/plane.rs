use crate::core::{Point3, Vec3, Hittable, HitRecord, Ray};
use crate::material::Material;

#[derive(Clone)]
pub struct Plane {
    center: Point3, // Bottom-left corner or reference point
    size: Vec3,     // Size in X and Z (Y is ignored)
    material: Material,
    bounding_box: (Point3, Point3),
}

impl Plane {
    pub fn new(center: Point3, size: Vec3, material: Material) -> Self {
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
            material,
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

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let outward_normal = self.normal();
        let denom = outward_normal.dot(ray.direction());

        if denom.abs() < 1e-6 {
            return None;
        }

        let t = (self.center.y() - ray.origin().y()) / ray.direction().y();
        if t < t_min || t > t_max {
            return None;
        }

        let p = ray.at(t);

        let (min, max) = self.bounding_box();

        if p.x() < min.x() || p.x() > max.x() || p.z() < min.z() || p.z() > max.z() {
            return None;
        }

        let u = (p.x() - min.x()) / (max.x() - min.x());
        let v = (p.z() - min.z()) / (max.z() - min.z());

        // Sample the texture
        let color = self.material.value_at(u, v, p);
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);

        Some(HitRecord {
            p,
            normal,
            t,
            color,
            u,
            v,
            front_face,
            material: self.material.clone(),
        })
    }
}
