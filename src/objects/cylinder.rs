use crate::core::{HitRecord, Hittable, Point3, Ray, Vec3};
use crate::material::MaterialType;
use crate::pixels::texture::{Texture, TexturedMaterial};

#[derive(Clone)]
pub struct Cylinder {
    center: Point3,
    radius: f32,
    height: f32,
    texture: Texture,
    material: Option<MaterialType>,
    textured_material: Option<TexturedMaterial>,
    bounding_box: (Point3, Point3),
}

impl Cylinder {
    pub fn new(center: Point3, radius: f32, height: f32, texture: Texture) -> Self {
        let min = Point3::new(center.x() - radius, center.y(), center.z() - radius);

        let max = Point3::new(
            center.x() + radius,
            center.y() + height,
            center.z() + radius,
        );

        Self {
            center,
            radius,
            height,
            texture,
            material: None,
            textured_material: None,
            bounding_box: (min, max),
        }
    }

    pub fn set_material(&mut self, material: MaterialType) {
        self.material = Some(material);
    }

    pub fn set_textured_material(&mut self, textured_material: TexturedMaterial) {
        self.textured_material = Some(textured_material);
    }

    pub fn material(&self) -> Option<&MaterialType> {
        self.material.as_ref()
    }

    pub fn textured_material(&self) -> Option<&TexturedMaterial> {
        self.textured_material.as_ref()
    }

    fn bounding_box(&self) -> (Point3, Point3) {
        self.bounding_box
    }

    fn compute_normal(&self, p: Point3) -> Vec3 {
        let dx = p.x() - self.center.x();
        let dz = p.z() - self.center.z();
        Vec3::new(dx, 0.0, dz).normalize() // Only radial component
    }

    fn compute_uv(&self, p: Point3) -> (f32, f32) {
        let (min, max) = self.bounding_box();
        let u = (p.x() - min.x()) / (max.x() - min.x());
        let v = (p.y() - min.y()) / (max.y() - min.y());
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }

    fn hit_cap(&self, ray: &Ray, t_min: f32, t_max: f32, y: f32) -> Option<HitRecord> {
        let dir_y = ray.direction().y();
        if dir_y.abs() < 1e-6 {
            return None;
        }

        let t = (y - ray.origin().y()) / dir_y;
        if t < t_min || t > t_max {
            return None;
        }

        let p = ray.at(t);
        let dx = p.x() - self.center.x();
        let dz = p.z() - self.center.z();

        if dx * dx + dz * dz <= self.radius * self.radius {
            let outward_normal = if y > self.center.y() {
                Vec3::Y
            } else {
                -Vec3::Y
            };
            let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
            let (u, v) = self.compute_uv(p);
            let color = self.texture.value_at(u, v, p);

            return Some(HitRecord {
                p,
                normal,
                t,
                color,
                u,
                v,
                front_face,
                material: self.material.clone(),
                textured_material: self.textured_material.clone(),
            });
        }

        None
    }

    fn hit_side(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;

        let dx = ray.direction().x();
        let dz = ray.direction().z();
        let ox = oc.x();
        let oz = oc.z();

        let a = dx * dx + dz * dz;
        let b = 2.0 * (ox * dx + oz * dz);
        let c = ox * ox + oz * oz - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrt_d = discriminant.sqrt();
        let inv_2a = 0.5 / a;

        for &t in [(-b - sqrt_d) * inv_2a, (-b + sqrt_d) * inv_2a].iter() {
            if t < t_min || t > t_max {
                continue;
            }

            let p = ray.at(t);
            let y = p.y();
            let y_min = self.center.y();
            let y_max = y_min + self.height;

            if y >= y_min && y <= y_max {
                let outward_normal = self.compute_normal(p);
                let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);
                let (u, v) = self.compute_uv(p);
                let color = self.texture.value_at(u, v, p);

                return Some(HitRecord {
                    p,
                    normal,
                    t,
                    color,
                    u,
                    v,
                    front_face,
                    material: self.material.clone(),
                    textured_material: self.textured_material.clone(),
                });
            }
        }

        None
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let y_bottom = self.center.y();
        let y_top = y_bottom + self.height;

        if let Some(hit) = self.hit_cap(ray, t_min, t_max, y_top) {
            return Some(hit);
        }

        if let Some(hit) = self.hit_side(ray, t_min, t_max) {
            return Some(hit);
        }

        if let Some(hit) = self.hit_cap(ray, t_min, t_max, y_bottom) {
            return Some(hit);
        }
        None
    }
}
