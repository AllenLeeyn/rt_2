use crate::core::*;
use crate::material::MaterialType;

use rand::Rng;

const DIELECTRIC_MIN_SHADOW: f32 = 0.04;     // at least 4% light loss
const DIELECTRIC_DENSITY: f32 = 0.6;         // absorption per unit of distance
const DIELECTRIC_TINT_POWER: f32 = 1.0;      // for strengthening the tint

const METAL_SHADOW_TINT: f32 = 0.06;         // simulated color bleed for shadows
const METAL_TINT_POWER: f32 = 1.0;

#[derive(Clone, Copy)]
struct ShadowResult {
    transmit: Color,
    add: Color,
}

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
    pub fn new_point(
        position: Point3,
        color: Color,
        intensity: f32,
        samples: usize,
        radius: f32,
        softness: f32,
    ) -> Self {
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
            light_type: LightType::Directional {
                direction: direction.normalize(),
            },
            position: Point3::ZERO,
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

    /// Returns light attenuation factor
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

    /// Calculate how much light reflects vs transmits at a surface based on viewing angle
    /// Returns a value f32 between 0 and 1 (1 = full reflection, 0 = full transmission)
    /// At glancing angles, more light reflects; at perpendicular angles, more transmits
    /// This is an approximation of the Fresnel effect (Schlick's method)
    fn schlick(cosine: f32, index_of_refraction: f32) -> f32 {
        let r0 = (1.0 - index_of_refraction) / (1.0 + index_of_refraction);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    /// Trace a shadow ray from light to surface, calculating how much light gets through
    /// Returns: 
    /// how much light passes through (0=blocked, 1=full visibility) (transmit)
    /// small color contribution from materials the ray passes through (add)
    fn transmission_along_ray(
        &self,
        objects: &[Box<dyn Hittable>],
        mut ray: Ray,
        mut t_max: f32,
    ) -> ShadowResult {
        let mut throughput = Color::WHITE;
        let mut add = Color::BLACK;
        let mut safety = 0;
        let mut saw_dielectric = false;

        while throughput.luminance() > 1e-3 && t_max > 1e-3 && safety < 64 {
            safety += 1;

            let mut closest = t_max;
            let mut hit_opt = None;
            for obj in objects.iter() {
                if let Some(h) = obj.hit(&ray, 1e-3, closest) {
                    closest = h.t;
                    hit_opt = Some(h);
                }
            }

            let Some(hit) = hit_opt else { break; };

            match &hit.material {
                Some(MaterialType::Lambertian(_)) => { 
                    return ShadowResult { transmit: Color::BLACK, add }; 
                }

                Some(MaterialType::Metal(_metal)) => {
                    // Add tiny color bleed
                    let tint_strength = hit.textured_material
                        .as_ref()
                        .map(|tm| (1.0 - tm.transparency).powf(METAL_TINT_POWER))
                        .unwrap_or(1.0);
                    let tint = hit.color * (METAL_SHADOW_TINT * tint_strength);
                    add = add + tint;
                    return ShadowResult { transmit: Color::BLACK, add };
                }

                Some(MaterialType::Dielectric(diel)) => {
                    saw_dielectric = true;

                    // even 100% transparent loses a few percent
                    let cos = (-ray.direction()).dot(hit.normal).abs().min(1.0);
                    let r = Self::schlick(cos, diel.refractive_index);
                    let t = 1.0 - r;

                    let tex_influence = hit.textured_material
                        .as_ref()
                        .map(|tm| (1.0 - tm.transparency).powf(DIELECTRIC_TINT_POWER))
                        .unwrap_or(0.0);
                    let tint = Color::lerp(Color::WHITE, hit.color, tex_influence);

                    let seg = closest;
                    let absorb = Color::new(
                        tint.r().powf(DIELECTRIC_DENSITY * seg),
                        tint.g().powf(DIELECTRIC_DENSITY * seg),
                        tint.b().powf(DIELECTRIC_DENSITY * seg),
                    );

                    throughput = throughput * (absorb * t);

                    // move the ray origin slightly forward so it doesn't hit the same object again
                    let new_origin = hit.p + ray.direction() * 1e-3;
                    t_max -= closest;
                    ray = Ray::new(new_origin, ray.direction());
                }

                None => { 
                    return ShadowResult { transmit: Color::BLACK, add }; 
                }
            }
        }

        // minimum shadow
        if saw_dielectric {
            let max_trans = 1.0 - DIELECTRIC_MIN_SHADOW;
            throughput = Color::new(
                throughput.r().min(max_trans),
                throughput.g().min(max_trans),
                throughput.b().min(max_trans),
            );
        }

        ShadowResult { transmit: throughput, add }
    }

    pub fn contribution_from_hit(&self, objects: &[Box<dyn Hittable>], hit: &HitRecord) -> Color {
        match self.light_type {
            LightType::Point => {
                let mut accum = Color::BLACK;

                for _ in 0..self.samples {
                    let sample_point = self.random_point_on_light();
                    let light_dir = (sample_point - hit.p).normalize();
                    let light_dist = sample_point.distance(hit.p);

                    let shadow_origin = hit.p + hit.normal * 1e-3;
                    let shadow_ray = Ray::new(shadow_origin, light_dir);

                    let ShadowResult { transmit, add } =
                        self.transmission_along_ray(objects, shadow_ray, light_dist);

                    if transmit.luminance() > 0.0 || add.luminance() > 0.0 {
                        let lambert = self.diffuse(hit.normal, light_dir);
                        accum = accum + (transmit * lambert) + (add * lambert);
                    }
                }

                let avg = accum / (self.samples as f32);
                let att = self.attenuation(hit.p);

                self.color * hit.color * (avg * att)
            }

            LightType::Directional { direction } => {
                let light_dir = -direction.normalize();
                let shadow_origin = hit.p + hit.normal * 1e-3;
                let shadow_ray = Ray::new(shadow_origin, light_dir);

                let ShadowResult { transmit, add } =
                    self.transmission_along_ray(objects, shadow_ray, 1.0e6);

                let lambert = self.diffuse(hit.normal, light_dir);

                self.color * hit.color * ((transmit * lambert + add * lambert) * self.intensity)
            }
        }
    }
}
