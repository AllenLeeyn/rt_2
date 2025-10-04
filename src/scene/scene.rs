use crate::core::*;
use crate::material::{Material, MaterialType};
use crate::pixels::*;
use crate::scene::*;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
    lights: Vec<Light>,
    background: Texture,
    camera: Camera,
    max_depth: u32,
    samples_per_pixel: u32,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            background: Texture::SolidColor(Color::BLACK),
            camera: Camera::new(),
            max_depth: 10,
            samples_per_pixel: 16,
        }
    }

    pub fn set_background(&mut self, texture: Texture) {
        self.background = texture;
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn set_max_depth(&mut self, depth: u32) {
        self.max_depth = depth;
    }

    pub fn set_samples_per_pixel(&mut self, samples: u32) {
        self.samples_per_pixel = samples;
    }

    pub fn add_object<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn render(&mut self, path: &str) -> std::io::Result<()> {
        let (width, height) = self.camera().resolution();

        let mut image = Image::new(width as usize, height as usize);

        let bar = ProgressBar::new(height as u64);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{bar:40.cyan/blue} {pos:>7}/{len:7} [{elapsed_precise}]")
                .unwrap(),
        );

        for y in 0..height {
            for x in 0..width {
                let mut pixel_color = Color::BLACK;
                
                for _ in 0..self.samples_per_pixel {
                    // adding small randoms for anti-aliasing
                    let s = (x as f32 + rand::random::<f32>()) / width as f32;
                    let t = 1.0 - ((y as f32 + rand::random::<f32>()) / height as f32);

                    let ray = self.camera().generate_ray(s, t);
                    pixel_color = pixel_color + self.ray_color(&ray, s, t, self.max_depth);
                }

                pixel_color = pixel_color / self.samples_per_pixel as f32;
                pixel_color = Color::new(
                    pixel_color.r().sqrt(),
                    pixel_color.g().sqrt(),
                    pixel_color.b().sqrt(),
                );
                
                image.set_pixel(x as usize, y as usize, pixel_color);
            }
            bar.inc(1);
        }

        image.save_ppm(path)?;
        Ok(())
    }

    pub fn ray_color(&self, ray: &Ray, u: f32, v: f32, depth: u32) -> Color {
        if depth == 0 {
            return Color::BLACK;
        }

        let mut closest_so_far = 50.0;
        let mut final_hit = None;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, 0.001, closest_so_far) {
                closest_so_far = hit.t;
                final_hit = Some(hit);
            }
        }

        if let Some(hit) = final_hit {
            if let Some(mat) = &hit.material {
                match mat {
                    MaterialType::Lambertian(_) => {
                        if let Some((scattered, attenuation)) = mat.scatter(ray, &hit) {
                            let bounced = self.ray_color(&scattered, u, v, depth - 1);

                            let mut direct = Color::BLACK;
                            for light in &self.lights {
                                direct = direct + light.contribution_from_hit(&self.objects, &hit);
                            }

                            let diffuse_weight = hit.textured_material
                                .as_ref()
                                .map(|tm| tm.get_alpha())
                                .unwrap_or(1.0);

                            return attenuation * (bounced * 0.2 + direct * 0.8 * diffuse_weight);
                        } else {
                            return Color::BLACK;
                        }
                    }

                    MaterialType::Metal(_) | MaterialType::Dielectric(_) => {
                        if let Some((scattered, attenuation)) = mat.scatter(ray, &hit) {
                            let bounced = self.ray_color(&scattered, u, v, depth - 1);
                            return attenuation * bounced;
                        } else {
                            return Color::BLACK;
                        }
                    }
                }
            } else {
                let mut final_color = Color::BLACK;
                for light in &self.lights {
                    final_color = final_color + light.contribution_from_hit(&self.objects, &hit);
                }
                return final_color * hit.color;
            }
        }

        self.background.value_at(u, v, ray.origin())
    }
}
