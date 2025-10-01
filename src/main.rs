use rt_2::core::*;
use rt_2::objects::*;
use rt_2::scene::*;
use rt_2::pixels::*;

fn main() -> std::io::Result<()> {
    let mut scene = Scene::new();

    default_scene(&mut scene);

    scene.render("output.ppm")?; // replace this with your scene setup

    Ok(())
}

fn default_scene(scene: &mut Scene) {

    scene.camera_mut().set(
        Point3::splat(3.0),
        Vec3::ZERO,
        Vec3::Y,
        60.0,
        1.0,
        (400, 300));
    
    scene.add_object(Cube::new(
        Point3::new(-1.5, 0.5, 0.5), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::Gradient(Color::DARK_RED, Color::RED, 1.571),
    ));

    scene.add_object(Cylinder::new(
        Point3::new(2.0, 0.0, -1.0), // base center
        0.25,
        2.0,
        Texture::Checkerboard(Color::BLUE, Color::YELLOW, 1.0),
    ));

    scene.add_object(Sphere::new(
        Point3::new(-0.5, 0.5, 0.0),
        0.5,
        Texture::SolidColor(Color::GREEN),
    ));

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Texture::Checkerboard(Color::GRAY, Color::PASTEL_GRAY, 20.0),
    ));
    
    scene.add_light(Light::new_point(
        Point3::new(0.0, 4.0, 0.0),
        Color::WHITE,
        30.0,
        512,
        0.6,
        50.0
    ));
}