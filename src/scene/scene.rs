use crate::core::*;
use crate::random_double;
use crate::scene::*;
use crate::pixels::*;

use indicatif::ProgressBar;
use indicatif::ProgressStyle;

pub struct Scene {
    objects: Vec<Box<dyn Hittable>>,
    lights: Vec<Light>,
    background: Texture,
    camera: Camera,
    max_depth: u32,
    sample_per_pixel: i32,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
            lights: Vec::new(),
            background: Texture::SolidColor(Color::BLACK),
            camera: Camera::new(),
            max_depth: 8,
            sample_per_pixel: 32,
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

        for y in 0..height {
            for x in 0..width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.sample_per_pixel {
                    let s = (x as f32 + random_double()) / width as f32;
                    let t = 1.0 - ((y as f32 + random_double()) / height as f32);

                    let ray = self.camera().generate_ray(s, t);
                    pixel_color = pixel_color + self.ray_color(&ray, s , t, self.max_depth);
                }
                let color = pixel_color/ self.sample_per_pixel;
                image.set_pixel(x as usize, y as usize, color);
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

            if let Some(scatter) = hit.material.scatter(ray, &hit) {
                let bounced_color = self.ray_color(&scatter.scattered_ray, u, v, depth - 1);
                final_color = final_color + scatter.attenuation * bounced_color;
            }

            return final_color;
        }
        self.background.value_at(u, v, ray.origin())
    }

}
