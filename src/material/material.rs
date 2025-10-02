use crate::core::{Color, HitRecord, Point3, Ray};

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        Color::BLACK
    }
    fn color(&self, u: f32, v: f32, p: &Point3) -> Color;
}
