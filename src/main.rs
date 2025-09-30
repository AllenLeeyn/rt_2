use rt_2::core::*;
use rt_2::objects::*;
use rt_2::scene::*;
use rt_2::pixels::*;

fn main() -> std::io::Result<()> {
    let mut scene = Scene::new();

    scene.camera_mut().set(
        Vec3::new(5.0, 4.0, 3.0),
        Vec3::ZERO,
        Point3::new(0.0, 1.0, 0.2),
        70.0,
        1.0,
        (800, 600));

    scene.set_background(Texture::Gradient(
        Color::PASTEL_BLUE, 
        Color::PASTEL_YELLOW,
        3.142/2.0
    ));
    
    scene.add_object(Cube::new(
        Point3::new(0.0, 1.0, -1.0), // center (1.0 y lifts it above plane)
        2.0, // size (width, height, depth)
        Texture::Checkerboard(Color::MAGENTA, Color::PASTEL_CYAN, 1.99),
    ));

    scene.add_object(Cylinder::new(
        Point3::new(3.0, 0.0, 1.0), // base center
        0.5,
        1.0,
        Texture::Gradient(Color::YELLOW, Color::BLUE, 3.142),
    ));

    scene.add_object(Sphere::new(
        Point3::new(-1.0, 0.0, 2.0),
        1.0,
        Texture::Gradient(Color::PASTEL_PURPLE, Color::RED, 0.0),
    ));

    // Add plane to the scene
    let image = Image::load("assets/bg.png")?;
    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Texture::Image(image),
    ));

    scene.add_light(Light::new_directional(
        Point3::new(3.0, -5.0, -5.0),
        Color::WHITE,
        1.0,
    ));

    scene.add_light(Light::new_point(
        Point3::new(3.0, 5.0, 5.0),
        Color::WHITE,
        20.0,
        16,
        0.5,
        100.0,
    ));

    scene.render("output.ppm")?;

    Ok(())
}
