use std::sync::Arc;

use crate::core::{HitRecord, Hittable, Point3, Ray, Vec3};
use crate::material::Material;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Arc<Material>,
    bounding_box: (Point3, Point3),
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center,
            radius,
            material: Arc::new(material),
            bounding_box: (center - rvec, center + rvec),
        }
    }

    fn bounding_box(&self) -> (Point3, Point3) {
        self.bounding_box
    }

    fn compute_normal(&self, point: Point3) -> Vec3 {
        // Normal at any point on sphere surface is (point - center) / radius
        (point - self.center) / self.radius
    }

    fn compute_uv(&self, point: Point3) -> (f32, f32) {
        let (min, max) = self.bounding_box();
        let u = (point.x() - min.x()) / (max.x() - min.x());
        let v = (point.y() - min.y()) / (max.y() - min.y());
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Vector from ray origin to sphere center
        let oc = ray.origin() - self.center;

        // Quadratic equation coefficients: at² + bt + c = 0
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        // Discriminant tells us if there are real solutions
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None; // No intersection
        }

        let sqrt_discriminant = discriminant.sqrt();

        // Try both roots of the quadratic equation
        let mut root = (-half_b - sqrt_discriminant) / a; // Closer intersection
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_discriminant) / a; // Farther intersection
            if root < t_min || root > t_max {
                return None; // Both intersections outside valid range
            }
        }

        // Calculate intersection point and surface properties
        let t = root;
        let point = ray.at(t);
        let outward_normal = self.compute_normal(point);
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
        let (u, v) = self.compute_uv(point);
        let color = self.material.value_at(u, v);

        Some(HitRecord {
            p: point,
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
