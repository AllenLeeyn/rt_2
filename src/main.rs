use rt_2::core::*;
use rt_2::objects::*;
use rt_2::scene::*;
use rt_2::pixels::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Scene number to render (1-4), or 0 to load from file
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

    let mut scene = if args.scene == 0 {
        match Scene::load_from_file("scene.json") {
            Ok(s) => {
                println!("Loaded scene from scene.json");
                s
            },
            Err(e) => {
                eprintln!("Could not load scene from scene.json: {}. Falling back to scene three.", e);
                let mut s = Scene::new();
                scene_three(&mut s);
                s
            }
        }
    } else {
        let mut s = Scene::new();
        match args.scene {
            1 => scene_one(&mut s),
            2 => scene_two(&mut s),
            3 => scene_three(&mut s),
            4 => scene_four(&mut s),
            _ => {
                eprintln!("Unknown scene {}, defaulting to scene_three", args.scene);
                scene_three(&mut s);
            }
        }
        s
    };

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
        Texture::SolidColor(Color::PASTEL_LIME),
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
        Texture::Checkerboard(
            Color::LIGHT_GRAY,
            Color::GRAY,
            5.0
        ),
    ));
    
    let image = Image::load("assets/test.png").unwrap();
    scene.add_object(Cube::new(
        Point3::new(0.0, 0.5, 0.0),
        1.0,
        Texture::Image(image),
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
        Vec3::ZERO,
        Vec3::Y,
        60.0,
        1.0,
        (400, 300));
    
    scene.add_object(Cube::new(
        Point3::new(-1.5, 0.5, 0.5),
        1.0, // size (width, height, depth)
        Texture::Gradient(Color::DARK_RED, Color::RED, 1.571),
    ));

    scene.add_object(Cylinder::new(
        Point3::new(2.0, 0.0, -1.0),
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
        Point3::new(0.0, 3.0, 0.0),
        Color::WHITE,
        1.0,
        4,
        1.0,
        50.0
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
        Texture::Gradient(Color::DARK_RED, Color::RED, 1.571),
    ));

    scene.add_object(Cylinder::new(
        Point3::new(2.0, 0.0, -1.0),
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