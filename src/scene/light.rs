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
    let quadratic = 0.032;

    self.intensity / (quadratic * dist * dist * dist * dist)
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
    ) -> Color {
        let (_light_dir, _light_dist, attenuation, visibility) = match self.light_type {
            LightType::Point => {
                let mut visible = 0;
                let mut total_diffuse = 0.0;

                for _ in 0..self.samples {
                    let sample_point = self.random_point_on_light();
                    let light_dir = (sample_point - hit.p).normalize();
                    let light_dist = sample_point.distance(hit.p);

                    let shadow_origin = hit.p + hit.normal * 1e-3;
                    let shadow_ray = Ray::new(shadow_origin, light_dir);

                    let in_shadow = objects.iter().any(|obj| {
                        obj.hit(&shadow_ray, 1e-3, light_dist).is_some()
                    });

                    if !in_shadow {
                        visible += 1;
                        total_diffuse += self.diffuse(hit.normal, light_dir);
                    }
                }

                let visibility = visible as f32 / self.samples as f32;
                let avg_diffuse = total_diffuse / self.samples as f32;
                let attenuation = self.attenuation(hit.p);

                let main_light_dir = self.direction_from(hit.p);

                (main_light_dir, self.distance(hit.p), attenuation, visibility * avg_diffuse)
            }, 

            LightType::Directional { direction } => {
                let light_dir = -direction.normalize(); // from light to hit point

                let shadow_origin = hit.p + hit.normal * 1e-3;
                let shadow_ray = Ray::new(shadow_origin, light_dir);

                let in_shadow = objects.iter().any(|obj| obj.hit(&shadow_ray, 1e-3, 1000.0).is_some());

                let diffuse = self.diffuse(hit.normal, light_dir);
                let visibility = if in_shadow { 0.0 } else { 1.0 };

                (light_dir, 1.0, self.intensity, visibility * diffuse)
            }
        };

        let surface_color = hit.material.color(hit.u, hit.v, &hit.p);
        surface_color.mul(self.color.mul_f32(attenuation * visibility))
    }

}
