use rand::Rng;
use rt_2::core::*;
use rt_2::material::*;
use rt_2::objects::*;
use rt_2::random_double;
use rt_2::scene::*;
use rt_2::pixels::*;
use rt_2::particle_system::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Scene number to render (1-4)
    #[arg(short = 's', long = "scene", default_value_t = 3)]
    scene: u32,

    /// Output filename
    #[arg(short = 'o', long = "output", default_value = "output.ppm")]
    output: String,

    /// Resolution width and height
    #[arg(short = 'r', long = "resolution", value_names = &["WIDTH", "HEIGHT"])]
    resolution: Option<Vec<u32>>,
    
    /// Samples per pixel
    #[arg(short = 'q', long = "quality", default_value_t = 32)]
    samples: u32,

    /// depth per pixel
    #[arg(short = 'd', long = "depth", default_value_t = 10)]
    depth: u32,
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let mut scene = Scene::new();

    // Select scene
    match args.scene {
        1 => scene_one(&mut scene),
        2 => scene_two(&mut scene),
        3 => scene_three(&mut scene),
        4 => scene_four(&mut scene),
        5 => scene_five(&mut scene),
        6 => scene_six(&mut scene),
        _ => {
            eprintln!("Unknown scene {}, defaulting to scene_three", args.scene);
            scene_three(&mut scene);
        }
    }

    // Set resolution if provided and exactly 2 values passed
    if let Some(res) = &args.resolution {
        if res.len() == 2 {
            scene.camera_mut().set_resolution((res[0], res[1]));
        } else {
            eprintln!("Resolution requires exactly two values: width and height");
        }
    }

    scene.set_sample_size(args.samples);
    scene.set_max_depth(args.depth);
    scene.render(&args.output)?;

    Ok(())
}

fn scene_one(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::splat(4.0),
        Vec3::ZERO,
        Vec3::Y,
        60.0,
        1.0,
        (400, 300));

    scene.add_object(Sphere::new(
        Point3::ZERO,
        2.0,
        Material{
            texture: Texture::SolidColor(Color::PASTEL_LIME),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));
    
    scene.add_light(Light::new_directional(
        Point3::new(0.0, -4.0, -4.0),
        Color::WHITE,
        1.0,
    ));
}

fn scene_two(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::splat(3.0),
        Vec3::ZERO,
        Vec3::Y,
        60.0,
        1.0,
        (400, 300));

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(5.0, 0.0, 5.0),
        Material{
            texture: Texture::Checkerboard(
                Color::LIGHT_GRAY,
                Color::GRAY,
                5.0
            ),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));
    
    let image = Image::load("assets/test.png").unwrap();
    scene.add_object(Cube::new(
        Point3::new(0.0, 0.5, 0.0),
        1.0,
        Material{
            texture: Texture::Image(image),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));
    
    scene.add_light(Light::new_directional(
        Point3::new(-1.0, -4.0, -4.0),
        Color::WHITE,
        1.0,
    ));
}

