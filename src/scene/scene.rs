use crate::core::*;
use crate::random_double;
use crate::scene::*;
use crate::pixels::*;

use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
    lights: Vec<Light>,
    background: Texture,
    camera: Camera,
    max_depth: u32,
    sample_size: u32,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            background: Texture::SolidColor(Color::BLACK),
            camera: Camera::new(),
            max_depth: 1,
            sample_size: 8,
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

    pub fn set_sample_size(&mut self, size: u32) {
        self.sample_size = size;
    }

    pub fn add_object<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn add_boxed_object(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }

    pub fn render(&mut self, path: &str) -> std::io::Result<()> {
        let (width, height) = self.camera().resolution();
        let mut image = Image::new(width as usize, height as usize);

        // Create progress bar
        let pb = ProgressBar::new(height as u64);
        pb.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>3}/{len:3} lines ({percent}%) {eta}"
            )
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  ")
        );

        println!("🚀 Starting render: {}x{} pixels", width, height);

        // Parallelize over rows
        let rows: Vec<(u32, Vec<Color>)> = (0..height)
            .into_par_iter()
            .map(|y| {
                let mut row_pixels = Vec::with_capacity(width as usize);

                for x in 0..width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                        for _ in 0..self.sample_size {
                        let u = (x as f32 + random_double()) / width as f32;
                        let v = 1.0 - ((y as f32 + random_double()) / height as f32);
                        let ray = self.camera().generate_ray(u, v);
                        pixel_color = pixel_color + self.ray_color(&ray, u , v, self.max_depth);
                    }
                    let color = pixel_color/ self.sample_size as i32;
                    row_pixels.push(color);
                }

                pb.inc(1);
                (y, row_pixels)
            })
            .collect();

        pb.finish();

        println!("💾 Saving to: {}", path);
        for (y, row) in rows {
            for (x, color) in row.into_iter().enumerate() {
                image.set_pixel(x, y as usize, color);
            }
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

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, 1e-6, closest_so_far) {
                closest_so_far = hit.t;
                final_hit = Some(hit);
            }
        }

        if let Some(hit) = final_hit {
            let mut final_color = Color::BLACK;

            for light in &self.lights {
                final_color = final_color + light.contribution_from_hit(&self.objects, &hit, ray);
            }

            // 📌 Emission directly contributes to color
            let emitted = hit.material.emitted(hit.u, hit.v, hit.p);

            if let Some(scatter) = hit.material.scatter(ray, &hit) {
                let bounced_color = self.ray_color(&scatter.scattered_ray, u, v, depth - 1);
                final_color = ( final_color *
                        (1.0 - hit.material.transparency).max(0.01) *
                        (1.0 - hit.material.reflectivity).max(0.01)
                    ) + scatter.attenuation * bounced_color;
            } else {
                final_color = emitted;
            }

            return final_color;
        }
        self.background.value_at(u, v, ray.origin())
    }
}
