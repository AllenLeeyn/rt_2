use super::*;
use rt_2::material::dielectric::Dielectric;

pub fn scene_one(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(0.0, 0.0, 6.0),
        Vec3::ZERO,
        Vec3::Y,
        50.0,
        1.0,
        (800, 600),
    );

    scene.add_object(Sphere::new(
        Point3::ZERO,
        2.0,
        Arc::new(Lambertian::new(Texture::SolidColor(
            Color::FLAT_GREEN * 255.0,
        ))),
    ));

    let light_material = Arc::new(DiffuseLight::new(Texture::SolidColor(Color::LIGHT_YELLOW * 100.0)));
    scene.add_object(Sphere::new(
        Point3::new(-2.0, 4.0, 3.0),
        0.2,
        light_material.clone(),
    ));
}

pub fn scene_two(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(0.0, 2.0, 5.0),
        Point3::new(0.0, 2.0, 0.0),
        Vec3::Y,
        60.0,
        1.0,
        (400, 300),
    );

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(5.0, 0.0, 5.0),
        Arc::new(Lambertian::new(Texture::Checkerboard(
            Color::DARK_GRAY * 255.0,
            Color::GRAY * 255.0,
            5.0,
        ))),
    ));

    let image = Image::load("assets/test.png").unwrap();
    scene.add_object(Cube::new(
        Point3::new(0.0, 0.5, 0.0),
        1.0,
        Arc::new(Lambertian::new(Texture::Image(image))),
    ));

    /* scene.add_light(Light::new_directional(
        Point3::new(-1.0, -4.0, -4.0),
        Color::WHITE,
        0.2,
    )); */
}

pub fn scene_three(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::splat(3.0),
        Vec3::new(0.4, 0.8, 0.0), //Vec3::ZERO,
        Vec3::Y,
        60.0,
        1.0,
        (400, 300),
    );

    scene.add_object(Cube::new(
        Point3::new(-1.5, 0.5, 0.5), // center (1.0 y lifts it above plane)
        1.0,                         // size (width, height, depth)
        Arc::new(Lambertian::new(Texture::Gradient(
            Color::DARK_RED,
            Color::RED,
            1.571,
        ))),
    ));

    scene.add_object(Cylinder::new(
        Point3::new(2.0, 0.0, -1.0),
        0.25,
        2.0,
        Arc::new(Lambertian::new(Texture::Checkerboard(
            Color::BLUE,
            Color::YELLOW,
            1.0,
        ))),
    ));

    scene.add_object(Sphere::new(
        Point3::new(-0.5, 0.5, 0.0),
        0.5,
        Arc::new(Lambertian::new(Texture::SolidColor(Color::GREEN))),
    ));

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Arc::new(Lambertian::new(Texture::Checkerboard(
            Color::GRAY,
            Color::PASTEL_GRAY,
            20.0,
        ))),
    ));

    scene.add_light(Light::new_point(
        Point3::new(1.0, 3.0, 0.0),
        Color::WHITE,
        1.0,
        16,
        10.0,
        60.0,
    ));
}

pub fn scene_four(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(-2.0, 2.0, -4.0),
        Vec3::new(0.4, 0.75, 0.0),
        Vec3::Y,
        60.0,
        1.0,
        (400, 300),
    );

    scene.add_object(Cube::new(
        Point3::new(-1.5, 0.5, 0.5),
        1.0,
        Arc::new(Lambertian::new(Texture::Gradient(
            Color::DARK_RED,
            Color::RED,
            1.571,
        ))),
    ));

    scene.add_object(Cylinder::new(
        Point3::new(2.0, 0.0, -1.0),
        0.25,
        2.0,
        Arc::new(Lambertian::new(Texture::Checkerboard(
            Color::BLUE,
            Color::YELLOW,
            1.0,
        ))),
    ));

    scene.add_object(Sphere::new(
        Point3::new(-0.5, 0.5, 0.0),
        0.5,
        Arc::new(Lambertian::new(Texture::SolidColor(Color::GREEN))),
    ));

    scene.add_object(Plane::new(
        Point3::ZERO,
        Vec3::new(20.0, 0.0, 20.0),
        Arc::new(Lambertian::new(Texture::Checkerboard(
            Color::GRAY,
            Color::PASTEL_GRAY,
            20.0,
        ))),
    ));

    scene.add_light(Light::new_directional(
        Point3::new(-1.5, -6.0, 3.0),
        Color::WHITE,
        0.25,
    ));

    scene.add_light(Light::new_directional(
        Point3::new(-1.0, -4.0, 3.0),
        Color::WHITE,
        0.5,
    ));
}

