use crate::core::{Color, HitRecord, Ray};
use crate::material::Material;
use rand::Rng;

#[derive(Clone)]
pub struct Dielectric {
    pub refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self {
            // Less than 0.1 is not gonna have any noticeable visual effect, and negative values are not possible
            refractive_index: refractive_index.max(0.1)
        }
    }

    fn reflectance(&self, cosine: f32, ref_idx: f32) -> f32 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0_squared = r0 * r0;
        r0_squared + (1.0 - r0_squared) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let texture_color = hit_record.color;
        
        let base_attenuation = if let Some(textured_material) = &hit_record.textured_material {
            // Higher transparency = more clear / less influenced by texture colors
            let material_influence = textured_material.transparency;
            let texture_influence = 1.0 - textured_material.transparency;
            
            // Base clear color for dielectric materials
            let base_clear = Color::lerp(
                Color::new(0.9, 0.9, 0.9),
                Color::WHITE,
                material_influence
            );
            
            let mixed_color = Color::lerp(
                base_clear,
                texture_color,
                texture_influence
            );
            
            mixed_color.with_transparency(textured_material.get_alpha())
        } else {
            let base_clear = Color::new(0.9, 0.9, 0.9);
            texture_color.alpha_blend_over(base_clear)
        };
        
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        // Normalize ray direction and calculate incident angle
        let unit_direction = ray.direction().normalize();
        let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        // Determine if total internal reflection occurs (Snell's law)
        let mut rng = rand::rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        // Probabilistically choose reflection vs refraction (Fresnel equations)
        let should_reflect = rng.random::<f32>() < self.reflectance(cos_theta, self.refractive_index);

        let direction = if cannot_refract || should_reflect {
            unit_direction.reflect(hit_record.normal)
        } else {
            unit_direction.refract(hit_record.normal, refraction_ratio)
        };

        let scattered = Ray::new(hit_record.p, direction);
        Some((scattered, base_attenuation))
    }
}
