use crate::core::{HitRecord, Hittable, Point3, Ray, Vec3};
use crate::material::material::Material;
use std::sync::Arc;

#[derive(Clone)]
pub struct Cylinder {
    center: Point3,
    radius: f32,
    height: f32,
    material: Arc<dyn Material>,
}

impl Cylinder {
    pub fn new(center: Point3, radius: f32, height: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            height,
            material,
        }
    }

    // Compute normal vector at a point on the cylinder surface
    fn compute_normal(&self, p: Point3) -> Vec3 {
        let dx = p.x() - self.center.x();
        let dz = p.z() - self.center.z();
        Vec3::new(dx, 0.0, dz).normalize()
    }

    // Compute UV coordinates for a point on the cylinder surface
    fn compute_uv(&self, p: Point3) -> (f32, f32) {
        let u = (p.x() - self.center.x() + self.radius) / (2.0 * self.radius);
        let v = (p.y() - self.center.y()) / self.height;
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }

    // Helper function to check intersection with a cap (top or bottom)
    fn hit_cap(&self, ray: &Ray, t_min: f32, t_max: f32, y: f32) -> Option<HitRecord> {
        // Skip if ray direction is too small (parallel to plane)
        if ray.direction().y().abs() < 0.001 {
            return None;
        }

        let t = (y - ray.origin().y()) / ray.direction().y();

        if t < t_min || t > t_max {
            return None;
        }
        // Compute the hit point on the cap
        let p = ray.at(t);
        let dx = p.x() - self.center.x();
        let dz = p.z() - self.center.z();

        if dx * dx + dz * dz <= self.radius * self.radius {
            let outward_normal = if y > self.center.y() {
                Vec3::Y
            } else {
                -Vec3::Y
            };
            let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
            let (u, v) = self.compute_uv(p);

            return Some(HitRecord {
                p,
                normal,
                t,
                u,
                v,
                front_face,
                material: &*self.material,
            });
        }

        None
    }

    fn hit_side(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

        let dx = ray.direction().x();
        let dz = ray.direction().z();
        let ox = oc.x();
        let oz = oc.z();

        let a = dx * dx + dz * dz;
        let b = 2.0 * (ox * dx + oz * dz);
        let c = ox * ox + oz * oz - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let inv_2a = 0.5 / a;

        for &t in [(-b - sqrt_d) * inv_2a, (-b + sqrt_d) * inv_2a].iter() {
            if t < t_min || t > t_max {
                continue;
            }

            let p = ray.at(t);
            let y = p.y();
            let y_min = self.center.y();
            let y_max = y_min + self.height;

            if y >= y_min && y <= y_max {
                let outward_normal = self.compute_normal(p);
                let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
                let (u, v) = self.compute_uv(p);

                return Some(HitRecord {
                    p,
                    normal,
                    t,
                    u,
                    v,
                    front_face,
                    material: &*self.material,
                });
            }
        }

        None
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = t_max;

        // Check intersection with cylinder side
        if let Some(side_hit) = self.hit_side(ray, t_min, closest_t) {
            closest_t = side_hit.t;
            closest_hit = Some(side_hit);
        }

        // Check intersection with bottom cap
        if let Some(bottom_hit) = self.hit_cap(ray, t_min, closest_t, self.center.y()) {
            closest_t = bottom_hit.t;
            closest_hit = Some(bottom_hit);
        }

        // Check intersection with top cap
        if let Some(top_hit) = self.hit_cap(ray, t_min, closest_t, self.center.y() + self.height) {
            closest_hit = Some(top_hit);
        }

        closest_hit
    }
}