pub fn scene_five(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(2.0, 1.0, 3.0),
        Vec3::ZERO,
        Vec3::Y,
        60.0,
        1.0,
        (600, 450),
    );

    let material_ground = Arc::new(Lambertian::new(Texture::SolidColor(Color::new(
        90.0, 130.0, 0.0,
    ))));
    let matte_blue = Arc::new(Lambertian::new(Texture::SolidColor(Color::new(
        26.0, 50.0, 120.0,
    ))));
    let smooth_metal = Arc::new(Metal::new(
        Texture::SolidColor(Color::new(160.0, 160.0, 200.0)),
        0.2,
    ));
    let rough_metal = Arc::new(Metal::new(
        Texture::SolidColor(Color::new(220.0, 160.0, 50.0)),
        0.8,
    ));

    scene.add_object(Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        Vec3::new(20.0, 0.0, 20.0),
        material_ground.clone(),
    ));
    scene.add_object(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        matte_blue.clone(),
    ));
    scene.add_object(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        smooth_metal.clone(),
    ));
    scene.add_object(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        rough_metal.clone(),
    ));

    let material_cube = Arc::new(Lambertian::new(Texture::SolidColor(Color::GRAY * 254.0)));
    scene.add_object(Cube::new(
        Point3::new(-1.5, 0.0, 0.5),
        1.0,
        material_cube.clone(),
    ));

    let material_cylinder = Arc::new(Metal::new(
        Texture::SolidColor(Color::new(50.0, 50.0, 50.0)),
        0.0,
    ));
    scene.add_object(Cylinder::new(
        Point3::new(2.0, -0.5, -1.0),
        0.25,
        2.0,
        material_cylinder.clone(),
    ));

    let light_material = Arc::new(DiffuseLight::new(Texture::SolidColor(Color::WHITE * 20.0)));
    scene.add_object(Sphere::new(
        Point3::new(0.0, 1.5, 0.0),
        0.2,
        light_material.clone(),
    ));
}

pub fn scene_six(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(3.0, 0.5, 2.5),
        Vec3::ZERO,
        Vec3::Y,
        40.0,
        1.0,
        (400, 300),
    );

    let blue_gray = Arc::new(Lambertian::new(Texture::SolidColor(Color::new(
        80.0, 85.0, 90.0,
    ))));
    let dark_mirror = Arc::new(Metal::new(
        Texture::SolidColor(Color::new(165.0, 150.0, 130.0)),
        0.0,
    ));
    let green_metal = Arc::new(Metal::new(
        Texture::SolidColor(Color::new(115.0, 130.0, 25.0)),
        0.3,
    ));

    scene.add_object(Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        Vec3::new(20.0, 0.0, 20.0),
        blue_gray.clone(),
    ));
    scene.add_object(Sphere::new(
        Point3::new(-1.0, 0.0, 0.0),
        0.5,
        dark_mirror.clone(),
    ));
    scene.add_object(Sphere::new(
        Point3::new(1.0, 0.0, 0.0),
        0.5,
        green_metal.clone(),
    ));
}

pub fn scene_seven(scene: &mut Scene) {
    scene.camera_mut().set(
        Point3::new(1.0, 1.5, 3.0),
        Point3::new(0.0, 0.5, 0.0),
        Vec3::Y,
        40.0,
        1.0,
        (400, 300),
    );

    let material_ground = Arc::new(Lambertian::new(Texture::SolidColor(Color::new(
        100.0, 170.0, 100.0,
    ))));
    let material_center = Arc::new(Dielectric::new(1.5, Color::WHITE));
    let material_left = Arc::new(Metal::new(
        Texture::SolidColor(Color::new(170.0, 170.0, 205.0)),
        0.0,
    ));
    let material_right = Arc::new(Metal::new(
        Texture::SolidColor(Color::new(250.0, 200.0, 100.0)),
        1.0,
    ));

    scene.add_object(Plane::new(
        Point3::new(0.0, -0.5, 0.0),
        Vec3::new(20.0, 0.0, 20.0),
        material_ground.clone(),
    ));

    scene.add_object(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    ));
    scene.add_object(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    ));
    scene.add_object(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    ));
}
