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
    // Set a new texture for the cylinder
    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = texture;
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

        let p = ray.at(t);
        let dx = p.x() - self.center.x();
        let dz = p.z() - self.center.z();

        if dx * dx + dz * dz > self.radius * self.radius {
            return None;
        }

        let normal = if y > self.center.y() {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(0.0, -1.0, 0.0)
        };

        let (normal, front_face) = HitRecord::face_normal(ray, normal);
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

    fn hit_side(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Simple cylinder-ray intersection (infinite cylinder, then clip by height)
        let oc_x = ray.origin().x() - self.center.x();
        let oc_z = ray.origin().z() - self.center.z();

        let a =
            ray.direction().x() * ray.direction().x() + ray.direction().z() * ray.direction().z();
        let b = 2.0 * (oc_x * ray.direction().x() + oc_z * ray.direction().z());
        let c = oc_x * oc_x + oc_z * oc_z - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 || a.abs() < 0.001 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let t1 = (-b - sqrt_d) / (2.0 * a);
        let t2 = (-b + sqrt_d) / (2.0 * a);

        // Try closest intersection first
        for &t in &[t1, t2] {
            if t >= t_min && t <= t_max {
                let p = ray.at(t);
                let y = p.y() - self.center.y();

                if y >= 0.0 && y <= self.height {
                    let normal = self.compute_normal(p);
                    let (normal, front_face) = HitRecord::face_normal(ray, normal);
                    let (u, v) = self.compute_uv(p);
                    let color = self.texture.value_at(u, v, p);

                    return Some(HitRecord {
                        p,
                        normal,
                        t,
                        color,
                        u,
                        v,
                        front_face,
                    });
                }
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
