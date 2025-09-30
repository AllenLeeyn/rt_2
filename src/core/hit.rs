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
}
 
impl HitRecord {
    pub fn new(
        p: Point3,
        normal: Vec3,
        t: f32,
        color: Color,
        u: f32,
        v: f32,
    ) -> Self {
        Self {
            p,
            normal,
            t,
            color,
            u,
            v,
        }
    }
}
 
pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
