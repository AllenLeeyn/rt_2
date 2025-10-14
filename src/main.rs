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
    Shorthand: -s, Full: -scene, Default value: 3
    Example: cargo run -- -s 2
    Purpose: Select the scene to render

    Output filename flag:
    Shorthand: -o, Full: -output, Default value: output.ppm
    Example: cargo run -- -o my_render.ppm
    Purpose: Specify the output filename

    Resolution width and height flag:
    Shorthand: -r, Full: -resolution, Default value is set individually for each scene
    Example: cargo run -- -r 800 600
    Purpose: Specify the resolution width and height

    Samples per pixel flag:
    Shorthand: -q, Full: -quality, Default value: 32
    Example: cargo run -- -q 128
    Purpose: Specify the samples per pixel

    Depth per pixel flag:
    Shorthand: -d, Full: -depth, Default value: 10
    Example: cargo run -- -d 8
    Purpose: Specify the depth per pixel

    Disable parallelization flag:
    Shorthand: -n, Full: -non-parallelized
    Example: cargo run -- -n
    Purpose: Disable parallelization, used for single-threaded rendering, typically for running the program without over-stressing your cpu

    Info flag:
    Shorthand: -i, Full: -info
    Example: cargo run -- -i
    Purpose: Print the usage info
    ")
}

fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // Handle info flag
    if args.info {
        show_info();
        return Ok(());
    }

    let scene_arg = args.scene.as_str();
    let scenes = vec!["1", "2", "3", "4", "5", "6", "7", "8"];
    let mut scene = if !scenes.contains(&scene_arg) {
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
