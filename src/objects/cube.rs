use std::ops::Neg;

use crate::core::{HitRecord, Hittable, Point3, Ray, Vec3};
use crate::material::MaterialType;
use crate::pixels::texture::{Texture, TexturedMaterial};

#[derive(Clone)]
pub struct Cube {
    pub min: Point3,
    pub max: Point3,
    pub texture: Texture,
    material: Option<MaterialType>,
    textured_material: Option<TexturedMaterial>,
}

impl Cube {
    pub fn new(center: Point3, size: f32, texture: Texture) -> Self {
        let half = size / 2.0;
        let min = center - Vec3::new(half, half, half);
        let max = center + Vec3::new(half, half, half);
        Self { 
            min, 
            max, 
            texture,
            material: None,
            textured_material: None,
        }
    }

    pub fn set_material(&mut self, material: MaterialType) {
        self.material = Some(material);
    }

    pub fn set_textured_material(&mut self, textured_material: TexturedMaterial) {
        self.textured_material = Some(textured_material);
    }

    pub fn material(&self) -> Option<&MaterialType> {
        self.material.as_ref()
    }

    pub fn textured_material(&self) -> Option<&TexturedMaterial> {
        self.textured_material.as_ref()
    }

    fn compute_normal(&self, p: Point3) -> Vec3 {
        let epsilon = 1e-4;

        if (p.x() - self.min.x()).abs() < epsilon {
            Vec3::X.neg()
        } else if (p.x() - self.max.x()).abs() < epsilon {
            Vec3::X
        } else if (p.y() - self.min.y()).abs() < epsilon {
            Vec3::Y.neg()
        } else if (p.y() - self.max.y()).abs() < epsilon {
            Vec3::Y
        } else if (p.z() - self.min.z()).abs() < epsilon {
            Vec3::Z.neg()
        } else {
            Vec3::Z
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

        for i in 0..3 {
            let inv_d = 1.0 / ray.direction()[i];
            let mut t0 = (self.min[i] - ray.origin()[i]) * inv_d;
            let mut t1 = (self.max[i] - ray.origin()[i]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            tmin = tmin.max(t0);
            tmax = tmax.min(t1);

            if tmax <= tmin {
                return None;
            }
        }

        let t = tmin;
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
            material: self.material.clone(),
            textured_material: self.textured_material.clone(),
        })
    }
}
