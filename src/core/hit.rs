use crate::core::ray::Ray;
use crate::core::vec3::{Point3, Vec3};
use crate::material::material::Material;

/// Stores data about a ray-object intersection.
#[derive(Clone)]
pub struct HitRecord<'a> {
    /// Point of intersection
    pub p: Point3,
    /// Surface normal at intersection
    pub normal: Vec3,
    /// Ray parameter at intersection (distance along ray)
    pub t: f32,
    /// Texture coordinate u
    pub u: f32,
    /// Texture coordinate v
    pub v: f32,
    pub front_face: bool,
    pub material: &'a dyn Material,
}

impl<'a> HitRecord<'a> {
    pub fn face_normal(ray: &Ray, outward_normal: Vec3) -> (Vec3, bool) {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        (normal, front_face)
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'_>>;
}
