use crate::core::*;

use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum LightType {
    Point,
    Directional { direction: Vec3 },
}

pub struct Light {
    light_type: LightType,
    position: Point3,
    color: Color,
    intensity: f32,
    samples: usize,
    radius: f32,
    softness: f32,
}

impl Light {
    pub fn new_point(position: Point3, color: Color, intensity: f32, samples: usize, radius: f32, softness: f32) -> Self {
        Self {
            light_type: LightType::Point,
            position,
            color,
            intensity,
            samples,
            radius,
            softness,
        }
    }

    pub fn new_directional(direction: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            light_type: LightType::Directional { direction: direction.normalize() },
            position: Point3::ZERO, // unused
            color,
            intensity,
            samples: 1,
            radius: 0.0,
            softness: 0.0,
        }
    }

    /// Returns normalized direction from hit point to light
    pub fn direction_from(&self, point: Point3) -> Vec3 {
        (self.position - point).normalize()
    }

    /// Returns squared distance from point to light (saves sqrt cost)
    pub fn distance_squared(&self, point: Point3) -> f32 {
        (self.position - point).length_squared()
    }

    /// Returns distance from point to light
    pub fn distance(&self, point: Point3) -> f32 {
        (self.position - point).length()
    }

    /// Computes diffuse light contribution using Lambertian reflection
    pub fn diffuse(&self, normal: Vec3, light_dir: Vec3) -> f32 {
        normal.dot(light_dir).max(0.0)
    }

    pub fn attenuation(&self, point: Point3) -> f32 {
        let dist = self.distance(point);
        let min_dist = 1.0;
        let max_dist = 10.0;

        if dist >= max_dist {
            return 0.0;
        }

        let t = (dist - min_dist) / (max_dist - min_dist);
        let falloff = 1.0 - t * t * (3.0 - 2.0 * t); // smoothstep

        self.intensity * falloff
    }

    pub fn random_point_on_light(&self) -> Point3 {
        let radius = self.radius * self.softness.clamp(1e-3, 1.0);
        let mut rng = rand::rng();
        let theta = rng.random_range(0.0..2.0 * std::f32::consts::PI);
        let r = rng.random_range(0.0..radius);
        let dx = r * theta.cos();
        let dz = r * theta.sin();
        self.position + Vec3::new(dx, 0.0, dz) // area light on XZ plane
    }

    pub fn contribution_from_hit(
        &self,
        objects: &[Box<dyn Hittable>],
        hit: &HitRecord,
        ray: &Ray,
    ) -> Color {
        let view_dir = -ray.direction().normalize();

        let (_light_dir, _light_dist, attenuation, visibility, specular) = match self.light_type {
            LightType::Point => {
                let mut total_diffuse = 0.0;
                let mut total_specular = 0.0;

                for _ in 0..self.samples {
                    let sample_point = self.random_point_on_light();
                    let light_dir = (sample_point - hit.p).normalize();
                    let light_dist = sample_point.distance(hit.p);

                    let shadow_origin = hit.p + hit.normal * 1e-3;
                    let shadow_ray = Ray::new(shadow_origin, light_dir);

                    let transmission = transparency_along_ray(&shadow_ray, objects, light_dist);

                    if transmission > 0.0 {
                        let diffuse = self.diffuse(hit.normal, light_dir);
                        let specular = hit.material.phong_specular(light_dir, view_dir, hit.normal);

                        total_diffuse += diffuse * transmission;
                        total_specular += specular * transmission;
                    }
                }
            
                let avg_diffuse = total_diffuse / self.samples as f32;
                let avg_specular = total_specular / self.samples as f32;
                let attenuation = self.attenuation(hit.p);

                let main_light_dir = self.direction_from(hit.p);

                let specular_color = self.color * attenuation * avg_specular * hit.material.specular;
                
                (main_light_dir, self.distance(hit.p), attenuation, avg_diffuse, specular_color)
            }, 

            LightType::Directional { direction } => {
                let light_dir = -direction.normalize(); // from light to hit point

                let shadow_origin = hit.p + hit.normal * 1e-3;
                let shadow_ray = Ray::new(shadow_origin, light_dir);

                let in_shadow = objects.iter().any(|obj| obj.hit(&shadow_ray, 1e-3, 1000.0).is_some());

                let diffuse = self.diffuse(hit.normal, light_dir);
                let visibility = if in_shadow { 0.0 } else { 1.0 };

                
                let specular_color = self.color * hit.material.phong_specular(light_dir, view_dir, hit.normal);

                (light_dir, 1.0, self.intensity, visibility * diffuse, specular_color)
            }
        };

        hit.color * self.color * (attenuation * visibility) + specular
    }

}

fn transparency_along_ray(ray: &Ray, objects: &[Box<dyn Hittable>], max_distance: f32) -> f32 {
    let mut transparency = 1.0;
    let mut t_min = 1e-3;

    while t_min < max_distance {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = max_distance;

        for obj in objects {
            if let Some(hit) = obj.hit(ray, t_min, closest_t) {
                closest_t = hit.t;
                closest_hit = Some(hit);
            }
        }

        if let Some(hit) = closest_hit {
            let opacity = 1.0 - hit.material.transparency.clamp(0.0, 1.0);
            transparency *= 1.0 - opacity;

            if transparency < 0.01 {
                return 0.0;
            }

            t_min = closest_t + 1e-3;
        } else {
            break;
        }
    }

    transparency
}
