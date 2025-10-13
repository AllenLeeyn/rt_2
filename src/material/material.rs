use crate::core::{Color, HitRecord, Ray, Vec3};
use crate::pixels::texture::Texture;
use crate::random_float;

#[derive(Debug, Clone)]
pub struct Material {
    pub texture: Texture,         // Base color or texture
    pub diffuse: f32,             // 0 = none, 1 = fully diffuse
    pub reflectivity: f32,        // 0 = none, 1 = mirror
    pub transparency: f32,        // 0 = opaque, 1 = fully transparent
    pub index_of_refraction: f32, // Index of refraction
    pub emission: Option<Color>,  // Light source
}

#[derive(Debug, Clone)]
pub struct ScatterResult {
    pub scattered_ray: Ray,
    pub attenuation: Color,
}

impl Material {
    pub fn value_at(&self, u: f32, v: f32) -> Color {
        self.texture.value_at(u, v)
    }

    pub fn scatter(&self, ray_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let diffuse = self.diffuse.clamp(0.0, 1.0);
        let reflectivity = self.reflectivity.clamp(0.0, 1.0);
        let transparency = self.transparency.clamp(0.0, 1.0);

        let total = diffuse + reflectivity + transparency;
        if total == 0.0 {
            return None;
        }

        match select_scatter(diffuse, reflectivity, transparency) {
            Some("diffuse") => {
                let scatter = self.scatter_diffuse(hit);
                Some(ScatterResult {
                    scattered_ray: scatter.scattered_ray,
                    attenuation: scatter.attenuation * (diffuse / total) * diffuse,
                })
            }
            Some("reflect") => self
                .scatter_reflection(ray_in, hit)
                .map(|scatter| ScatterResult {
                    scattered_ray: scatter.scattered_ray,
                    attenuation: scatter.attenuation * (reflectivity / total) * reflectivity,
                }),
            Some("refract") => {
                let scatter = self.scatter_refraction(ray_in, hit);
                Some(ScatterResult {
                    scattered_ray: scatter.scattered_ray,
                    attenuation: scatter.attenuation * (transparency / total),
                })
            }
            _ => None,
        }
    }

    fn scatter_diffuse(&self, hit: &HitRecord) -> ScatterResult {
        let scatter_dir = Vec3::random_in_hemisphere(hit.normal);

        // Apply epsilon offset along the normal to prevent self-intersection
        let epsilon = 1e-4;
        let point = hit.p + hit.normal * epsilon;
        let ray = Ray::new(point, scatter_dir);
        let surface_color = self.texture.value_at(hit.u, hit.v);

        ScatterResult {
            scattered_ray: ray,
            attenuation: surface_color * (1.0 / std::f32::consts::PI),
        }
    }

    fn scatter_reflection(&self, ray_in: &Ray, hit: &HitRecord) -> Option<ScatterResult> {
        let reflected = ray_in.direction().reflect(hit.normal).normalize();
        let fuzz = self.diffuse;
        let fuzzed = (reflected + fuzz * Vec3::random_in_unit_sphere()).normalize();

        if fuzzed.dot(hit.normal) > 0.0 {
            // Apply epsilon offset along the normal to prevent self-intersection
            let epsilon = 1e-4;
            let point = hit.p + hit.normal * epsilon;
            let ray = Ray::new(point, fuzzed);

            let surface_color = self.texture.value_at(hit.u, hit.v);
            Some(ScatterResult {
                scattered_ray: ray,
                attenuation: surface_color,
            })
        } else {
            None
        }
    }

    fn scatter_refraction(&self, ray_in: &Ray, hit: &HitRecord) -> ScatterResult {
        let refraction_ratio = if hit.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };
        let unit_dir = ray_in.direction().normalize();
        let cos_theta = (-unit_dir).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_ratio * sin_theta > 1.0
            || schlick(cos_theta, refraction_ratio) > random_float()
        {
            unit_dir.reflect(hit.normal)
        } else {
            Vec3::refract(&unit_dir, hit.normal, refraction_ratio)
        };

        // Apply epsilon offset along the normal to prevent self-intersection
        let epsilon = 1e-4;
        let point = hit.p - hit.normal * epsilon;
        let ray = Ray::new(point, direction);

        let surface_color = self.texture.value_at(hit.u, hit.v);
        let tint_strength = 1.0 - self.transparency.clamp(0.0, 1.0);
        let attenuation = Color::WHITE * self.transparency + surface_color * tint_strength;
        ScatterResult {
            scattered_ray: ray,
            attenuation,
        }
    }
}

pub fn select_scatter(diffuse: f32, reflectivity: f32, transparency: f32) -> Option<&'static str> {
    let diffuse = diffuse.clamp(0.0, 1.0);
    let reflectivity = reflectivity.clamp(0.0, 1.0);
    let transparency = transparency.clamp(0.0, 1.0);

    let total = diffuse + reflectivity + transparency;
    if total == 0.0 {
        return None;
    }

    let choice = random_float() * total;

    if choice < diffuse {
        Some("diffuse")
    } else if choice < diffuse + reflectivity {
        Some("reflect")
    } else {
        Some("refract")
    }
}

fn schlick(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
