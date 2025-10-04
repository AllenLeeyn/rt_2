use crate::core::{Color, HitRecord, Ray};

pub trait Material: Send + Sync + Clone {
    /// Returns Some(scattered_ray, attenuation) if the ray scatters, None if absorbed
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}
