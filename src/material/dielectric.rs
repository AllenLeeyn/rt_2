use crate::core::{Color, HitRecord, Point3, Ray, Vec3};
use crate::material::material::Material;

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (uv * -1.0).dot(n).min(1.0);
    let r_out_perp = (uv + n * cos_theta) * etai_over_etat;
    let r_out_parallel = n * -(1.0 - r_out_perp.length_squared()).abs().sqrt();
    r_out_perp + r_out_parallel
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * v.dot(n) * 2.0
}

pub struct Dielectric {
    pub ir: f32, // Index of Refraction
    pub color: Color,
}

impl Dielectric {
    pub fn new(ir: f32, color: Color) -> Self {
        Self { ir, color }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        //let attenuation = Color::new(255.0, 255.0, 255.0);
        let attenuation = self.color * 255.0;
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().normalize();
        let cos_theta = (unit_direction * -1.0).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random::<f32>() {
                reflect(unit_direction, rec.normal)
            } else {
                refract(unit_direction, rec.normal, refraction_ratio)
            };

        let scattered = Ray::new(rec.p, direction);

        Some((attenuation, scattered))
    }

    fn color(&self, _u: f32, _v: f32, _p: &Point3) -> Color {
        Color::WHITE
    }
}
