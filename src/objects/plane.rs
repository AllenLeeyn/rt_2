use crate::core::{HitRecord, Hittable, Point3, Ray, Vec3};
use crate::material::Material;

#[derive(Clone)]
pub struct Plane {
    center: Point3, // Bottom-left corner or reference point
    size: Vec3,     // Size in X and Z (Y is ignored)
    material: Material,
    bounding_box: (Point3, Point3),
}

impl Plane {
    // Create a new plane centered at 'center' with given 'size' and 'texture'
    pub fn new(center: Point3, size: Vec3, material: Material) -> Self {
        let half_size = size / 2.0;

        let min = Point3::new(
            center.x - half_size.x,
            center.y - 1e-4,
            center.z - half_size.z,
        );

        let max = Point3::new(
            center.x + half_size.x,
            center.y + 1e-4,
            center.z + half_size.z,
        );

        Self {
            center,
            size,
            material,
            bounding_box: (min, max),
        }
    }

    fn normal(&self) -> Vec3 {
        Vec3::Y
    }

    fn compute_uv(&self, point: Point3) -> (f32, f32) {
        let half_size = self.size / 2.0;
        let u = (point.x() - (self.center.x() - half_size.x())) / self.size.x();
        let v = (point.z() - (self.center.z() - half_size.z())) / self.size.z();
        (u.clamp(0.0, 1.0), v.clamp(0.0, 1.0))
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    pub fn size(&self) -> Vec3 {
        self.size
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Check if ray is parallel to plane (no intersection)
        if ray.direction().y.abs() < 1e-8 {
            return None;
        }

        // Calculate intersection parameter t
        let t = (self.center.y - ray.origin().y) / ray.direction().y;

        // Check if intersection is within valid range
        if t < t_min || t > t_max {
            return None;
        }

        // Calculate intersection point
        let point = ray.at(t);

        // Check if intersection point is within plane bounds
        let (min, max) = self.bounding_box;
        if point.x() < min.x() || point.x() > max.x() || point.z() < min.z() || point.z() > max.z()
        {
            return None;
        }

        // Sample the texture
        let (u, v) = self.compute_uv(point);
        let color = self.material.value_at(u, v);

        // Calculate surface properties
        let outward_normal = self.normal();
        let (normal, front_face) = HitRecord::face_normal(ray, outward_normal);

        Some(HitRecord {
            p: point,
            normal,
            t,
            color,
            u,
            v,
            front_face,
            material: self.material.clone(),
        })
    }
}
