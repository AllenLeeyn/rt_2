# rt

rt is a simple ray tracing project designed to build foundational understanding of rendering 3D scenes through ray tracing techniques. Below is the default scene rendered.

![Rendered output](demo1.png)

In this project, we implement:
- simple objects:
    - 2D plane
    - Cube
    - Cylinder
    - Sphere
- camera
- lights:
    - directional
    - point
- material:
    - lambertian
    - metal
    - dielectric
- Texture:
    - Solid Color
    - Checked Box
    - Gradient
    - Image

We explore the following concepts:
- The mathematical foundations of ray tracing
- How geometric shapes are defined and rendered in 3D space
- The role of light, material, and texture in producing realistic images

## Installation
This project is written entirely in Rust. To get started:
- Make sure Rust is installed.
- Clone the repository

## Usage
Running the project with:
`cargo run`
will render a default scene and save the output to a file named `output.ppm` in the current directory.

To render your own scene, you will have to define your scene in `main.rs` `fn main()`.
```rust
fn main() -> std::io::Result<()> {
    let mut scene = Scene::new();

    default_scene(&mut scene); // replace this with your scene setup
    scene.render("output.ppm")?; 

    Ok(())
}
```

Read about the [**basic types**](README_basic_types.md) that you will be working with.

Read about the [**scene elements**](README_scene_elements.md) and how to set them up.
