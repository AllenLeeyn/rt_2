use crate::core::{HitRecord, Hittable, Point3, Ray, Vec3};
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
        // Normal at any point on sphere surface is (point - center) / radius
        (p - self.center) / self.radius
    }

    fn compute_uv(&self, p: Point3) -> (f32, f32) {
        let (min, max) = self.bounding_box();
        let u = (p.x - min.x) / (max.x - min.x);
        let v = (p.y - min.y) / (max.y - min.y);
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
        let p = ray.at(t);
        let outward_normal = self.compute_normal(p);
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
