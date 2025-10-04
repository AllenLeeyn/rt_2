use crate::core::*;
use crate::pixels::*;
use crate::scene::*;
use rayon::prelude::*;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
    lights: Vec<Light>,
    background: Texture,
    camera: Camera,
    max_depth: u32,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            background: Texture::SolidColor(Color::BLACK),
            camera: Camera::new(),
            max_depth: 1,
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

        // Parallelize over rows
        let rows: Vec<(u32, Vec<Color>)> = (0..height)
            .into_par_iter()
            .map(|y| {
                let mut row_pixels = Vec::with_capacity(width as usize);

                for x in 0..width {
                    let u = (x as f32 + 0.5) / (width - 1) as f32;
                    let v = 1.0 - (y as f32 + 0.5) / (height - 1) as f32;
                    let ray = self.camera().generate_ray(u, v);
                    let color = self.ray_color(&ray, u, v, self.max_depth);

                    row_pixels.push(color);
                }

                bar.inc(1); // safe: indicatif ProgressBar is internally synchronized
                (y, row_pixels)
            })
            .collect();

        // Write pixels back into image in order
        for (y, row) in rows {
            for (x, color) in row.into_iter().enumerate() {
                image.set_pixel(x, y as usize, color);
            }
        }

        bar.finish();

        image.save_ppm(path)?;
        Ok(())
    }

    pub fn ray_color(&self, ray: &Ray, u: f32, v: f32, _depth: u32) -> Color {
        let mut closest_so_far = 50.0;
        let mut final_hit = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, 0.001, closest_so_far) {
                closest_so_far = hit.t;
                final_hit = Some(hit);
            }
        }

        if let Some(hit) = final_hit {
            let mut final_color = Color::BLACK;

            for light in &self.lights {
                final_color = final_color + light.contribution_from_hit(&self.objects, &hit);
            }

            return final_color;
        }
        self.background.value_at(u, v, ray.origin())
    }
}
