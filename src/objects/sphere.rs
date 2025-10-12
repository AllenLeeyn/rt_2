use crate::core::{Point3, Hittable, HitRecord, Ray, Vec3};
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

    fn compute_normal(&self, p: Point3) -> Vec3 {
        // Normal at any point on sphere surface is (point - center) / radius
        (p - self.center) / self.radius
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
        let p = ray.at(t);
        let outward_normal = self.compute_normal(p);
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
