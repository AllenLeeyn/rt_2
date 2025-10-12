pub mod color;
pub mod hit;
pub mod ray;
pub mod vec3;

pub use color::Color;
pub use hit::{HitRecord, Hittable};
pub use ray::Ray;
pub use vec3::{Point3, Vec3};
