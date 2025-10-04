use crate::core::{Color, HitRecord, Ray, Vec3};
use crate::material::Material;

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        // Use cosine-weighted hemisphere sampling for realistic diffuse reflection
        let mut scatter_direction = hit_record.normal + Vec3::random_cosine_direction();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered = Ray::new(hit_record.p, scatter_direction.normalize());
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
    }
}
