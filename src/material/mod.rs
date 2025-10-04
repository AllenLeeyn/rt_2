pub mod dielectric;
pub mod lambertian;
pub mod material;
pub mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use material::Material;
pub use metal::Metal;

#[derive(Clone)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

impl Material for MaterialType {
    fn scatter(
        &self,
        ray: &crate::core::Ray,
        hit_record: &crate::core::HitRecord,
    ) -> Option<(crate::core::Ray, crate::core::Color)> {
        match self {
            MaterialType::Lambertian(m) => m.scatter(ray, hit_record),
            MaterialType::Metal(m) => m.scatter(ray, hit_record),
            MaterialType::Dielectric(m) => m.scatter(ray, hit_record),
        }
    }
}
