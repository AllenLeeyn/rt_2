use crate::core::*;

#[derive(Debug)]
pub struct Camera {
    origin: Point3,
    look_at: Point3,
    vup: Vec3,
    vfov: f32,
    focal_length: f32,
    aspect_ratio: f32,
    resolution: (u32, u32),

    // precompute variables for direction calculation
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        let mut camera = Camera {
            origin: Vec3::ONE,
            look_at: Vec3::ZERO,
            vup: Vec3::Y,
            vfov: 60.0,
            focal_length: 1.0,
            aspect_ratio: 600.0 / 400.0,
            resolution: (600, 400),
            lower_left_corner: Point3::ZERO,
            horizontal: Vec3::ZERO,
            vertical: Vec3::ZERO,
        };
        camera.update_viewport();
        camera
    }

    pub fn resolution(&self) -> (u32, u32) {
        self.resolution
    }

    pub fn set(
        &mut self,
        origin: Point3,
        look_at: Point3,
        vup: Vec3,
        vfov: f32,
        focal_length: f32,
        resolution: (u32, u32),
    ) {
        self.origin = origin;
        self.look_at = look_at;
        self.vup = vup;
        self.vfov = vfov;
        self.focal_length = focal_length;
        self.resolution = resolution;
        self.aspect_ratio = resolution.0 as f32 / resolution.1 as f32;

        self.update_viewport();
    }

    pub fn set_resolution(&mut self, resolution: (u32, u32)) {
        self.resolution = resolution;
        self.aspect_ratio = resolution.0 as f32 / resolution.1 as f32;
        self.update_viewport();
    }

    pub fn update_viewport(&mut self) {
        let theta = self.vfov.to_radians();
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = self.aspect_ratio * viewport_height;

        let w = (self.origin - self.look_at).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        self.horizontal = u * viewport_width * self.focal_length;
        self.vertical = v * viewport_height * self.focal_length;

        self.lower_left_corner = self.origin
            - self.horizontal / 2.0
            - self.vertical / 2.0
            - w * self.focal_length;
    }

    pub fn generate_ray(&self, s: f32, t: f32 ) -> Ray {
        let point_on_plane = self.lower_left_corner
            + self.horizontal * s
            + self.vertical * t;
        let direction = point_on_plane - self.origin;

        Ray::new( self.origin, direction )
    }

    pub fn generate_rays(&self) -> Vec<Vec<Ray>> {
        let (width, height) = self.resolution;
        let mut rays: Vec<Vec<Ray>> = Vec::with_capacity(height as usize);

        for y in 0..height {
            let mut row = Vec::with_capacity(width as usize);

            for x in 0..width {
                let s = (x as f32 + 0.5) / width as f32;
                let t = 1.0 - ((y as f32 + 0.5) / height as f32); // flip y

                let ray = self.generate_ray(s, t);
                row.push(ray);
            }

            rays.push(row);
        }

        rays
    }
}