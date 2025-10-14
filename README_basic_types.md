# Basic types
Here are the basic types that you will be working with to setup the scene:
- [Vec3, Point3](#vec3-point3)
- [Color](#color)
- [Image](#image)
- [Texture](#texture)

## **Vec3, Point3**
`Point3` is just an alias for `Vec3`.
The only difference is the meaning we associate we each of them.
A `Point3` is a point in the 3D space and a `Vec3` is to describe a direction in 3D space.

We have predefine a few values:
```rust
    Vec3::ONE   // 1, 1, 1
    Vec3::ZERO  // 0, 0, 0
    Vec3::X     // 1, 0, 0
    Vec3::Y     // 0, 1, 0
    Vec3::Z     // 0, 0, 1
```

You define your own `Vec3` or `Point3` with:
```rust
    Vec3::new(x: f32, y: f32, z: f32);
    Vec3::splat(val: f32); // val is applied to x, y ,z
```

## **Texture**
The different texture types:
```rust
    Texture::SolidColor(Color);
    Texture::Gradient(Color, Color, f32); // Color_1, Color_2, angle_radian
    Texture::Checkerboard(Color, Color, 1.f32); // Color_1, Color_2, scale
    Texture::Image(Arc<Image>);
```
Do note that the textures are projected flatly onto an object.

## **Color**
There is predefined list of colors:
- greyscale: `WHITE`, `LIGHT_GRAY`, `GRAY`, `DARK_GRAY`, `BLACK`
- neutral: `IVORY`, `BEIGE`, `TAUPE`, `CHARCOAL`, `SLATE`
- basic: `RED`, `GREEN`, `BLUE`, `YELLOW`, `CYAN`, `MAGENTA`, `ORANGE`
- light: `LIGHT_<basic>`
- dark: `DARK_<basic>`
- pastel: `PASTEL_<basic>`
- neon: `NEON_<basic>`

Or you can use your own color with `Color::new(R: u8, G: u8, B: u8)`.

## **Image**
Loading an image (.jpeg, .png, .tiff, .bmp):
```rust
    let image = Image::load(path: &str)?;
```
