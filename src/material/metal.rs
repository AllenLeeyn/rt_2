use crate::core::{Color, Ray, HitRecord, Vec3, Point3};
use crate::material::material::Material;
use crate::pixels::texture::Texture;

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.0
}

pub struct Metal {
    texture: Texture,
    fuzz: f32,
}

impl Metal {
    pub fn new(texture: Texture, fuzz: f32) -> Self {
        Self { texture, fuzz: fuzz.min(1.0) }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(r_in.direction().normalize(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere() * self.fuzz);
        if scattered.direction().dot(rec.normal) > 0.0 {
            let attenuation = self.texture.value_at(rec.u, rec.v, rec.p);
            Some((attenuation, scattered))
        } else {
            None
        }
    }

    fn color(&self, u: f32, v: f32, p: &Point3) -> Color {
        self.texture.value_at(u, v, *p)
    }
}
