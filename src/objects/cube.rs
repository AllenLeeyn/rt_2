use crate::core::{HitRecord, Hittable, Point3, Ray, Vec3};
use crate::material::Material;

#[derive(Clone)]
pub struct Cube {
    pub min: Point3,
    pub max: Point3,
    material: Material,
}

impl Cube {
    pub fn new(center: Point3, size: f32, material: Material) -> Self {
        let half = size / 2.0;
        let min = center - Vec3::new(half, half, half);
        let max = center + Vec3::new(half, half, half);
        Self { min, max, material }
    }

    fn compute_normal(&self, p: Point3) -> Vec3 {
        // Determine which face was hit by comparing the hit point to the box faces.
        // Use an epsilon to tolerate floating point error.
        let eps = 1e-4;

        if (p.x() - self.min.x()).abs() < eps {
            return Vec3::new(-1.0, 0.0, 0.0);
        }
        if (p.x() - self.max.x()).abs() < eps {
            return Vec3::new(1.0, 0.0, 0.0);
        }

        if (p.y() - self.min.y()).abs() < eps {
            return Vec3::new(0.0, -1.0, 0.0);
        }
        if (p.y() - self.max.y()).abs() < eps {
            return Vec3::new(0.0, 1.0, 0.0);
        }

        if (p.z() - self.min.z()).abs() < eps {
            return Vec3::new(0.0, 0.0, -1.0);
        }
        if (p.z() - self.max.z()).abs() < eps {
            return Vec3::new(0.0, 0.0, 1.0);
        }

        // Fallback: choose largest component of difference from center (rare)
        let center = (self.min + self.max) / 2.0;
        let diff = p - center;
        let abs = Vec3::new(diff.x().abs(), diff.y().abs(), diff.z().abs());
        if abs.x() > abs.y() && abs.x() > abs.z() {
            Vec3::new(diff.x().signum(), 0.0, 0.0)
        } else if abs.y() > abs.z() {
            Vec3::new(0.0, diff.y().signum(), 0.0)
        } else {
            Vec3::new(0.0, 0.0, diff.z().signum())
        }
    }

    fn compute_uv(&self, p: Point3) -> (f32, f32) {
        let u = (p.x() - self.min.x()) / (self.max.x() - self.min.x());
        let v = (p.y() - self.min.y()) / (self.max.y() - self.min.y());
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut t_enter = t_min;
        let mut t_exit = t_max;

        for i in 0..3 {
            let inv_d = 1.0 / ray.direction()[i];
            let mut t0 = (self.min[i] - ray.origin()[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin()[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_enter = t_enter.max(t0);
            t_exit = t_exit.min(t1);

            if t_exit <= t_enter {
                return None;
            }
        }

        // If ray starts inside the box, t_enter may equal t_min; choose t_exit then.
        let t_hit = if t_enter <= t_min { t_exit } else { t_enter };
        if t_hit < t_min || t_hit > t_max {
            return None;
        }

        let t = t_hit;
        let p = ray.at(t);
        let outward_normal = self.compute_normal(p);
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
        let (u, v) = self.compute_uv(p);
        let color = self.material.value_at(u, v);

        Some(HitRecord {
            p,          // hit_point
            normal,     // surface normal
            t,          // distance along ray
            color,      // surface color
            u,          // texture coordinate u
            v,          // texture coordinate v
            front_face, // whether the ray hits the front face
            material: self.material.clone(),
        })
    }
}
