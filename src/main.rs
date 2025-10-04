use rt_2::core::*;
use rt_2::material::*;
use rt_2::objects::*;
use rt_2::pixels::*;
use rt_2::pixels::texture::TexturedMaterial;
use rt_2::scene::*;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Scene number to render (1-6)
    #[arg(short = 's', long = "scene", default_value_t = 3)]
    scene: u32,

    /// Output filename
    #[arg(short = 'o', long = "output", default_value = "output.ppm")]
    output: String,

    /// Resolution width and height
    #[arg(short = 'r', long = "resolution", value_names = &["WIDTH", "HEIGHT"])]
    resolution: Option<Vec<u32>>,

    /// Number of samples per pixel (higher = less noise, slower)
    #[arg(short = 'p', long = "samples", default_value_t = 16)]
    samples: u32,

    /// Maximum ray depth (higher = more reflections/refractions, slower)
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
        5 => scene_materials(&mut scene), // Basic materials scene
        6 => scene_six(&mut scene), // Complex materials scene
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

    scene.set_samples_per_pixel(args.samples);
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
        (400, 300),
    );

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
        (400, 300),
    );

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(5.0, 0.0, 5.0),
        Texture::Checkerboard(Color::LIGHT_GRAY, Color::GRAY, 5.0),
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
        (400, 300),
    );

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
        128,
        1.0,
        50.0,
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

fn scene_materials(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(0.0, 2.0, 5.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::Y,
        60.0,
        1.0,
        (800, 600),
    );

    scene.set_background(Texture::Gradient(Color::new(0.3, 0.1, 0.5), Color::new(0.1, 0.2, 0.8), 1.0));

    let _lambertian_blue = Lambertian::new(Color::BLUE);

    let lambertian_red_scene = Lambertian::new(Color::RED);
    let metal_gold_scene = Metal::new(Color::new(1.0, 0.8, 0.2), 0.1);
    let glass_scene = Dielectric::new(1.5);


    scene.add_object(Sphere::new_with_material(
        Point3::new(0.0, -0.5, -2.0),
        0.5,
        Texture::SolidColor(Color::RED),
        MaterialType::Lambertian(lambertian_red_scene.clone()),
    ));

    scene.add_object(Sphere::new_with_material(
        Point3::new(0.0, -0.5, 0.0),
        0.5,
        Texture::SolidColor(Color::new(1.0, 0.8, 0.2)),
        MaterialType::Metal(metal_gold_scene.clone()),
    ));

    scene.add_object(Sphere::new_with_material(
        Point3::new(2.0, -0.5, 0.0),
        0.5,
        Texture::SolidColor(Color::new(0.8, 0.9, 1.0)),
        MaterialType::Dielectric(glass_scene.clone()),
    ));

    let textured_orange_material = TexturedMaterial::opaque(
        Texture::Gradient(Color::new(1.0, 0.5, 0.0), Color::new(1.0, 0.8, 0.2), 0.5),
    );
    scene.add_object(Sphere::new_with_textured_material(
        Point3::new(-4.0, -0.5, 0.0),
        0.5,
        textured_orange_material,
        MaterialType::Metal(metal_gold_scene),
    ));

    let textured_blue_material = TexturedMaterial::transparent(
        Texture::SolidColor(Color::BLUE),
        0.8, // 80% transparency
    );
    scene.add_object(Sphere::new_with_textured_material(
        Point3::new(4.0, -0.5, 0.0),
        0.5,
        textured_blue_material,
        MaterialType::Dielectric(glass_scene.clone()),
    ));

    let textured_green_material = TexturedMaterial::transparent(
        Texture::SolidColor(Color::GREEN),
        0.2, // 20% transparency
    );
    scene.add_object(Sphere::new_with_textured_material(
        Point3::new(-2.0, -0.5, 0.0),
        0.5,
        textured_green_material,
        MaterialType::Dielectric(glass_scene),
    ));

    scene.add_object(Plane::new(
        Point3::new(0.0, -1.0, 0.0),
        Vec3::new(10.0, 0.0, 10.0),
        Texture::Checkerboard(Color::GRAY, Color::LIGHT_GRAY, 2.0),
    ));

    scene.add_light(Light::new_point(
        Point3::new(0.0, 3.0, 2.0),
        Color::WHITE,
        1.0,
        32,
        1.0,
        50.0,
    ));

    scene.add_light(Light::new_directional(
        Point3::new(-2.0, -1.0, -1.0),
        Color::new(0.3, 0.3, 0.5),
        0.3,
    ));

}

