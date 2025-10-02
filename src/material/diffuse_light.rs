use crate::core::{Color, Point3, Ray, HitRecord};
use crate::material::material::Material;
use crate::pixels::texture::Texture;

pub struct DiffuseLight {
    texture: Texture,
}

impl DiffuseLight {
    pub fn new(texture: Texture) -> Self {
        Self { texture }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.texture.value_at(u, v, *p)
    }

    fn color(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        Color::BLACK
    }
}
