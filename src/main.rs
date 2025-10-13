use rt_2::core::{Color, Point3, Vec3};
use rt_2::material::{diffuse_light::*, lambertian::*, metal::*};
use rt_2::objects::{Cube, Cylinder, Plane, Sphere};
use rt_2::pixels::{Image, Texture};
use rt_2::scene::Scene;
use std::sync::Arc;
use scenes::*;
use rt_2::particle_system::*;
use clap::Parser;

mod scenes;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Scene number to render (1-4)
    #[arg(short = 's', long = "scene", default_value_t = 5)]
    scene: u32,

    /// Output filename
    #[arg(short = 'o', long = "output", default_value = "output.ppm")]
    output: String,

    /// Resolution width and height
    #[arg(short = 'r', long = "resolution", value_names = &["WIDTH", "HEIGHT"])]
    resolution: Option<Vec<u32>>,
    
    /// Samples per pixel
    #[arg(short = 'q', long = "quality", default_value_t = 128)]
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
        7 => scene_seven(&mut scene),
        8 => scene_eight(&mut scene),
        9 => scene_nine(&mut scene),
        _ => {
            eprintln!("Unknown scene {}, defaulting to scene_five", args.scene);
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