fn scene_six(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(-2.0, 1.5, 3.0),
        Point3::new(0.0, 1.0, 0.0),
        Vec3::Y,
        75.0,
        1.0,
        (400, 300),
    );

    scene.set_background(Texture::SolidColor(Color::new(0.02, 0.02, 0.02)));

    let lambertian_green = Lambertian::new(Color::GREEN);
    let metal_reflective = Metal::new(Color::WHITE, 0.0);
    let dielectric_purple = Dielectric::new(1.5);
    let dielectric_cylinder = Dielectric::new(1.5);

    let image_texture = match Image::load("assets/test.png") {
        Ok(img) => Texture::Image(img),
        Err(_) => Texture::SolidColor(Color::new(0.8, 0.6, 0.4)),
    };

    let purple_translucent_material = TexturedMaterial::transparent(
        Texture::SolidColor(Color::new(0.6, 0.2, 0.8)),
        0.60,
    );

    let reflective_image_material = TexturedMaterial::opaque(image_texture);

    let cylinder_gradient_material = TexturedMaterial::transparent(
        Texture::Gradient(
            Color::new(0.2, 0.4, 0.8),
            Color::new(0.8, 0.2, 0.4),
            1.0
        ),
        0.50,
    );

    scene.add_object(Sphere::new_with_textured_material(
        Point3::new(0.0, 0.6, 0.0),
        0.6,
        purple_translucent_material,
        MaterialType::Dielectric(dielectric_purple.clone()),
    ));

    scene.add_object(Sphere::new_with_textured_material(
        Point3::new(2.5, 0.7, 1.0),
        0.7,
        reflective_image_material,
        MaterialType::Metal(metal_reflective.clone()),
    ));

    let mut cylinder = Cylinder::new(
        Point3::new(-2.0, 0.6, 0.0),
        0.4,
        1.2,
        Texture::Gradient(
            Color::new(0.2, 0.4, 0.8),
            Color::new(0.8, 0.2, 0.4),
            1.0
        ),
    );
    cylinder.set_material(MaterialType::Dielectric(dielectric_cylinder.clone()));
    cylinder.set_textured_material(cylinder_gradient_material);
    scene.add_object(cylinder);

    let mut small_cube = Cube::new(
        Point3::new(1.0, 0.15, -1.5),
        0.3,
        Texture::SolidColor(Color::new(0.9, 0.9, 0.9)),
    );
    small_cube.set_material(MaterialType::Metal(metal_reflective.clone()));
    scene.add_object(small_cube);

    let mut green_cube = Cube::new(
        Point3::new(-1.5, 0.25, 2.0),
        0.5,
        Texture::SolidColor(Color::GREEN),
    );
    green_cube.set_material(MaterialType::Lambertian(lambertian_green));
    scene.add_object(green_cube);

    scene.add_object(Plane::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(15.0, 0.0, 15.0),
        Texture::Checkerboard(Color::new(0.3, 0.3, 0.3), Color::new(0.7, 0.7, 0.7), 1.5),
    ));

    scene.add_light(Light::new_point(
        Point3::new(3.0, 4.0, 2.0),
        Color::new(1.0, 0.9, 0.7),
        1.2,
        6,
        0.8,
        0.6,
    ));

    scene.add_light(Light::new_point(
        Point3::new(-3.0, 3.5, 1.5),
        Color::new(0.8, 0.9, 1.0),
        0.8,
        6,
        0.6,
        0.7,
    ));

    scene.add_light(Light::new_point(
        Point3::new(0.0, 2.0, -4.0),
        Color::new(0.9, 0.7, 1.0),
        0.6,
        6,
        0.4,
        0.8,
    ));
}
