use serde::{Deserialize, Serialize};
use crate::core::color::Color;
use crate::core::vec3::{Point3, Vec3};
use crate::pixels::texture::Texture;
use crate::scene::light::Light;
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SphereData {
    pub center: Point3,
    pub radius: f32,
    pub texture: TextureData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaneData {
    pub center: Point3,
    pub size: Vec3,
    pub texture: TextureData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CubeData {
    pub center: Point3,
    pub size: f32,
    pub texture: TextureData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CylinderData {
    pub center: Point3,
    pub radius: f32,
    pub height: f32,
    pub texture: TextureData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TextureData {
    SolidColor(Color),
    Gradient(Color, Color, f32),
    Checkerboard(Color, Color, f32),
    Image(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LightData {
    Point(PointLightData),
    Directional(DirectionalLightData),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointLightData {
    pub position: Point3,
    pub color: Color,
    pub intensity: f32,
    pub samples: usize,
    pub radius: f32,
    pub softness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl From<SphereData> for Sphere {
    fn from(data: SphereData) -> Self {
        Sphere::new(data.center, data.radius, data.texture.into())
    }
}

impl From<PlaneData> for Plane {
    fn from(data: PlaneData) -> Self {
        Plane::new(data.center, data.size, data.texture.into())
    }
}

impl From<CubeData> for Cube {
    fn from(data: CubeData) -> Self {
        Cube::new(data.center, data.size, data.texture.into())
    }
}

impl From<CylinderData> for Cylinder {
    fn from(data: CylinderData) -> Self {
        Cylinder::new(data.center, data.radius, data.height, data.texture.into())
    }
}

impl From<PointLightData> for Light {
    fn from(data: PointLightData) -> Self {
        Light::new_point(data.position, data.color, data.intensity, data.samples, data.radius, data.softness)
    }
}

impl From<DirectionalLightData> for Light {
    fn from(data: DirectionalLightData) -> Self {
        Light::new_directional(data.direction, data.color, data.intensity)
    }
}