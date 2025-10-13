use crate::core::*;
use crate::pixels::*;
use crate::random_float;
use crate::scene::*;
use crate::scene::storage::*;
use crate::objects::{Cube, Cylinder, Plane, Sphere};

use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use rayon::prelude::*;

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
    background: Texture,
    camera: Camera,
    max_depth: u32,
    sample_size: u32,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            background: Texture::SolidColor(Color::BLACK),
            camera: Camera::new(),
            max_depth: 1,
            sample_size: 8,
        }
    }

    pub fn load_from_file(path: &str) -> Result<Scene, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path)?;
        let scene_data: SceneData = serde_json::from_str(&data)?;

        let mut scene = Scene::new();

        for object in scene_data.objects {
            match object {
                ObjectData::Sphere(s) => {
                    let sphere: Sphere = s.into();
                    scene.add_object(sphere);
                }
                ObjectData::Plane(p) => {
                    let plane: Plane = p.into();
                    scene.add_object(plane);
                }
                ObjectData::Cube(c) => {
                    let cube: Cube = c.into();
                    scene.add_object(cube);
                }
                ObjectData::Cylinder(cy) => {
                    let cylinder: Cylinder = cy.into();
                    scene.add_object(cylinder);
                }
            }
        }

        let camera_data = scene_data.camera;
        scene.camera_mut().set(
            camera_data.position,
            camera_data.look_at,
            camera_data.up,
            camera_data.fov,
            camera_data.aspect_ratio,
            camera_data.resolution,
        );

        Ok(scene)
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

    pub fn render(&mut self, path: &str, parallelized: bool) -> std::io::Result<()> {
        let (width, height) = self.camera().resolution();
        let mut image = Image::new(width as usize, height as usize);

        // Create progress bar
        let prog_bar = ProgressBar::new(height as u64);
        prog_bar.set_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>3}/{len:3} lines ({percent}%) {eta}"
            )
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏  ")
        );

        println!("Starting render: {width}x{height} pixels");
        if parallelized {
            println!("Using parallelized rendering");
        } else {
            println!("Using single-threaded rendering");
        }

        // Common rendering logic for each row
        let render_row = |y: u32| {
            let mut row_pixels = Vec::with_capacity(width as usize);
            for x in 0..width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_size {
                    let horizontal_offset = (x as f32 + random_float()) / width as f32;
                    let vertical_offset = 1.0 - ((y as f32 + random_float()) / height as f32);
                    let ray = self
                        .camera()
                        .generate_ray(horizontal_offset, vertical_offset);
                    pixel_color = pixel_color
                        + self.ray_color(&ray, horizontal_offset, vertical_offset, self.max_depth);
                }
                let color = pixel_color / self.sample_size as i32;
                row_pixels.push(color);
            }
            prog_bar.inc(1);
            (y, row_pixels)
        };

        let rows: Vec<(u32, Vec<Color>)> = if parallelized {
            (0..height).into_par_iter().map(render_row).collect()
        } else {
            (0..height).map(render_row).collect()
        };

        prog_bar.finish();

        println!("Saving to: {path}");
        for (y, row) in rows {
            for (x, color) in row.into_iter().enumerate() {
                image.set_pixel(x, y as usize, color);
            }
        }

        image.save_ppm(path)?;
        Ok(())
    }

    pub fn ray_color(
        &self,
        ray: &Ray,
        horizontal_offset: f32,
        vertical_offset: f32,
        depth: u32,
    ) -> Color {
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
            let glow = hit.material.emission.unwrap_or(Color::BLACK);

            let mut final_color = glow;

            if depth > 0 {
                if let Some(scatter) = hit.material.scatter(ray, &hit) {
                    let bounced = self.ray_color(
                        &scatter.scattered_ray,
                        horizontal_offset,
                        vertical_offset,
                        depth - 1,
                    );
                    final_color = final_color + scatter.attenuation * bounced;
                }
            }

            return final_color;
        }
        self.background.value_at(horizontal_offset, vertical_offset)
    }
}
