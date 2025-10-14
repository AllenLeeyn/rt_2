use clap::Parser;
use rand::Rng;
use rt_2::core::*;
use rt_2::material::*;
use rt_2::objects::*;
use rt_2::particle_system::*;
use rt_2::pixels::*;
use rt_2::random_float;
use rt_2::scene::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Scene number to render
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

    /// Depth per pixel
    #[arg(short = 'd', long = "depth", default_value_t = 10)]
    depth: u32,

    /// Disable parallelization
    #[arg(short = 'n', long = "non-parallelized")]
    non_parallelized: bool,

    /// Info
    #[arg(short = 'i', long = "info")]
    info: bool,
}

fn show_info() {
    println!("    Scene number to render flag:
    Shorthand: -s, Full: -\"scene\", Default value: = 3
    Example: cargo run ---s 2
    Purpose: Select the scene to render

    Output filename flag:
    Shorthand: -o, Full: -\"output\", Default value: = \"output.ppm\"
    Example: cargo run -- -o my_render.ppm
    Purpose: Specify the output filename

    Resolution width and height flag:
    Shorthand: -r, Full: -\"resolution\", Default value is set individually for each scene
    Example: cargo run -- -r 800 600
    Purpose: Specify the resolution width and height

    Samples per pixel flag:
    Shorthand: -q, Full: -\"quality\", Default value: = 32
    Example: cargo run -- -q 128
    Purpose: Specify the samples per pixel

    Depth per pixel flag:
    Shorthand: -d, Full: -\"depth\", Default value: = 10
    Example: cargo run -- -d 8
    Purpose: Specify the depth per pixel

    Disable parallelization flag:
    Shorthand: -n, Full: -\"non-parallelized\"
    Example: cargo run -- -n
    Purpose: Disable parallelization, used for single-threaded rendering, typically for running the program without over-stressing your cpu

    Info flag:
    Shorthand: -i', Full: \"info\"
    Example: cargo run -- -i
    Purpose: Print the usage info
    ")
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    if args.info {
        show_info();
        return Ok(());
    }

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
    scene.render(&args.output, !args.non_parallelized)?;

    Ok(())
}

fn scene_one(scene: &mut Scene) {
    scene.set_background(Texture::Gradient(Color::WHITE, Color::LIGHT_BLUE, 90.0));

    scene.camera_mut().set(
        Point3::splat(4.0),
        Vec3::ZERO,
        Vec3::Y,
        60.0,
        1.0,
        (400, 300),
    );

    scene.add_object(Sphere::new(
        Point3::ZERO,
        2.0,
        Material {
            texture: Texture::SolidColor(Color::PASTEL_LIME),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.0, 5.0, 4.0),
        1.0,
        Material {
            texture: Texture::SolidColor(Color::WHITE),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: Some(Color::WHITE * 10.0),
        },
    ));
}

fn scene_two(scene: &mut Scene) {
    scene.set_background(Texture::Gradient(
        Color::BLACK,
        Color::DARK_PURPLE * 0.3,
        90.0,
    ));

    scene.camera_mut().set(
        Point3::splat(3.0),
        Vec3::ZERO,
        Vec3::Y,
        60.0,
        1.0,
        (400, 300),
    );

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(5.0, 0.0, 5.0),
        Material {
            texture: Texture::Checkerboard(Color::LIGHT_GRAY, Color::GRAY, 5.0),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    let image = Image::load("assets/test.png").unwrap();
    scene.add_object(Cube::new(
        Point3::new(0.0, 0.5, 0.0),
        1.0,
        Material {
            texture: Texture::Image(image),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    /*     scene.add_light(Light::new_directional(
        Point3::new(-1.0, -4.0, -4.0),
        Color::WHITE,
        1.0,
    )); */

    scene.add_object(Sphere::new(
        Point3::new(-0.7, 4.0, 2.0),
        1.0,
        Material {
            texture: Texture::SolidColor(Color::WHITE),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: Some(Color::WHITE * 4.0),
        },
    ));
}

fn scene_three(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(0.0, 2.0, 4.0),
        Vec3::new(0.0, 1.5, 0.0),
        Vec3::Y,
        60.0,
        1.0,
        (400, 300),
    );

    scene.set_background(Texture::SolidColor(Color::PASTEL_BLUE));

    scene.add_object(Cube::new(
        Point3::new(-7.0, 5.0, 0.0),
        10.0,
        Material {
            texture: Texture::SolidColor(Color::RED),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Cube::new(
        Point3::new(7.0, 5.0, 0.0),
        10.0,
        Material {
            texture: Texture::SolidColor(Color::GREEN),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Cube::new(
        Point3::new(0.0, 5.0, -7.0),
        10.0,
        Material {
            texture: Texture::SolidColor(Color::WHITE),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Cube::new(
        Point3::new(0.0, 5.0, 10.0),
        10.0,
        Material {
            texture: Texture::SolidColor(Color::WHITE),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Cube::new(
        Point3::new(0.0, 8.5, 0.0),
        10.0,
        Material {
            texture: Texture::SolidColor(Color::WHITE),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Cube::new(
        Point3::new(0.0, 3.9, 0.0),
        1.0,
        Material {
            texture: Texture::SolidColor(Color::WHITE),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: Some(Color::WHITE * 5.0),
        },
    ));

    scene.add_object(Cylinder::new(
        Point3::new(1.5, 0.0, 0.5),
        0.3,
        4.0,
        Material {
            texture: Texture::Checkerboard(Color::CYAN, Color::PASTEL_BLUE, 1.0),
            diffuse: 0.3,
            reflectivity: 0.9,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Cube::new(
        Point3::new(-1.0, 0.4, -1.1),
        0.8,
        Material {
            texture: Texture::SolidColor(Color::ORANGE),
            diffuse: 0.8,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Cube::new(
        Point3::new(-1.0, 0.55, -0.85),
        0.6,
        Material {
            texture: Texture::SolidColor(Color::ORANGE),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: Some(Color::ORANGE * 5.0),
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(-0.4, 0.5, 1.0),
        0.5,
        Material {
            texture: Texture::SolidColor(Color::GREEN),
            diffuse: 0.1,
            reflectivity: 0.1,
            transparency: 0.92,
            index_of_refraction: 1.49,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(-0.4, 0.5, 1.0),
        -0.45,
        Material {
            texture: Texture::SolidColor(Color::GREEN),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.95,
            index_of_refraction: 1.49,
            emission: None,
        },
    ));

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Material {
            texture: Texture::Checkerboard(Color::GRAY, Color::PASTEL_GRAY, 20.0),
            diffuse: 0.3,
            reflectivity: 0.3,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
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
        (400, 300),
    );

    scene.add_object(Cube::new(
        Point3::new(-1.5, 0.5, 0.5),
        1.0,
        Material {
            texture: Texture::Gradient(Color::DARK_RED, Color::RED, 1.571),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Cylinder::new(
        Point3::new(2.0, 0.0, -1.0),
        0.25,
        2.0,
        Material {
            texture: Texture::Checkerboard(Color::BLUE, Color::YELLOW, 1.0),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(-0.5, 0.5, 0.0),
        0.5,
        Material {
            texture: Texture::SolidColor(Color::GREEN),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Material {
            texture: Texture::Checkerboard(Color::GRAY, Color::PASTEL_GRAY, 20.0),
            diffuse: 1.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(-1.5, 6.0, -3.0),
        1.0,
        Material {
            texture: Texture::SolidColor(Color::WHITE),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: Some(Color::BEIGE * 20.0),
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(3.0, 8.0, 3.0),
        1.0,
        Material {
            texture: Texture::SolidColor(Color::WHITE),
            diffuse: 10.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: Some(Color::PASTEL_GREEN * 20.0),
        },
    ));
}

fn scene_five(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(0.0, 0.5, 1.0),
        Vec3::new(0.0, 0.5, 0.0),
        Vec3::Y,
        60.0,
        1.0,
        (400, 300),
    );

    scene.set_background(Texture::Gradient(Color::PASTEL_BLUE, Color::WHITE, 1.571));

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Material {
            texture: Texture::SolidColor(Color::YELLOW),
            diffuse: 0.5,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.0, 0.5, -1.0),
        0.5,
        Material {
            texture: Texture::SolidColor(Color::new(0.7, 0.3, 0.3)),
            diffuse: 0.5,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(-1.0, 0.5, -1.0),
        0.5,
        Material {
            texture: Texture::SolidColor(Color::new(0.8, 0.8, 0.8)),
            diffuse: 0.0,
            reflectivity: 1.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(1.0, 0.5, -1.0),
        0.5,
        Material {
            texture: Texture::SolidColor(Color::new(0.8, 0.7, 0.3)),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 1.0,
            index_of_refraction: 1.6,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(1.0, 0.5, -1.0),
        -0.45,
        Material {
            texture: Texture::SolidColor(Color::DARK_PURPLE),
            diffuse: 0.0,
            reflectivity: 0.,
            transparency: 0.7,
            index_of_refraction: 1.5,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(1.0, 0.5, -1.0),
        0.3,
        Material {
            texture: Texture::SolidColor(Color::DARK_ORANGE),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.6,
            index_of_refraction: 1.5,
            emission: None,
        },
    ));

    /* scene.add_light(Light::new_point(
        Point3::new(0.0, 3.0, 1.0),
        Color::WHITE,
        1.0,
        4,
        0.8,
        1.0,
    )); */

    scene.add_object(Sphere::new(
        Point3::new(0.0, 3.0, 1.0),
        1.0,
        Material {
            texture: Texture::SolidColor(Color::WHITE),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: Some(Color::WHITE * 2.0),
        },
    ));
}

fn scene_six(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(4.0, 2.0, 4.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::Y,
        60.0,
        1.0,
        (400, 300),
    );

    scene.set_background(Texture::Gradient(Color::DARK_BLUE, Color::DARK_CYAN, 1.571));

    scene.add_object(Cylinder::new(
        Point3::ZERO,
        0.2,
        1.2,
        Material {
            texture: Texture::SolidColor(Color::new(0.8, 0.6, 0.4)),
            diffuse: 0.5,
            reflectivity: 0.5,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(-4.0, 6.0, -4.0),
        0.75,
        Material {
            texture: Texture::SolidColor(Color::ORANGE),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: Some(Color::ORANGE * 10.0),
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.0, 1.92, 0.0),
        0.75,
        Material {
            texture: Texture::SolidColor(Color::GREEN),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.9,
            index_of_refraction: 1.5,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.0, 1.92, 0.0),
        -0.65,
        Material {
            texture: Texture::SolidColor(Color::GREEN),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.95,
            index_of_refraction: 1.5,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.6, 1.2, 0.0),
        0.4,
        Material {
            texture: Texture::SolidColor(Color::GREEN),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.9,
            index_of_refraction: 1.5,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.6, 1.2, 0.0),
        -0.35,
        Material {
            texture: Texture::SolidColor(Color::GREEN),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.95,
            index_of_refraction: 1.5,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.0, 1.3, 0.6),
        0.3,
        Material {
            texture: Texture::SolidColor(Color::GREEN),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.9,
            index_of_refraction: 1.5,
            emission: None,
        },
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.0, 1.3, 0.6),
        -0.25,
        Material {
            texture: Texture::SolidColor(Color::GREEN),
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: 0.95,
            index_of_refraction: 1.5,
            emission: None,
        },
    ));

    // Setup ParticleSys to create random spheres within a box
    let grounds = ParticleSys::new(
        Point3::new(-5.0, -0.5, -5.0), // min corner
        Point3::new(5.0, 0.2, 5.0),    // max corner
        100,                           // number of particles
        |i: usize, pos: Point3| {
            let grid_cols = 10;
            let col = i % grid_cols;
            let row = i / grid_cols;

            let spacing = 1.0;
            let x = -5.0 + col as f32 * spacing;
            let z = -5.0 + row as f32 * spacing;

            let size = 1.0 + random_float() * 0.02;

            let green_shade = Color::new(
                0.1 + random_float() * 0.1, // R
                0.5 + random_float() * 0.5, // G
                0.1 + random_float() * 0.1, // B
            );
            let material = Material {
                texture: Texture::SolidColor(green_shade),
                diffuse: 1.0,
                reflectivity: 0.0,
                transparency: 0.0,
                index_of_refraction: 0.0,
                emission: None,
            };

            Box::new(Cube::new(Point3::new(x, pos.y(), z), size, material)) as Box<dyn Hittable>
        },
        0.15,
    );

    // Generate and add particles to the scene
    for ground in grounds.generate() {
        scene.add_boxed_object(ground);
    }

    let stars = ParticleSys::new(
        Point3::new(-30.0, 10.0, -30.0), // min corner
        Point3::new(0.0, 25.0, 0.0),     // max corner
        40,                              // number of particles
        |_i: usize, pos: Point3| {
            let material = Material {
                texture: Texture::SolidColor(Color::WHITE),
                diffuse: 0.0,
                reflectivity: 0.0,
                transparency: 0.0,
                index_of_refraction: 0.0,
                emission: Some(Color::WHITE),
            };

            Box::new(Sphere::new(pos, 0.1, material)) as Box<dyn Hittable>
        },
        3.0,
    );

    // Generate and add particles to the scene
    for star in stars.generate() {
        scene.add_boxed_object(star);
    }
}

fn random_material() -> Material {
    let mut rng = rand::rng();

    let color = Color::new(random_float(), random_float(), random_float());
    let texture = Texture::SolidColor(color);

    match rng.random_range(0..3) {
        0 => Material {
            texture,
            diffuse: 0.5,
            reflectivity: 0.0,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
        1 => Material {
            texture,
            diffuse: rng.random_range(0.01..0.2),
            reflectivity: 0.9,
            transparency: 0.0,
            index_of_refraction: 0.0,
            emission: None,
        },
        2 => Material {
            texture,
            diffuse: 0.0,
            reflectivity: 0.0,
            transparency: rng.random_range(0.5..0.95),
            index_of_refraction: rng.random_range(1.3..1.7),
            emission: None,
        },
        _ => unreachable!(),
    }
}
