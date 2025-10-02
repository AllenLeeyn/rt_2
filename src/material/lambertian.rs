use crate::core::{Color, Ray, HitRecord, Point3, Vec3};
use crate::material::material::Material;
use crate::pixels::texture::Texture;

pub struct Lambertian {
    pub texture: Texture,
}

impl Lambertian {
    pub fn new(texture: Texture) -> Self {
        Self { texture }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalize();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.texture.value_at(rec.u, rec.v, rec.p);
        Some((attenuation, scattered))
    }

    fn color(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.texture.value_at(u, v, *p)
    }

    fn is_diffuse(&self) -> bool {
        true
    }
}
