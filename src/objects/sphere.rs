use crate::core::{Point3, Hittable, HitRecord, Ray};
use crate::material::material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find nearest root in [t_min, t_max]
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);

        Some(HitRecord {
            p,
            normal,
            t: root,
            u: 0.0, // Will be fixed later
            v: 0.0, // Will be fixed later
            front_face,
            material: &*self.material,
        })
    }
}
