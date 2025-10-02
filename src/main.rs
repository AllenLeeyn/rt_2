use rt_2::core::{Color, Point3, Vec3};
use rt_2::objects::{Cube, Cylinder, Plane, Sphere};
use rt_2::pixels::Texture;
use rt_2::scene::Scene;
use rt_2::material::{lambertian::*, metal::*, diffuse_light::*};
use std::sync::Arc;
use rt_2::scene::light::Light;

fn main() -> std::io::Result<()> {
    let mut scene = Scene::new();

    default_scene(&mut scene);

    scene.render("output.ppm")?; // replace this with your scene setup

    Ok(())
}

fn default_scene(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(2.0, 1.0, 3.0),
        Vec3::ZERO,
        Vec3::Y,
        60.0,
        1.0,
        (400, 300),
    );

    let material_ground = Arc::new(Lambertian::new(Texture::SolidColor(Color::new_f32(0.8, 0.8, 0.0))));
    let material_center = Arc::new(Lambertian::new(Texture::SolidColor(Color::new_f32(0.1, 0.2, 0.5))));
    let material_left = Arc::new(Metal::new(Texture::SolidColor(Color::new_f32(0.8, 0.8, 0.8)), 0.3));
    let material_right = Arc::new(Metal::new(Texture::SolidColor(Color::new_f32(0.8, 0.6, 0.2)), 1.0));

    scene.add_object(Plane::new(Point3::new(0.0, -0.5, 0.0), Vec3::new(20.0, 0.0, 20.0), material_ground.clone()));
    scene.add_object(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center.clone()));
    scene.add_object(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone()));
    scene.add_object(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right.clone()));

    let material_cube = Arc::new(Lambertian::new(Texture::SolidColor(Color::new_f32(0.5, 0.5, 0.5))));
    scene.add_object(Cube::new(Point3::new(-1.5, 0.0, 0.5), 1.0, material_cube.clone()));

    let material_cylinder = Arc::new(Metal::new(Texture::SolidColor(Color::new_f32(0.5, 0.5, 0.5)), 0.0));
    scene.add_object(Cylinder::new(Point3::new(2.0, -0.5, -1.0), 0.25, 2.0, material_cylinder.clone()));

    let light_material = Arc::new(DiffuseLight::new(Texture::SolidColor(Color::WHITE)));
    scene.add_object(Sphere::new(Point3::new(0.0, 1.5, 0.0), 0.2, light_material.clone()));

    scene.add_light(Light::new_point(
        Point3::new(0.0, 3.0, 1.0),
        Color::WHITE,
        5.0,
        8,
        10.0,
        10.0,
    ));

    scene.add_light(Light::new_directional(
        Vec3::new(-2.0, -1.0, 1.0),
        Color::WHITE,
        0.2,
    ));
}
