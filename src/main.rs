use rt_2::core::*;
use rt_2::objects::*;
use rt_2::scene::*;
use rt_2::pixels::*;

fn main() -> std::io::Result<()> {
    let mut scene = Scene::new();

    scene.camera_mut().set(
        Vec3::new(0.0, 0.5, 5.0),
        Point3::new(0.0, 0.5, 1.0),
        Vec3::Y,
        60.0,
        1.0,
        (800, 600));

    scene.set_background(Texture::Gradient(
        Color::PASTEL_BLUE, 
        Color::PASTEL_YELLOW,
        3.142/2.0
    ));
    
    scene.add_object(Cube::new(
        Point3::new(1.0, 0.5, 1.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(1.0, 0.5, 0.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(1.0, 0.5, 2.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(1.0, 0.5, 4.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(1.0, 0.5, 5.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));

    scene.add_object(Cube::new(
        Point3::new(-1.0, 0.5, 1.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(-1.0, 0.5, 2.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(-1.0, 0.5, 4.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(-1.0, 0.5, 5.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(-1.0, 0.5,-1.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(-2.0, 0.5,-2.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(-1.0, 0.5,-2.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(0.0, 0.5,-2.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(1.0, 0.5,-2.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));
    scene.add_object(Cube::new(
        Point3::new(2.0, 0.5,-2.0), // center (1.0 y lifts it above plane)
        1.0, // size (width, height, depth)
        Texture::SolidColor(Color::RED),
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.0, 0.1, 4.0),
        0.1,
        Texture::Gradient(Color::DARK_PURPLE, Color::RED, 1.571),
    ));

    // Add plane to the scene
    let image = Image::load("assets/bg.png")?;
    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Texture::Checkerboard(Color::NEON_GREEN, Color::NEON_LIME, 10.0),
    ));

    scene.add_light(Light::new_directional(
        Point3::new(0.5, -3.0, -0.5),
        Color::WHITE,
        3.0,
    ));

    scene.add_light(Light::new_directional(
        Point3::new(0.0, -3.0, -0.5),
        Color::WHITE,
        0.6,
    ));

    scene.render("output.ppm")?;

    Ok(())
}
