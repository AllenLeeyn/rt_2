use crate::core::ray::Ray;
use crate::core::vec3::{Point3, Vec3};
use crate::core::color::Color;

/// Stores data about a ray-object intersection.
#[derive(Clone)]
pub struct HitRecord {
    /// Point of intersection
    pub p: Point3,
    /// Surface normal at intersection
    pub normal: Vec3,
    /// Ray parameter at intersection (distance along ray)
    pub t: f32,
    /// Color from texture/material at the hit point
    pub color: Color,
    /// Texture coordinate u
    pub u: f32,
    /// Texture coordinate v
    pub v: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn face_normal(ray: &Ray, outward_normal: Vec3) -> (Vec3, bool) {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        (normal, front_face)
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
