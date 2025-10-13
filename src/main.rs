use clap::Parser;
use rt_2::scene::Scene;
use scenes::*;

mod scenes;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Scene number to render (1-4), or 0 to load from file
    #[arg(short = 's', long = "scene", default_value = "3")]
    scene: String,

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
    let scene_arg = args.scene.as_str();
    let mut scene = if scene_arg != "1" && scene_arg != "2" && scene_arg != "3" && scene_arg != "4"
    {
        match Scene::load_from_file(&scene_arg) {
            Ok(s) => {
                println!("Loaded scene from {}.", scene_arg);
                s
            }
            Err(e) => {
                eprintln!(
                    "Could not load scene from {}: {}. Falling back to scene three.",
                    scene_arg, e
                );
                let mut s = Scene::new();
                scene_three(&mut s);
                s
            }
        }
    } else {
        let mut s = Scene::new();
        match scene_arg {
            "1" => scene_one(&mut s),
            "2" => scene_two(&mut s),
            "3" => scene_three(&mut s),
            "4" => scene_four(&mut s),
            "5" => scene_five(&mut s),
            "6" => scene_six(&mut s),
            "7" => scene_seven(&mut s),
            "8" => scene_eight(&mut s),
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
    scene.render(&args.output, !args.non_parallelized)?;

    Ok(())
}