fn scene_three(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::splat(3.0),
        Vec3::new(0.0, 1.5, 0.0),
        Vec3::Y,
        60.0,
        1.0,
        (400, 300));
    
    scene.add_object(Cube::new(
        Point3::new(-2.0, 1.0, 0.0),
        2.0,
        Material{
            texture: Texture::SolidColor(Color::RED),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.9,
            ior: 1.7,
            emission: None,
            specular: 20.0,
            shininess: 128.0,
        },
    ));

    scene.add_object(Cube::new(
        Point3::new(-2.75, 2.25, -0.75),
        0.5,
        Material{
            texture: Texture::SolidColor(Color::NEON_PINK),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior:0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Cylinder::new(
        Point3::new(1.0, 0.0, -3.5),
        0.3,
        2.0,
        Material{
            texture: Texture::Checkerboard(Color::YELLOW, Color::PASTEL_BLUE, 1.0),
            diffuse: 0.0,
            reflectivity: 0.8,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 8.0,
            shininess: 16.0,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.5, 3.0, -0.5),
        -0.2,
        Material{
            texture: Texture::SolidColor(Color::YELLOW),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.9,
            ior: 1.0,
            emission: None,
            specular: 4.0,
            shininess: 4.0,
        },
    ));

    scene.add_light(Light::new_point(
        Point3::new(0.5, 3.0, -0.5),
        Color::WHITE,
        0.5,
        1,
        0.1,
        1.0
    ));

    scene.add_object(Sphere::new(
        Point3::new(-0.5, 0.5, 0.0),
        0.5,
        Material{
            texture: Texture::SolidColor(Color::LIGHT_GREEN),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.9,
            ior: 1.5,
            emission: None,
            specular: 30.0,
            shininess: 1024.0,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(-0.5, 0.5, 0.0),
        -0.45,
        Material{
            texture: Texture::SolidColor(Color::DARK_GRAY),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.9,
            ior: 1.5,
            emission: None,
            specular: 0.0,
            shininess: 128.0,
        },
    ));

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Material{
            texture: Texture::Checkerboard(Color::GRAY, Color::PASTEL_GRAY, 20.0),
            diffuse: 1.00,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

}

fn scene_four(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(-2.0, 2.0, -4.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::Y,
        60.0,
        1.0,
        (400, 300));
    
    scene.add_object(Cube::new(
        Point3::new(-1.5, 0.5, 0.5),
        1.0,
        Material{
            texture: Texture::Gradient(Color::DARK_RED, Color::RED, 1.571),
            diffuse: 0.1,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Cylinder::new(
        Point3::new(2.0, 0.0, -1.0),
        0.25,
        2.0,
        Material{
            texture: Texture::Checkerboard(Color::BLUE, Color::YELLOW, 1.0),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(-0.5, 0.5, 0.0),
        0.5,
        Material{
            texture: Texture::SolidColor(Color::GREEN),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Material{
            texture: Texture::Checkerboard(Color::GRAY, Color::PASTEL_GRAY, 20.0),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));
    scene.add_light(Light::new_directional(
        Point3::new(-1.5, -6.0, 3.0),
        Color::WHITE,
        0.05,
    ));

    scene.add_light(Light::new_directional(
        Point3::new(-3.0, -8.0, 3.0),
        Color::WHITE,
        0.4,
    ));
}

fn scene_five(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(0.0, 0.5, 1.0),
        Vec3::new(0.0, 0.5, 0.0),
        Vec3::Y,
        60.0,
        1.0,
        (400, 300));

    scene.set_background(
        Texture::Gradient(Color::PASTEL_BLUE,
            Color::WHITE, 1.571));

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Material{
            texture: Texture::SolidColor(Color::YELLOW),
            diffuse: 0.5,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.0, 0.5, -1.0),
        0.5,
        Material{
            texture: Texture::SolidColor(Color::new(0.7, 0.3, 0.3)),
            diffuse: 0.5,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(-1.0, 0.5, -1.0),
        0.5,
        Material{
            texture: Texture::SolidColor(Color::new(0.8, 0.8, 0.8)),
            diffuse: 0.0,
            reflectivity: 1.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(1.0, 0.5, -1.0),
        0.5,
        Material{
            texture: Texture::SolidColor(Color::new(0.8, 0.7, 0.3)),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 1.0,
            ior: 1.6,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(1.0, 0.5, -1.0),
        -0.45,
        Material{
            texture: Texture::SolidColor(Color::DARK_PURPLE),
            diffuse: 0.0,
            reflectivity: 0.,
            transparency: 0.7,
            ior: 1.5,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(1.0, 0.5, -1.0),
        0.3,
        Material{
            texture: Texture::SolidColor(Color::DARK_ORANGE),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.6,
            ior: 1.5,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_light(Light::new_point(
        Point3::new(0.0, 3.0, 1.0),
        Color::WHITE,
        1.0,
        4,
        0.8,
        1.0
    ));
}

fn scene_six(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(-0.1, 0.7, 0.3),
        Vec3::new(-0.05, 0.57, 0.0),
        Vec3::Y,
        60.0,
        1.0,
        (400, 300));

    scene.set_background(
        Texture::Gradient(Color::PASTEL_BLUE,
            Color::WHITE, 1.571));

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Material{
            texture: Texture::SolidColor(Color::PASTEL_BLUE),
            diffuse: 0.5,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.25, 0.4, -2.0),
        0.4,
        Material{
            texture: Texture::SolidColor(Color::DARK_GRAY),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.9,
            ior: 1.5,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.0, 0.4, -3.0),
        0.4,
        Material{
            texture: Texture::SolidColor(Color::DARK_ORANGE),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.5, 0.4, -1.0),
        0.4,
        Material{
            texture: Texture::SolidColor(Color::GRAY),
            diffuse: 0.0,
            reflectivity: 1.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
    ));

    // Setup ParticleSys to create random spheres within a box
    let particle_sys = ParticleSys::new(
        Point3::new(-5.0, 0.0, -5.0), // min corner
        Point3::new(5.0, 0.01, 5.0),  // max corner
        1000,                          // number of particles
        |pos: Point3| {
            let radius = 0.03 + random_double() * 0.05;
            let material = random_material();

            Box::new(Sphere::new(
                Point3::new(pos.x(), radius, pos.z()),
                radius,
                material
            )) as Box<dyn Hittable>
        },
        0.1
    );

    // Generate and add particles to the scene
    for particle in particle_sys.generate() {
        scene.add_boxed_object(particle);
    }

    scene.add_light(Light::new_point(
        Point3::new(0.0, 3.0, 1.0),
        Color::WHITE,
        1.0,
        4,
        0.8,
        1.0
    ));
}

fn random_material() -> Material {
    let mut rng = rand::rng();

    let color = Color::new(random_double(), random_double(), random_double());
    let texture = Texture::SolidColor(color);

    match rng.random_range(0..3) {
        0 => Material {
            texture,
            diffuse: 0.5,
            reflectivity: 0.0,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
        1 => Material {
            texture,
            diffuse: rng.random_range(0.0..0.2),
            reflectivity: 0.9,
            transparency: 0.0,
            ior: 0.0,
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
        2 => Material {
            texture,
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: rng.random_range(0.5..0.95),
            ior: rng.random_range(1.3..1.7),
            emission: None,
            specular: 0.0,
            shininess: 0.0,
        },
        _ => unreachable!(),
    }
}