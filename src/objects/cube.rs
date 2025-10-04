use crate::core::{Point3, Vec3, Hittable, HitRecord, Ray};
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
        Self {
            min,
            max,
            material,
        }
    }

    fn compute_uv(&self, p: Point3) -> (f32, f32) {
        let u = (p.x() - self.min.x()) / (self.max.x() - self.min.x());
        let v = (p.y() - self.min.y()) / (self.max.y() - self.min.y());
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut tmin = t_min;
        let mut tmax = t_max;
        let mut hit_axis = 0;

        for i in 0..3 {
            let inv_d = 1.0 / ray.direction()[i];
            let mut t0 = (self.min[i] - ray.origin()[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin()[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            if t0 > tmin {
                tmin = t0;
                hit_axis = i; // track axis of entry
            }

            tmax = tmax.min(t1);

            if tmax <= tmin {
                return None;
            }
        }

        let t = tmin;
        let p = ray.at(t);

        // Compute normal from hit axis
        let mut outward_normal = Vec3::ZERO;
        let dir = ray.direction()[hit_axis];

        outward_normal[hit_axis] = if dir < 0.0 { 1.0 } else { -1.0 };

        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
        let (u, v) = self.compute_uv(p);
        let color = self.material.value_at(u, v, p);

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
