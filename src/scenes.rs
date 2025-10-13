
use rt_2::objects::*;
use rt_2::particle_system::*;
use rt_2::pixels::*;
use rt_2::random_float;
use rt_2::scene::*;
use rt_2::core::*;
use rt_2::material::*;

pub fn scene_one(scene: &mut Scene) {
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

pub fn scene_two(scene: &mut Scene) {

    scene.set_background(Texture::Gradient(Color::BLACK, Color::DARK_PURPLE * 0.3, 90.0));

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

pub fn scene_three(scene: &mut Scene) {
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

pub fn scene_four(scene: &mut Scene) {
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

pub fn scene_five(scene: &mut Scene) {
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

pub fn scene_six(scene: &mut Scene) {
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