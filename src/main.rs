use clap::Parser;
use rand::Rng;
use rt_2::core::*;
use rt_2::material::*;
use rt_2::pixels::Texture;
use rt_2::random_float;
use rt_2::scene::Scene;
use scenes::*;

mod scenes;

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

    /// Disable parallelization (use single-threaded rendering, for testing without over-stressing gpu)
    #[arg(short = 'n', long = "non-parallelized")]
    non_parallelized: bool,
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
        7 => scene_seven(&mut scene),
        8 => scene_eight(&mut scene),
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

#[allow(dead_code)]
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
