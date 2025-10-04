use crate::core::{HitRecord, Hittable, Point3, Ray, Vec3};
use crate::pixels::texture::Texture;

#[derive(Clone)]
pub struct Cube {
    pub min: Point3,
    pub max: Point3,
    pub texture: Texture,
}

impl Cube {
    pub fn new(center: Point3, size: f32, texture: Texture) -> Self {
        let half = size / 2.0;
        let min = center - Vec3::new(half, half, half);
        let max = center + Vec3::new(half, half, half);
        Self { min, max, texture }
    }

    fn compute_normal(&self, p: Point3) -> Vec3 {
        let center = (self.min + self.max) / 2.0;
        let diff = p - center;
        let extents = (self.max - self.min) / 2.0;

        let dx = (diff.x() / extents.x()).abs();
        let dy = (diff.y() / extents.y()).abs();
        let dz = (diff.z() / extents.z()).abs();

        if dx > dy && dx > dz {
            Vec3::new(diff.x().signum(), 0.0, 0.0)
        } else if dy > dz {
            Vec3::new(0.0, diff.y().signum(), 0.0)
        } else {
            Vec3::new(0.0, 0.0, diff.z().signum())
        }
    }
    
    // Compute UV coordinates for texture mapping (2D texture on 3D surface)
    fn compute_uv(&self, p: Point3) -> (f32, f32) {
        let u = (p.x() - self.min.x()) / (self.max.x() - self.min.x());
        let v = (p.y() - self.min.y()) / (self.max.y() - self.min.y());
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }

    pub fn set_texture(&mut self, texture: Texture) {
        self.texture = texture;
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

        let t = t_enter;
        let p = ray.at(t);
        let outward_normal = self.compute_normal(p);
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
        let (u, v) = self.compute_uv(p);
        let color = self.texture.value_at(u, v, p);

        Some(HitRecord {
            p,          // hit_point
            normal,     // surface normal
            t,          // distance along ray
            color,      // surface color
            u,          // texture coordinate u
            v,          // texture coordinate v
            front_face, // whether the ray hits the front face
        })
    }
}
