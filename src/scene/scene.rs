use crate::core::*;
use crate::pixels::*;
use crate::scene::camera::Camera;
use crate::scene::light::Light;
use rayon::prelude::*;
use std::ops::{Add, Mul};

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
            samples_per_pixel: 30,
        }
    }

    pub fn set_samples_per_pixel(&mut self, samples: u32) {
        self.samples_per_pixel = samples;
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
                    let mut pixel_color = Vec3::ZERO;
                    for _ in 0..self.samples_per_pixel {
                        let u = (x as f32 + rand::random::<f32>()) / (width - 1) as f32;
                        let v = 1.0 - (y as f32 + rand::random::<f32>()) / (height - 1) as f32;
                        let ray = self.camera().generate_ray(u, v);
                        pixel_color = pixel_color.add(self.ray_color(&ray, self.max_depth));
                    }

                    let color = Color::from_vec3(pixel_color / self.samples_per_pixel as f32);
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

    pub fn ray_color(&self, ray: &Ray, depth: u32) -> Vec3 {
        if depth <= 0 {
            return Vec3::ZERO;
        }

        let mut closest_so_far = 50.0;
        let mut final_hit = None;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, 0.001, closest_so_far) {
                closest_so_far = hit.t;
                final_hit = Some(hit);
            }
        }

        if let Some(hit) = final_hit {
            let emitted = hit.material.emitted(hit.u, hit.v, &hit.p).to_vec3();

            if let Some((attenuation, scattered)) = hit.material.scatter(ray, &hit) {
                // This branch is for materials that scatter (e.g., Lambertian, Metal)
                let scattered_light = attenuation
                    .to_vec3()
                    .mul(self.ray_color(&scattered, depth - 1));

                let mut direct_light = Vec3::ZERO;
                if hit.material.is_diffuse() {
                    for light in &self.lights {
                        direct_light = direct_light
                            .add(light.contribution_from_hit(&self.objects, &hit).to_vec3());
                    }
                }
                return emitted.add(scattered_light).add(direct_light);
            } else {
                // This branch is for materials that don't scatter (e.g., DiffuseLight)
                return emitted;
            }
        }


        //Color::DARK_GRAY.to_vec3()

        let unit_direction = ray.direction().normalize();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::lerp(Color::WHITE, Color::new(0.5, 0.7, 1.0), t).to_vec3()

        //self.background.value_at(u, v, ray.origin())
    }
}
