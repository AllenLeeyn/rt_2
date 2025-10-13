use serde::{Deserialize, Serialize};
use crate::core::color::Color;
use crate::core::vec3::{Point3, Vec3};
use crate::pixels::texture::Texture;
use crate::objects::{Sphere, Plane, Cube, Cylinder};
use crate::pixels::image::Image;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneData {
    pub objects: Vec<ObjectData>,
    pub lights: Vec<LightData>,
    pub camera: CameraData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraData {
    pub position: Point3,
    pub look_at: Point3,
    pub up: Vec3,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub resolution: (u32, u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectData {
    Sphere(SphereData),
    Plane(PlaneData),
    Cube(CubeData),
    Cylinder(CylinderData),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SphereData {
    pub center: Point3,
    pub radius: f32,
    pub material: MaterialData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlaneData {
    pub center: Point3,
    pub size: Vec3,
    pub material: MaterialData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CubeData {
    pub center: Point3,
    pub size: f32,
    pub material: MaterialData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CylinderData {
    pub center: Point3,
    pub radius: f32,
    pub height: f32,
    pub material: MaterialData,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MaterialData {
    pub texture: TextureData,
    pub diffuse: f32,
    pub reflectivity: f32,
    pub transparency: f32,
    pub index_of_refraction: f32,
    pub emission: Option<Color>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextureData {
    SolidColor(Color),
    Gradient(Color, Color, f32),
    Checkerboard(Color, Color, f32),
    Image(String),
}

impl Default for TextureData {
    fn default() -> Self {
        TextureData::SolidColor(Color::WHITE)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LightData {
    Point(PointLightData),
    Directional(DirectionalLightData),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PointLightData {
    pub position: Point3,
    pub color: Color,
    pub intensity: f32,
    pub samples: usize,
    pub radius: f32,
    pub softness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DirectionalLightData {
    pub direction: Vec3,
    pub color: Color,
    pub intensity: f32,
}

impl From<TextureData> for Texture {
    fn from(data: TextureData) -> Self {
        match data {
            TextureData::SolidColor(c) => Texture::SolidColor(c),
            TextureData::Gradient(c1, c2, angle) => Texture::Gradient(c1, c2, angle),
            TextureData::Checkerboard(c1, c2, freq) => Texture::Checkerboard(c1, c2, freq),
            TextureData::Image(path) => Texture::Image(Image::load(&path).unwrap()),
        }
    }
}

use crate::material::Material;

impl From<MaterialData> for Material {
    fn from(data: MaterialData) -> Self {
        Material {
            texture: data.texture.into(),
            diffuse: data.diffuse,
            reflectivity: data.reflectivity,
            transparency: data.transparency,
            index_of_refraction: data.index_of_refraction,
            emission: data.emission,
        }
    }
}

impl From<SphereData> for Sphere {
    fn from(data: SphereData) -> Self {
        Sphere::new(data.center, data.radius, data.material.into())
    }
}

impl From<PlaneData> for Plane {
    fn from(data: PlaneData) -> Self {
        Plane::new(data.center, data.size, data.material.into())
    }
}

impl From<CubeData> for Cube {
    fn from(data: CubeData) -> Self {
        Cube::new(data.center, data.size, data.material.into())
    }
}

impl From<CylinderData> for Cylinder {
    fn from(data: CylinderData) -> Self {
        Cylinder::new(data.center, data.radius, data.height, data.material.into())
    }
}

/* impl From<PointLightData> for Light {
    fn from(data: PointLightData) -> Self {
        Light::new_point(data.position, data.color, data.intensity, data.samples, data.radius, data.softness)
    }
}

impl From<DirectionalLightData> for Light {
    fn from(data: DirectionalLightData) -> Self {
        Light::new_directional(data.direction, data.color, data.intensity)
    }
} */