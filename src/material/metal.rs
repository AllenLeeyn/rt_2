use crate::core::{Color, HitRecord, Ray, Vec3};
use crate::material::Material;

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().normalize().reflect(hit_record.normal);

        // Fuzz is for randomizing reflection direction for rough metals
        let fuzz_vector = if self.fuzz > 0.0 {
            self.fuzz * Vec3::random_in_unit_sphere()
        } else {
            Vec3::ZERO
        };
        
        let scattered_direction = (reflected + fuzz_vector).normalize();
        let scattered = Ray::new(hit_record.p, scattered_direction);

        // Do not scatter light into the surface (for fuzzy metals)
        if scattered.direction().dot(hit_record.normal) > 0.0 {
            let texture_color = hit_record.color;
            let final_color = if let Some(textured_material) = &hit_record.textured_material {
                // Higher transparency = more clear / less influenced by texture colors
                let texture_influence = 1.0 - textured_material.transparency;

                let mixed_albedo = Color::lerp(
                    self.albedo,
                    texture_color,
                    texture_influence
                );

                mixed_albedo.with_transparency(textured_material.get_alpha())
            } else {
                texture_color
            };

            Some((scattered, final_color))
        } else {
            None
        }
    }
}
